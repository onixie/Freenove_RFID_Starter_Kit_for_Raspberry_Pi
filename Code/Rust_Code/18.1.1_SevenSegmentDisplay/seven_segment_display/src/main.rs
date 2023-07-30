use anyhow::{Context, Result};
use rppal::gpio::{Gpio, OutputPin};
use std::thread;
use std::time::Duration;

const NUM_TO_VALUE: &'static [(u8, u8); 16] = &[
    (0, 0b1100_0000),
    (1, 0b1111_1001),
    (2, 0b1010_0100),
    (3, 0b1011_0000),
    (4, 0b1001_1001),
    (5, 0b1001_0010),
    (6, 0b1000_0010),
    (7, 0b1111_1000),
    (8, 0b1000_0000),
    (9, 0b1001_0000),
    (0xA, 0b1000_1000),
    (0xB, 0b1000_0000),
    (0xC, 0b1100_0110),
    (0xD, 0b1100_0000),
    (0xE, 0b1000_0110),
    (0xF, 0b1000_1110),
];

fn main() -> Result<()> {
    println!("Hello, world!");

    let mut bar_led = SN74HC595::new(17, 27, 22)?;

    let mut show_digit = false;
    loop {
        for (num, value) in NUM_TO_VALUE {
            println!("display {num}: value {:#x}", *value);
            if show_digit {
                bar_led.write(*value);
            } else {
                bar_led.write(*value & 0b0111_1111);
            }
            bar_led.update();
            thread::sleep(Duration::from_secs(1));
        }
        show_digit = !show_digit;
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
        for i in (0..8).rev() {
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
