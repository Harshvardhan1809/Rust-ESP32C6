// Reference: https://blog.theembeddedrustacean.com/esp32-embedded-rust-at-the-hal-uart-serial-communication

use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::delay::{FreeRtos, Delay, BLOCK};
use esp_idf_svc::hal::uart::*;
use esp_idf_svc::hal::gpio;
use esp_idf_svc::hal::prelude::*;
use nb::block;
use nmea::{Nmea, SentenceType};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("GPS Sensor");

    // Step 1 : Set up an UART Driver
    let peripherals = Peripherals::take().expect("Error finding peripherals");
    let tx = peripherals.pins.gpio5;
    let rx = peripherals.pins.gpio6;

    let config = config::Config::new().baudrate(Hertz(115_200));
    let uart = UartDriver::new(peripherals.uart1, tx, rx, Option::<gpio::Gpio0>::None, Option::<gpio::Gpio1>::None, &config).unwrap();

    // let mut nmea_parser = Nmea::default();

    loop {
        println!("Hello World");
        FreeRtos::delay_ms(1000);

        let mut rec = Vec::new();
        let mut buf = [0_u8; 1];
        uart.read(&mut buf, BLOCK).unwrap();
        rec.extend_from_slice(&buf);

        println!("Recieved Garbled Message Values: {:?}", rec);
        // let nmea_sentence = rec.into_iter().collect();

        // // Parse the received NMEA sentence
        // match nmea_parser.parse(nmea_sentence) {
        //     Ok(sentence) => {
        //         if let Some(fix) = nmea_parser.fix() {
        //             println!(
        //                 "Latitude: {:?}, Longitude: {:?}, Altitude: {:?}",
        //                 fix.latitude, fix.longitude, fix.altitude
        //             );
        //         } else {
        //             println!("No fix data available.");
        //         }
        //     }
        //     Err(err) => {
        //         eprintln!("Failed to parse NMEA sentence: {}", err);
        //     }
        // }

    }

}