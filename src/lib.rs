#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#[warn(unused_imports)]

// use esp_idf_svc::hal::peripherals::Peripherals;
// use esp_idf_svc::hal::gpio::PinDriver; 
// use esp_idf_svc::hal::gpio;
// use esp_idf_svc::hal::delay::{FreeRtos, Delay, Ets};
// use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
// use esp_idf_svc::hal::uart::*;
// use esp_idf_svc::hal::uart::{config as UARTConfig};
// // use esp_idf_svc::hal::spi::*;
// // use esp_idf_svc::hal::spi::{config as SPIConfig};
// use esp_idf_svc::hal::task::*;
// use esp_idf_svc::hal::prelude::*;
// use esp_idf_svc::sys::EspError;

// use one_wire_bus::{self, OneWire, OneWireResult};
// use mpu6050::{Mpu6050};
// use ds18b20::{Ds18b20, self, Resolution};
// use ina219::{address::Address, SyncIna219};
// use embedded_sdmmc::{VolumeManager, FatVolume};

// use crate::sensors::ina219::INA219;

mod sensors;
use crate::sensors::ina219::ina219::INA219;

pub fn run(){
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting Cubesat Operations");

    let ina219 = INA219::new(); 
    ina219.read();

}

// pub fn ran(){
//     esp_idf_svc::sys::link_patches();
//     esp_idf_svc::log::EspLogger::initialize_default();

//     log::info!("Starting Cubesat Operations");

//     // initialize peripherals
//     let peripherals = Peripherals::take().expect("Unable to access device peripheral");
//     // let spi = peripherals.spi2;

//     // initialize pins
//     let mut i2c = peripherals.i2c0;
//     let sda = peripherals.pins.gpio6;
//     let sda_mpu = peripherals.pins.gpio6;
//     let scl = peripherals.pins.gpio7;
//     let scl_mpu = peripherals.pins.gpio7;
//     let rx = peripherals.pins.gpio17;
//     let tx = peripherals.pins.gpio16;
//     // let mosi = peripherals.pins.gpio6;
//     // let miso = peripherals.pins.gpio5;
//     // let cs = peripherals.pins.gpio10;

//     // initialize drivers
//     let pin_driver = PinDriver::input_output_od(peripherals.pins.gpio10).unwrap();
//     let mpu_config = I2cConfig::new().baudrate(100.kHz().into());
//     let ina_config = I2cConfig::new().baudrate(100.kHz().into());
//     let i2c_driver_ina= I2cDriver::new(&mut i2c, sda, scl, &ina_config).expect("I2c Driver Error");
//     let i2c_driver_mpu= I2cDriver::new(&mut i2c, sda_mpu, scl_mpu, &mpu_config).expect("I2c Driver Error");

//     // initialize buses
//     let mut one_wire_bus = OneWire::new(pin_driver).unwrap();

//     // initialize drivers
//     let uart_config = UARTConfig::Config::new().baudrate(Hertz(9600));
//     let uart = AsyncUartDriver::new(peripherals.uart1, tx, rx, Option::<gpio::Gpio0>::None, Option::<gpio::Gpio1>::None, &uart_config).unwrap();
//     // let spi_config = SPIConfig::new();
//     // let spi_driver: Result<SpiDriver, EspError> = SpiDriver::new::<SPI2>(spi, scl, mosi, Some(miso), &SpiDriverConfig::new());
//     // let spi_device = SpiDeviceDriver::new(&spi_driver, Some(cs), &SPIConfig::Config::new()).unwrap();

//     // initialize devices
//     let mut ina = SyncIna219::new(i2c_driver_ina, Address::from_byte(0x40).expect("Out of bounds")).expect("INA219 Not detected");  
//     let mut delay = Delay::new(10);
//     let mut mpu = Mpu6050::new(i2c_driver_mpu);
//     mpu.init(&mut delay).expect("MPU Initialization Error");

//     // SD Card
//     // let sd_card = SdMmcSpi::new(spi_device);
//     // let mut volume_mgr = VolumeManager::<_, FatVolume>::new(sd_card, 1);
//     // let volume = volume_mgr.get_volume(0).unwrap();
//     // let root_dir = volume.open_root_dir().unwrap();
//     // let mut file = root_dir.open_file_in_dir("test.txt", embedded_sdmmc::Mode::ReadWriteCreate).unwrap();
//     // file.write(b"Hello, ESP32-C6!").unwrap();

//     block_on(async {
//         loop {      

//             println!("Starting measurements");

//             // Read value from temperature sensor
//             ds18b20::start_simultaneous_temp_measurement(&mut one_wire_bus, &mut Ets).unwrap();
//             Resolution::Bits10.delay_for_measurement_time(&mut FreeRtos);
//             if let Some((device_address, _)) = one_wire_bus.device_search(None, false, &mut Ets).unwrap() {
//                 let sensor = Ds18b20::new::<()>(device_address);
//                 let sensor_data = match sensor.expect("").read_data(&mut one_wire_bus, &mut Ets) {
//                     Ok(sensor_data) =>  { println!("Temperature is {}°C", sensor_data.temperature); }
//                     Err(_) => println!("Error in measuring temperature")
//                 };
                
//             } else {
//                 println!("Temperature sensor not found");
//             }

//             // Read values from current sensor
//             println!("Bus Voltage: {}", ina.bus_voltage().expect("Bus Voltage Error"));
//             println!("Shunt Voltage: {}", ina.shunt_voltage().expect("Shunt Voltage Error"));

//             //:: Read values from gyroscope
//             let ang = mpu.get_acc_angles().expect("Get Angle Error");
//             let temp = mpu.get_temp().expect("Get Temperature Error");    
//             let gyro = mpu.get_gyro().expect("Get gyro Error");
//             let acc = mpu.get_acc().expect("Get Accelerometer Error");
//             println!("ang: {:?}, temp: {:?}, gyro: {:?}, acc: {:?}", ang, temp, gyro, acc);

//             // Read values from GPU
//             let mut rec = Vec::new();
//             for _i in 0..82 {
//                 let mut buf = [0_u8; 1];
//                 uart.read(&mut buf).await.unwrap();
//                 rec.extend_from_slice(&buf);
//             }
//             let nmea_sentence = String::from_utf8(rec).unwrap();
//             println!("Received NMEA Sentence {:?}", nmea_sentence);

//             // Write into SD Card
//             println!("Write to SD Card ...");

//             // 1 second delay
//             FreeRtos::delay_ms(1000);

//         }
//     })

// }