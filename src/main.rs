use esp_idf_hal::{
    delay,
    gpio::{self, PinDriver},
    prelude::*,
    spi,
};
use esp_idf_sys as _;
use log::*;

mod nixie;
mod util;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    // Turn the high voltage psu off
    let mut n_psu_enable = PinDriver::output(pins.gpio12)?;
    n_psu_enable.set_high()?;

    info!("High voltage PSU disabled");

    // Enable shift output
    let mut n_output_enable = PinDriver::output(pins.gpio16)?;
    n_output_enable.set_low()?;

    info!("Shift output enable");

    let spi_driver = spi::SpiDeviceDriver::new_single(
        peripherals.spi2,
        pins.gpio5,
        pins.gpio4,
        Option::<gpio::AnyIOPin>::None,
        Some(pins.gpio17),
        &spi::config::DriverConfig::default(),
        &spi::SpiConfig::new().baudrate(4_u32.MHz().into()),
    )?;

    info!("Starting nixie loop");
    n_psu_enable.set_low()?;
    let mut nixie_clock = nixie::driver::NixieClock::new(spi_driver);
    let _nixie_thread = std::thread::Builder::new()
        .stack_size(2048)
        .spawn(move || nixie_clock.run())?;

    loop {
        delay::Delay::delay_ms(100);
    }
}
