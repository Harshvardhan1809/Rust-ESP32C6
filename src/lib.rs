use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::gpio::PinDriver; 
use esp_idf_svc::hal::gpio;
use esp_idf_svc::hal::delay::{FreeRtos, Delay, Ets};
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver, I2c};
use esp_idf_svc::hal::uart::*;
use esp_idf_svc::hal::uart::{config as UARTConfig};
use esp_idf_svc::hal::spi::*;
use esp_idf_svc::hal::spi::{config::Config as SPIConfig, SpiDriver, config::MODE_3, SPI2};
use esp_idf_svc::hal::task::*;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::sys::EspError;
use esp_idf_svc::io::asynch::Read;
use shared_bus;
// use embedded_hal::spi::{SpiBus, SpiDevice};
use core::cell::RefCell;

use one_wire_bus::{self, OneWire, OneWireResult};
use mpu6050::{Mpu6050};
use ds18b20::{Ds18b20, self, Resolution};
use ina219_rs::ina219::{INA219,Opts};
use ina219_rs::physic;
use nmea_parser::*;
use embedded_sdmmc::{VolumeManager, FatVolume, SdCard};

pub fn run(){
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting cubesat operations");

    let peripherals = Peripherals::take().expect("Unable to access device peripheral");
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio6;
    let scl = peripherals.pins.gpio7;
    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c_driver = I2cDriver::new(i2c, sda, scl, &config).expect("I2c Driver Error");
    let i2c_bus = shared_bus::BusManagerSimple::new(i2c_driver);

    let pin_driver = PinDriver::input_output_od(peripherals.pins.gpio10).unwrap();
    let mut one_wire_bus = OneWire::new(pin_driver).unwrap();

    let rx = peripherals.pins.gpio17;
    let tx = peripherals.pins.gpio16;
    let uart_config = UARTConfig::Config::new().baudrate(Hertz(9600));
    let mut uart = AsyncUartDriver::new(peripherals.uart1, tx, rx, Option::<gpio::Gpio0>::None, Option::<gpio::Gpio1>::None, &uart_config).unwrap();

    let spi = peripherals.spi2;
    let sclk = peripherals.pins.gpio19;
    let serial_in = peripherals.pins.gpio20; // SDI
    let serial_out = peripherals.pins.gpio21; // SDO
    let cs = peripherals.pins.gpio22;

    let driver = SpiDriver::new::<SPI2>(spi,sclk,serial_out,Some(serial_in),&SpiDriverConfig::new()).unwrap();
    let mut delay = Delay::new(10);
    let spi_config = SPIConfig::new().baudrate(26.MHz().into()).data_mode(MODE_3);
    let mut spi_device = SpiDeviceDriver::new(&driver, Some(cs), &spi_config).unwrap();

    let mut buf = [0_u8; 82];
    let mut rec = Vec::new();
    let mut parser = NmeaParser::new();

    block_on(async {
        loop {      

            println!("Starting measurements");

            FreeRtos::delay_ms(500);

            // Read value from temperature sensor
            {
                ds18b20::start_simultaneous_temp_measurement(&mut one_wire_bus, &mut Ets).unwrap();
                Resolution::Bits10.delay_for_measurement_time(&mut FreeRtos);
                if let Some((device_address, _)) = one_wire_bus.device_search(None, false, &mut Ets).unwrap() {
                    let sensor = Ds18b20::new::<()>(device_address);
                    let sensor_data = match sensor.expect("").read_data(&mut one_wire_bus, &mut Ets) {
                        Ok(sensor_data) =>  { println!("Temperature is {}°C", sensor_data.temperature); }
                        Err(_) => println!("Error in measuring temperature")
                    };
                    
                } else {
                    println!("Temperature sensor not found");
                }
            }

            // Read values from current sensor
            {
                let opt = Opts::new(0x40,100 * physic::MilliOhm,1 * physic::Ampere);
                let mut ina = INA219::new(i2c_bus.acquire_i2c(), opt);
                ina.init().unwrap();
                let pm = ina.sense().unwrap();
                println!("{:?}",pm);
            }

            // Read values from gyroscope
            {
                // let i2c_driver = I2cDriver::new(i2c, sda, scl, &config).expect("I2c Driver Error");
                let mut delay = Delay::new(10);
                let mut mpu = Mpu6050::new(i2c_bus.acquire_i2c());
                mpu.init(&mut delay).expect("MPU Initialization Error");
                let ang = mpu.get_acc_angles().expect("Get Angle Error");
                let temp = mpu.get_temp().expect("Get Temperature Error");    
                let gyro = mpu.get_gyro().expect("Get gyro Error");
                let acc = mpu.get_acc().expect("Get Accelerometer Error");
                println!("ang: {:?}, temp: {:?}, gyro: {:?}, acc: {:?}", ang, temp, gyro, acc);
            }

            // Read values from GPS
            {
                while buf[0] != 36 {
                    uart.read_exact(&mut buf).await.unwrap();
                }
                rec.extend_from_slice(&buf);
                // println!("Received NMEA: {:?}", rec);
                
                if let Some(index) = rec.iter().position(|&x| x == 13) {
                    rec.resize(index, 0);
                }

                // println!("Print received sentence {:?}", String::from_utf8(rec.clone()));
                if rec.len() !=0 {
                    // println!("Received NMEA {:?}", rec);
                    let mut nmea_sentence = match String::from_utf8(rec.clone()) {
                        Ok(sentence) => sentence,
                        Err(_) => {
                            println!("Invalid sentence");
                            continue;
                        }
                    };
                    nmea_sentence.pop();
                    nmea_sentence.pop();
                    // println!("Received NMEA Sentence {:?}", nmea_sentence);
                    rec.clear();
                    
                    // Parse the sentence
                    match parser.parse_sentence(&mut nmea_sentence) {
                        Ok(message) => match message {
                            ParsedMessage::Gga(gga) => {
                                println!("GPS Data: ");
                                println!("Source:    {}",     gga.source);
                                println!("Latitude:  {:?}°", gga.latitude);
                                println!("Longitude: {:?}°", gga.longitude);
                                println!("");
                            },
                            ParsedMessage::Rmc(rmc) => {
                                println!("GPS Data: ");
                                println!("Source:  {}",        rmc.source);
                                println!("Speed:   {:?} kts", rmc.sog_knots);
                                println!("Bearing: {:?}°",       rmc.bearing);
                                println!("");
                            },
                            _ => ()
                        },
                        Err(_) => {
                            println!("Getting Error");
                        }
                    };
                }
            }

            {
                // Write into SD Card
                // Not working
                // let sdcard = SdCard::new(&mut spi_device, delay);
                // println!("Card size is {} bytes", sdcard.num_bytes().unwrap());
                // println!("Write to SD Card ...");

                // let mut response = [0u8; 1];
                // spi_device.transfer(&mut response, &[0xFF])?;
                // println!("SPI Response: {:?}", response);

                // // 1 second delay
                // FreeRtos::delay_ms(1000);
            }

        }
    })
}