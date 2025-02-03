// Reference: https://blog.theembeddedrustacean.com/esp32-embedded-rust-at-the-hal-uart-serial-communication
// https://github.com/esp-rs/esp-idf-hal/blob/master/examples/uart_loopback_async.rs
// https://github.com/esp-rs/esp-idf-hal/blob/master/src/uart.rs
// ChatGPT

use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::uart::*;
use esp_idf_svc::hal::gpio;
use esp_idf_svc::hal::task::*;
use esp_idf_svc::hal::prelude::*;
use nmea_parser::*;

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

    let mut buf = [0_u8; 1];
    let mut rec = Vec::new();
    let mut parser = NmeaParser::new();

    block_on(async {
        loop {        
            // All NMEA sentences start with $ mark
            // https://aprs.gids.nl/nmea/
            uart.read(&mut buf).await.unwrap();
            if buf[0] == 36 {
                if rec.len() !=0 {
                    // println!("Received NMEA: {:?}", rec);
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
                                println!("Source:    {}",     gga.source);
                                println!("Latitude:  {:?}°", gga.latitude);
                                println!("Longitude: {:?}°", gga.longitude);
                                println!("");
                            },
                            ParsedMessage::Rmc(rmc) => {
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
                rec.push(36);
            } else {
                rec.extend_from_slice(&buf);
            }
        }
    })
}