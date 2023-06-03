use std::{sync::mpsc::channel, time::Duration};

use embedded_hal::blocking::spi;
use esp_idf_svc::timer::EspTimerService;
use log::info;

pub struct NixieClock<S> {
    spi: S,
    buffer: [u8; 4],
    count: u8,
}

impl<'a, S> NixieClock<S>
where
    S: spi::Write<u8> + Send + 'a,
{
    pub fn new(spi: S) -> Self {
        let buffer = [0x00, 0x00, 0x00, 0x00];
        let count = 0;
        Self { spi, buffer, count }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        if self.count >= 3 {
            self.send(&self.buffer.clone())?;
            self.count = 0;
        } else {
            self.send(&[0x00, 0x00, 0x00, 0x00])?;
            self.count += 1;
        }

        Ok(())
    }

    pub fn set(&mut self, mut buffer: &[u8; 4]) {
        self.buffer = *buffer;
    }

    fn send(&mut self, mut buffer: &[u8]) -> anyhow::Result<()> {
        let _x = self.spi.write(buffer);
        Ok(())
    }
}
