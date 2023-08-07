use anyhow::{anyhow, Result};
use bitvec::{field::BitField, prelude::*};
use rppal::gpio::{Gpio, IoPin, Level, Mode};
use std::thread;
use std::time::{Duration, Instant};

pub struct DHT {
    data_pin: IoPin,
    buffer: BitArray<[usize; 2]>, // 40 bits
}

impl DHT {
    pub fn new(data_pin: u8) -> Result<Self> {
        let gpio = Gpio::new()?;
        Ok(Self {
            data_pin: gpio.get(data_pin)?.into_io(Mode::Output),
            buffer: bitarr![0; 40],
        })
    }

    pub fn read(&mut self) -> Result<BitArray<[usize; 2]>> {
        self.data_pin.set_mode(Mode::Output);
        // clear sda
        self.data_pin.set_high();
        thread::sleep(Duration::from_millis(500));
        // start signal
        self.data_pin.set_low();
        thread::sleep(Duration::from_millis(20)); // 18ms
        self.data_pin.set_high();

        self.data_pin.set_mode(Mode::Input);
        self.wait_for(Level::Low, Duration::from_micros(100))?;
        self.wait_end_of(Level::Low, Duration::from_micros(100))?; // 80us
        self.wait_end_of(Level::High, Duration::from_micros(100))?; // 80us

        for i in (0..40).rev() {
            self.wait_end_of(Level::Low, Duration::from_micros(100))?; // 50us
            let time = self.wait_end_of(Level::High, Duration::from_micros(100))?;

            self.buffer.set(i, time > Duration::from_micros(60)); // 26-28us -> 0, 70us -> 1
        }

        self.data_pin.set_mode(Mode::Output);
        self.data_pin.set_high();

        if self.check_parity() {
            Ok(self.buffer)
        } else {
            Err(anyhow!("checksum error"))
        }
    }

    pub fn get_temperature(&self) -> f32 {
        let temperature_decimal = self.buffer[8..16].load::<u8>() as f32;
        let temperature = self.buffer[16..24].load::<u8>() as f32;
        temperature + temperature_decimal * 0.1
    }

    pub fn get_humidity(&self) -> f32 {
        self.buffer[32..40].load::<u8>() as f32
    }

    fn check_parity(&self) -> bool {
        self.buffer[0..8].load::<u8>()
            == (1..=4)
                .map(|i| self.buffer[8 * i..8 * (i + 1)].load::<u8>())
                .sum::<u8>()
    }

    fn wait_for(&self, level: Level, timeout: Duration) -> Result<Duration> {
        let t = Instant::now();
        loop {
            if self.data_pin.read() == level {
                return Ok(t.elapsed());
            }
            if t.elapsed() > timeout {
                return Err(anyhow!("timeout to wait for {level}"));
            }
        }
    }

    fn wait_end_of(&self, level: Level, timeout: Duration) -> Result<Duration> {
        let t = Instant::now();
        while self.data_pin.read() == level {
            if t.elapsed() > timeout {
                return Err(anyhow!("timeout to wait for the end of {level}"));
            }
        }
        Ok(t.elapsed())
    }
}
