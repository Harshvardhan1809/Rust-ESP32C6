// // Reference: https://forum.arduino.cc/t/problem-esp32-ina219-not-connected/1190686
// // https://github.com/scttnlsn/ina219/blob/master/examples/minimal.rs

// use esp_idf_svc::hal::prelude::*;

// use ina219::address::Address;
// use ina219::SyncIna219;
// use std::error::Error;

pub mod ina219 {

    use esp_idf_svc::hal::peripherals::Peripherals;
    // use esp_idf_svc::hal::delay::FreeRtos;
    use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
    use ina219::calibration::UnCalibrated;
    use ina219::{SyncIna219, address::Address};
    use esp_idf_svc::hal::prelude::*;

    // pub struct Components<'a> {
    //     i2c: &'a mut I2C0, //Peripherals::I2C0,
    //     sda: &'a mut Gpio6, //Peripherals::pins::GPIO6,
    //     scl: &'a mut Gpio7, //Peripherals::pins::GPIO7,
    //     driver: I2cDriver<'a>, //i2c::I2cDriver,
    // }

    // impl<'a> Components<'a> {
    //     fn new() -> Self {
    //         Self { i2c: &mut i2c, sda: &mut sda, scl: &mut scl, driver }            
    //     }
    // }
    
    // pub struct INA219<'a> {
    //     // peripherals: Components<'a>,
    //     device: SyncIna219<I2cDriver<'a>, Address>,
    // }

    pub type INA219 = SyncIna219<I2cDriver<'static>, UnCalibrated>;

    trait SensorFunctions {
        fn new() -> Self;
        fn read(&self);
    }

    impl SensorFunctions for INA219 {
        fn new() -> Self {
            let peripherals = Peripherals::take().expect("Unable to access device peripheral");
            let i2c = peripherals.i2c0;
            let sda = peripherals.pins.gpio6;
            let scl = peripherals.pins.gpio7;
            let config = I2cConfig::new().baudrate(100.kHz().into());
            let driver = I2cDriver::new(i2c, sda, scl, &config).expect("I2c Driver Error");

            SyncIna219::new(driver, Address::from_byte(0x40).expect("Out of bounds")).expect("INA219 Not detected")
        }

        fn read(&self) {
            println!("ds");
            // println!("Bus Voltage: {}", self.bus_voltage().expect("Bus Voltage Error"));
            // println!("Shunt Voltage: {}", self.shunt_voltage().expect("Shunt Voltage Error"));
        }
    }
    
    // impl<'a> INA219<'a> {
    //     pub fn read(&self){
    //         println!("Bus Voltage: {}", &self.device.bus_voltage().expect("Bus Voltage Error"));
    //         println!("Shunt Voltage: {}", &self.device.shunt_voltage().expect("Shunt Voltage Error"));
    //     }
    
    //     pub fn new(&self) -> Self {        
    //         let peripherals = Peripherals::take().expect("Unable to access device peripheral");
    //         let i2c = peripherals.i2c0;
    //         let sda = peripherals.pins.gpio6;
    //         let scl = peripherals.pins.gpio7;
    //         let config = I2cConfig::new().baudrate(100.kHz().into());
    //         let driver = I2cDriver::new(i2c, sda, scl, &config).expect("I2c Driver Error");
            
    //         Self {
    //             // peripherals: Components::new(),
    //             device: SyncIna219::new(driver, Address::from_byte(0x40).expect("Out of bounds")).expect("INA219 Not detected"),
    //         }
    //     }
    // }

}


// fn main() -> Result<(), Box<dyn Error>> {
//     esp_idf_svc::sys::link_patches();
//     esp_idf_svc::log::EspLogger::initialize_default();
    
//     let peripherals = Peripherals::take().expect("Unable to access device peripheral");
//     let i2c = peripherals.i2c0;
//     let sda = peripherals.pins.gpio6;
//     let scl = peripherals.pins.gpio7;

//     let config = I2cConfig::new().baudrate(100.kHz().into());
//     let i2c = I2cDriver::new(i2c, sda, scl, &config).expect("I2c Driver Error");
//     let mut ina = SyncIna219::new(i2c, Address::from_byte(0x40).expect("Out of bounds")).expect("INA219 Not detected");    

//     // std::thread::sleep(ina.configuration()?.conversion_time_us().expect("Error"));
//     //FreeRtos::delay_ms(ina.configuration().expect("Configuration Error").conversion_time_us().expect("Conversion Error") * 1000);

//     loop {
//         FreeRtos::delay_ms(500);
//         println!("Bus Voltage: {}", ina.bus_voltage().expect("Bus Voltage Error"));
//         println!("Shunt Voltage: {}", ina.shunt_voltage().expect("Shunt Voltage Error"));
//     }
// }

