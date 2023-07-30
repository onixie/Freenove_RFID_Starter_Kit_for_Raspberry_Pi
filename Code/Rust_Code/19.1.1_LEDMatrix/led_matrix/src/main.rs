use anyhow::{Context, Result};
use rppal::gpio::{Gpio, OutputPin};
use std::thread;
use std::time::Duration;

const SMILE: [u8; 8] = [0x1c, 0x22, 0x51, 0x45, 0x45, 0x51, 0x22, 0x1c];
const HEART: [u8; 8] = [
    0b00110000,
    0b01111000,
    0b01111100,
    0b00111110,
    0b00111110,
    0b01111100,
    0b01111000,
    0b00110000,
];
const NUM: [u8; 144] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // " "
    0x00, 0x00, 0x3E, 0x41, 0x41, 0x3E, 0x00, 0x00, // "0"
    0x00, 0x00, 0x21, 0x7F, 0x01, 0x00, 0x00, 0x00, // "1"
    0x00, 0x00, 0x23, 0x45, 0x49, 0x31, 0x00, 0x00, // "2"
    0x00, 0x00, 0x22, 0x49, 0x49, 0x36, 0x00, 0x00, // "3"
    0x00, 0x00, 0x0E, 0x32, 0x7F, 0x02, 0x00, 0x00, // "4"
    0x00, 0x00, 0x79, 0x49, 0x49, 0x46, 0x00, 0x00, // "5"
    0x00, 0x00, 0x3E, 0x49, 0x49, 0x26, 0x00, 0x00, // "6"
    0x00, 0x00, 0x60, 0x47, 0x48, 0x70, 0x00, 0x00, // "7"
    0x00, 0x00, 0x36, 0x49, 0x49, 0x36, 0x00, 0x00, // "8"
    0x00, 0x00, 0x32, 0x49, 0x49, 0x3E, 0x00, 0x00, // "9"
    0x00, 0x00, 0x3F, 0x44, 0x44, 0x3F, 0x00, 0x00, // "A"
    0x00, 0x00, 0x7F, 0x49, 0x49, 0x36, 0x00, 0x00, // "B"
    0x00, 0x00, 0x3E, 0x41, 0x41, 0x22, 0x00, 0x00, // "C"
    0x00, 0x00, 0x7F, 0x41, 0x41, 0x3E, 0x00, 0x00, // "D"
    0x00, 0x00, 0x7F, 0x49, 0x49, 0x41, 0x00, 0x00, // "E"
    0x00, 0x00, 0x7F, 0x48, 0x48, 0x40, 0x00, 0x00, // "F"
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // " "
];

fn main() -> Result<()> {
    println!("Hello, world!");

    let mut bar_led = SN74HC595::new(17, 27, 22)?;

    loop {
        for _ in 0..100 { // repeat output to display longer
            for (col, row) in SMILE.iter().enumerate() {
                bar_led.write(*row);
                bar_led.write(!(0x80 >> col));
                bar_led.update();
            }
        }

        for i in 0..(NUM.len() - 8) {
            let screen = &NUM[i..i + 8];
            for _ in 0..10 { // repeat output to display longer
                for (col, row) in screen.iter().enumerate() {
                    bar_led.write(*row);
                    bar_led.write(!(0x80 >> col));
                    bar_led.update();
                }
            }
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
