use std::{sync::mpsc::channel, time::Duration};

use embedded_hal::blocking::spi;
use esp_idf_svc::timer::EspTimerService;

pub struct NixieClock<S> {
    spi: S,
}

impl<'a, S> NixieClock<S>
where
    S: spi::Write<u8> + Send + 'a,
{
    pub fn new(spi: S) -> Self {
        Self { spi }
    }

    pub fn run(&mut self) -> Result<(), anyhow::Error> {
        let (tx, rx) = channel();
        let periodic_timer = EspTimerService::new()?.timer(move || {
            tx.send("hello").unwrap();
        })?;

        periodic_timer.every(Duration::from_millis(3))?;

        // hack a duty cycle
        let mut count = 0;

        loop {
            if rx.recv().is_ok() {
                if count >= 3 {
                    self.send();
                    count = 0;
                } else {
                    self.send_off();
                    count += 1;
                }
            } else {
                anyhow::bail!("Channel failed.");
            }
        }
    }

    fn send(&mut self) {
        let _x = self.spi.write(&[0x40, 0x00, 0x00, 0x08]);
    }

    fn send_off(&mut self) {
        let _x = self.spi.write(&[0x00, 0x00, 0x00, 0x00]);
    }
}
