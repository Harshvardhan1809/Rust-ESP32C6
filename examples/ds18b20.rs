// Reference: https://github.com/esp-rs/esp-idf-hal/blob/master/examples/rmt_onewire_temperature.rs

use std::time::Duration;
use esp_idf_svc::hal::onewire::{OWAddress, OWCommand, OWDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::sys::EspError;
use esp_idf_svc::hal::delay::FreeRtos;
use anyhow;

fn main() -> anyhow::Result<()> {

    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Temperature Sensor!");

    let peripherals = Peripherals::take().expect("Unable to access device peripheral");
    let channel = peripherals.rmt.channel0;
    let onewire_gpio_pin = peripherals.pins.gpio10;

    log::info!("GPI011");

    let mut onewire_bus: OWDriver = OWDriver::new(onewire_gpio_pin, channel)?;
    let device = {
        let mut search = onewire_bus.search()?;
        search.next()
    };

    log::info!("Onewire Bus");

    if device.is_none() {
        println!("No device found");
        return Ok(());
    }

    let device = device.unwrap();
    if let Err(err) = device {
        println!("An error occured searching for the device, err = {}", err);
        return Err(err.into());
    }
    let device = device.unwrap();
    println!(
        "Found Device: {:?}, family code = {}",
        device,
        device.family_code()
    );

    loop {
        ds18b20_trigger_temp_conversion(&device, &onewire_bus)?;
        let temp = ds18b20_get_temperature(&device, &onewire_bus)?;
        println!("Temperature: {}", temp);
        FreeRtos::delay_ms(3000);
    }
}

fn ds18b20_send_command<'a>(addr: &OWAddress, bus: &OWDriver, cmd: u8) -> Result<(), EspError> {
    let mut buf = [0; 10];
    buf[0] = OWCommand::MatchRom as _;
    let addr = addr.address().to_le_bytes();
    buf[1..9].copy_from_slice(&addr);
    buf[9] = cmd;

    bus.write(&buf)
}

enum Ds18b20Command {
    ConvertTemp = 0x44,
    WriteScratch = 0x4E,
    ReadScratch = 0xBE,
}

fn ds18b20_trigger_temp_conversion<'a>(addr: &OWAddress, bus: &OWDriver) -> Result<(), EspError> {
    // reset bus and check if the ds18b20 is present
    bus.reset()?;

    ds18b20_send_command(addr, bus, Ds18b20Command::ConvertTemp as u8)?;

    // delay proper time for temp conversion,
    // assume max resolution (12-bits)
    std::thread::sleep(Duration::from_millis(800));

    Ok(())
}

fn ds18b20_get_temperature<'a>(addr: &OWAddress, bus: &OWDriver) -> Result<f32, EspError> {
    bus.reset()?;

    ds18b20_send_command(addr, bus, Ds18b20Command::ReadScratch as u8)?;

    let mut buf = [0u8; 9];
    bus.read(&mut buf)?;
    let lsb = buf[0];
    let msb = buf[1];

    let temp_raw: u16 = (u16::from(msb) << 8) | u16::from(lsb);

    Ok(f32::from(temp_raw) / 16.0)
}