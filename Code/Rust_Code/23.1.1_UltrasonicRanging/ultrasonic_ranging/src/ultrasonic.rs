pub use anyhow::{anyhow, Result};
use rppal::gpio::{Gpio, InputPin, Level, OutputPin, Trigger};
use std::thread;
use std::time::{Duration, Instant};

pub struct RangeDetector {
    trig: OutputPin,
    echo: InputPin,
    /// unit: m
    max_range: f32,
    /// unit: m/s
    sound_speed: f32,
}

impl RangeDetector {
    pub fn new(trig: u8, echo: u8, max_range: f32) -> Result<Self> {
        let gpio = Gpio::new()?;
        Ok(Self {
            trig: gpio.get(trig)?.into_output_low(),
            echo: gpio.get(echo)?.into_input(),
            max_range,
            sound_speed: 340.0,
        })
    }

    fn trigger(&mut self) {
        self.trig.set_high();
        thread::sleep(Duration::from_micros(10));
        self.trig.set_low();
    }

    fn measure(&mut self) -> Result<f32> {
        let timeout = Duration::from_secs_f32(self.max_range * 2.0 / self.sound_speed);
        self.echo.set_interrupt(Trigger::Both)?;
        if let Some(Level::High) = self.echo.poll_interrupt(true, Some(timeout))? {
            let t = Instant::now();
            if let Some(Level::Low) = self.echo.poll_interrupt(true, Some(timeout))? {
                let duration = t.elapsed().as_secs_f32();
                return Ok(duration * self.sound_speed / 2.0);
            }
        }

        // timeout
        Err(anyhow!("cannot receive data from echo."))
    }

    pub fn get_distance(&mut self) -> Result<f32> {
        self.trigger();
        self.measure()
    }
}
