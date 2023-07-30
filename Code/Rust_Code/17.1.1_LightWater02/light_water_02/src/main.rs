use anyhow::{Context, Result};
use rppal::gpio::{Gpio, OutputPin};
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    println!("Hello, world!");

    let mut bar_led = SN74HC595::new(17, 27, 22)?;

    loop {
        for i in (0..8).chain((0..8).rev()) {
            bar_led.write(1 << i);
            bar_led.update();
            thread::sleep(Duration::from_millis(100));
        }
    }
}

struct SN74HC595 {
    data: OutputPin,
    latch: OutputPin,
    clock: OutputPin,
    duration: Duration,
}

impl SN74HC595 {
    fn new(data_pin: u8, latch_pin: u8, clock_pin: u8) -> Result<Self> {
        let gpio = Gpio::new()?;
        Ok(Self {
            data: gpio
                .get(data_pin)
                .with_context(|| "failed to connect serial data input.")?
                .into_output_low(),
            latch: gpio
                .get(latch_pin)
                .with_context(|| "failed to connect parallel update input")?
                .into_output_low(),
            clock: gpio
                .get(clock_pin)
                .with_context(|| "failed to connect serial clock input.")?
                .into_output_low(),
            // CHECKME: Probably no time constraint on the duration of a signal.
            duration: Duration::from_micros(1),
        })
    }

    fn write(&mut self, value: u8) {
        print!("data output pins: ");
        for i in 0..8 {
            self.clock.set_low();
            if value & (1 << i) != 0 {
                print!("1");
                self.data.set_high();
            } else {
                print!("0");
                self.data.set_low();
            }
            thread::sleep(self.duration);
            self.clock.set_high();
            thread::sleep(self.duration);
        }
        println!("");
    }

    fn update(&mut self) {
        self.latch.set_low();
        thread::sleep(self.duration);
        self.latch.set_high();
        thread::sleep(self.duration);
    }
}
