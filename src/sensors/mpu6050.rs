// // Reference: https://github.com/juliangaal/mpu6050
// // https://github.com/esp-rs/esp-idf-hal/blob/master/examples/i2c_ssd1306.rs

// use esp_idf_svc::hal::peripherals::Peripherals;
// use esp_idf_svc::hal::delay::{FreeRtos, Delay};
// use esp_idf_svc::hal::i2c::*;
// use esp_idf_svc::hal::prelude::*;
// use mpu6050::*;

// fn main() -> Result<(), ()> {
//     esp_idf_svc::sys::link_patches();
//     esp_idf_svc::log::EspLogger::initialize_default();

//     log::info!("MPU6050");

//     let peripherals = Peripherals::take().expect("Unable to access device peripheral");
//     let i2c = peripherals.i2c0;
//     let sda = peripherals.pins.gpio6;
//     let scl = peripherals.pins.gpio7;

//     let config = I2cConfig::new().baudrate(100.kHz().into());
//     let i2c = I2cDriver::new(i2c, sda, scl, &config).expect("I2c Driver Error");

//     let mut delay = Delay::new(10);
//     let mut mpu = Mpu6050::new(i2c);
  
//     mpu.init(&mut delay).expect("MPU Initialization Error");

//     loop {
//         // we are sleeping here to make sure the watchdog isn't triggered
//         FreeRtos::delay_ms(500);

//         // get roll and pitch estimate
//         let acc = mpu.get_acc_angles().expect("Get Angle Error");
//         println!("r/p: {:?}", acc);

//         // get temp
//         let temp = mpu.get_temp().expect("Get Temperature Error");
//         println!("temp: {:?}c", temp);

//         // get gyro data, scaled with sensitivity 
//         let gyro = mpu.get_gyro().expect("Get gyro Error");
//         println!("gyro: {:?}", gyro);

//         // get accelerometer data, scaled with sensitivity
//         let acc = mpu.get_acc().expect("Get Accelerometer Error");
//         println!("acc: {:?}", acc);
//     }

// }