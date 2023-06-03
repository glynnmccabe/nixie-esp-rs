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
        //[0x40, 0x00, 0x00, 0x08]
        let send_buffer = [0x40u8, 0x00u8, 0x00u8, 0x08u8];

        let (tx, rx) = channel();
        let periodic_timer = EspTimerService::new()?.timer(move || {
            tx.send(send_buffer).unwrap();
        })?;

        periodic_timer.every(Duration::from_millis(3))?;

        // hack a duty cycle
        let mut count = 0;

        loop {
            let buffer = rx.recv()?;
            if count >= 3 {
                self.send(&buffer);
                count = 0;
            } else {
                self.send(&[0x00, 0x00, 0x00, 0x00]);
                count += 1;
            }
        }
    }

    fn send(&mut self, buffer: &[u8]) {
        let _x = self.spi.write(buffer);
    }
}
