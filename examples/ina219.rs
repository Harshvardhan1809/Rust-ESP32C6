// Reference: https://forum.arduino.cc/t/problem-esp32-ina219-not-connected/1190686
// https://github.com/scttnlsn/ina219/blob/master/examples/minimal.rs

use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::prelude::*;

use ina219::address::Address;
use ina219::SyncIna219;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    
    let peripherals = Peripherals::take().expect("Unable to access device peripheral");
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio6;
    let scl = peripherals.pins.gpio7;

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config).expect("I2c Driver Error");
    let mut ina = SyncIna219::new(i2c, Address::from_byte(0x40).expect("Out of bounds")).expect("INA219 Not detected");    

    // std::thread::sleep(ina.configuration()?.conversion_time_us().expect("Error"));
    //FreeRtos::delay_ms(ina.configuration().expect("Configuration Error").conversion_time_us().expect("Conversion Error") * 1000);

    loop {
        FreeRtos::delay_ms(500);
        println!("Bus Voltage: {}", ina.bus_voltage().expect("Bus Voltage Error"));
        println!("Shunt Voltage: {}", ina.shunt_voltage().expect("Shunt Voltage Error"));
    }
}