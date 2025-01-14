// Reference: https://blog.theembeddedrustacean.com/esp32-embedded-rust-at-the-hal-uart-serial-communication
// https://github.com/esp-rs/esp-idf-hal/blob/master/examples/uart_loopback_async.rs
// https://github.com/esp-rs/esp-idf-hal/blob/master/src/uart.rs
// ChatGPT

use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::uart::*;
use esp_idf_svc::hal::gpio;
use esp_idf_svc::hal::task::*;
use esp_idf_svc::hal::prelude::*;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("GPS Sensor");

    let peripherals = Peripherals::take().expect("Error finding peripherals");
    let rx = peripherals.pins.gpio17;
    let tx = peripherals.pins.gpio16;
    let config = config::Config::new().baudrate(Hertz(9600)); // .data_bits(8.into()).stop_bits(1.into()).parity_none();
    let uart = AsyncUartDriver::new(peripherals.uart1, tx, rx, Option::<gpio::Gpio0>::None, Option::<gpio::Gpio1>::None, &config).unwrap();

    log::info!("Connected to NEO6M");

    block_on(async {
        loop {        
            FreeRtos::delay_ms(1000);
            let mut rec = Vec::new();

            // NMEA 0183 message is 82 ASCII characters long, 82 bytes
            for _i in 0..82 {
                let mut buf = [0_u8; 1];
                uart.read(&mut buf).await.unwrap();
                rec.extend_from_slice(&buf);
            }

            println!("Received NMEA: {:?}", rec);
            let nmea_sentence = String::from_utf8(rec).unwrap();
            println!("Received NMEA Sentence {:?}", nmea_sentence);

            // TODO: Parse the NMEA to get data
        }
    })
}