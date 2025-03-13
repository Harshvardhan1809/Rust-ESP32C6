// // Reference: https://github.com/esp-rs/esp-idf-hal/blob/master/examples/rmt_onewire_temperature.rs
// // https://github.com/zerom0/fearless-embedded-rust/blob/main/src/main.rs
// // https://crates.io/crates/one-wire-bus
// // https://crates.io/crates/ds18b20

// use esp_idf_svc::hal::peripherals::Peripherals;
// use esp_idf_svc::hal::gpio::PinDriver; 
// use esp_idf_svc::hal::delay::{FreeRtos, Ets};
// use embedded_hal::digital::v2::{InputPin, OutputPin};
// use ds18b20::{Ds18b20, self, Resolution};
// use one_wire_bus::{self, OneWire, OneWireResult};

// fn main() {

//     esp_idf_svc::sys::link_patches();
//     esp_idf_svc::log::EspLogger::initialize_default();
    
//     log::info!("Temperature Sensor!");
//     let peripherals = Peripherals::take().expect("Unable to access device peripheral");
//     let driver = PinDriver::input_output_od(peripherals.pins.gpio10).unwrap();
//     let mut one_wire_bus = OneWire::new(driver).unwrap();
    
//     let _ = get_temperature(&mut one_wire_bus);
// }

// fn get_temperature<P, E>(
//     one_wire_bus: &mut OneWire<P>,
// ) -> OneWireResult<(), E>
//     where
//     P: OutputPin<Error = E> + InputPin<Error = E>,
// {
//     loop {
//         // initiate a temperature measurement for all connected devices
//         ds18b20::start_simultaneous_temp_measurement(one_wire_bus, &mut Ets)?;
//         // wait until the measurement is done. This depends on the resolution you specified
//         // If you don't know the resolution, you can obtain it from reading the sensor data,
//         // or just wait the longest time, which is the 12-bit resolution (750ms)
//         Resolution::Bits10.delay_for_measurement_time(&mut FreeRtos);

//         // iterate over all the devices, and report their temperature
//         if let Some((device_address, _)) = one_wire_bus.device_search(None, false, &mut Ets)? {
//             if device_address.family_code() != ds18b20::FAMILY_CODE {
//                 continue; // skip other devices
//             }
//             // You will generally create the sensor once, and save it for later
//             let sensor = Ds18b20::new(device_address)?;

//             // contains the read temperature, as well as config info such as the resolution used
//             let sensor_data = sensor.read_data(one_wire_bus, &mut Ets)?;
//             println!("Device at {:?} is {}°C", device_address, sensor_data.temperature);
//         } else {
//             break;
//         }
//         FreeRtos::delay_ms(1000);
//     }
//     Ok(())
// }