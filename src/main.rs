use esp_idf_hal::{gpio::PinDriver, prelude::*};
use esp_idf_sys as _;
use log::*;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    // Turn the high voltage psu off
    let mut n_psu_enable = PinDriver::output(pins.gpio12)?;
    n_psu_enable.set_high()?;

    info!("Hello, world!");
    Ok(())
}
