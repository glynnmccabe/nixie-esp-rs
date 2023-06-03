use std::{sync::mpsc::channel, time::Duration};

use embedded_hal::blocking::spi;
use esp_idf_svc::timer::EspTimerService;

pub struct NixieClock<S> {
    spi: S,
    buffer: [u8; 4],
}

impl<'a, S> NixieClock<S>
where
    S: spi::Write<u8> + Send + 'a,
{
    pub fn new(spi: S) -> Self {
        let buffer = [0x00, 0x00, 0x00, 0x00];
        Self { spi, buffer }
    }

    pub fn run(&mut self) -> Result<(), anyhow::Error> {
        let (tx, rx) = channel();
        let periodic_timer = EspTimerService::new()?.timer(move || {
            tx.send(true).unwrap();
        })?;

        periodic_timer.every(Duration::from_millis(3))?;

        // hack a duty cycle
        let mut count = 0;

        loop {
            if rx.recv()? {
                if count >= 3 {
                    self.buffer = [0x40, 0x00, 0x00, 0x08];
                    count = 0;
                } else {
                    self.buffer = [0x00, 0x00, 0x00, 0x00];
                    count += 1;
                }
                self.send();
            }
        }
    }

    fn send(&mut self) {
        let _x = self.spi.write(&self.buffer);
    }
}
