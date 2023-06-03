use std::{sync::mpsc::channel, time::Duration};

use esp_idf_hal::{
    delay,
    gpio::{self, PinDriver},
    prelude::*,
    spi,
};
use esp_idf_svc::timer::EspTimerService;
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

    let (tx, rx) = channel();
    let periodic_timer = EspTimerService::new()?.timer(move || {
        tx.send(true).unwrap();
    })?;

    periodic_timer.every(Duration::from_millis(3))?;
    let mut index = 0;
    let values1 = [0x02, 0x01, 0x00, 0x00, 0x00, 0x40, 0x20, 0x10, 0x08, 0x04];
    let values2 = [0x00, 0x00, 0x04, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut start = time::Instant::now();
    loop {
        let now = time::Instant::now();
        let difference = now - start;
        if difference.whole_seconds() >= 1 {
            start = now;
            nixie_clock.set(&[0x40, 0x00, values2[index], values1[index]]);
            index += 1;
            if (index > 9) {
                index = 0;
            }
        }
        if (rx.recv().is_ok()) {
            nixie_clock.run();
        }
    }
}

fn nixie_update() {}
