use anyhow::{Context, Result};
use chrono::Duration;
use log::{debug, info};
use rppal::gpio::{Gpio, OutputPin};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration as StdDuration;
use timer::Timer;

const NUM_TO_VALUE: [(u8, u8); 10] = [
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
];

fn main() -> Result<()> {
    env_logger::init();

    info!("Program is starting ...");

    let mut stop_watch = StopWatch::new()?;
    let time = Arc::new(AtomicUsize::new(0));

    let timer = Timer::new();
    let _guard = {
        let time = time.clone();
        timer.schedule_repeating(Duration::seconds(1), move || {
            info!("counter : {}", time.load(Ordering::Relaxed));
            time.fetch_add(1, Ordering::Relaxed);
        })
    };

    loop {
        stop_watch.display(time.load(Ordering::Relaxed));
    }
}

struct StopWatch {
    sn74hc595: SN74HC595,
    digits: [OutputPin; 4],
}

impl StopWatch {
    fn new() -> Result<Self> {
        let gpio = Gpio::new()?;
        Ok(Self {
            sn74hc595: SN74HC595::new(24, 23, 18)?,
            digits: [
                gpio.get(10)?.into_output_high(),
                gpio.get(22)?.into_output_high(),
                gpio.get(27)?.into_output_high(),
                gpio.get(17)?.into_output_high(),
            ],
        })
    }

    fn enable(&mut self, digit: usize) {
        if digit > self.digits.len() - 1 {
            return;
        }

        for i in 0..self.digits.len() {
            if i == digit {
                self.digits[i].set_low();
            } else {
                self.digits[i].set_high();
            }
        }
    }

    fn display(&mut self, time: usize) {
        let mut time = time;
        for i in 0..self.digits.len() {
            self.enable(i);
            self.sn74hc595.write(NUM_TO_VALUE[time % 10].1);
            self.sn74hc595.update();
            thread::sleep(StdDuration::from_micros(1));
            self.sn74hc595.write(0xff);
            self.sn74hc595.update();
            time = time / 10;
        }
    }
}

struct SN74HC595 {
    data: OutputPin,
    latch: OutputPin,
    clock: OutputPin,
    duration: StdDuration,
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
            duration: StdDuration::from_micros(1),
        })
    }

    fn write(&mut self, value: u8) {
        debug!("data output pins: ");
        for i in (0..8).rev() {
            self.clock.set_low();
            if value & (1 << i) != 0 {
                debug!("1");
                self.data.set_high();
            } else {
                debug!("0");
                self.data.set_low();
            }
            thread::sleep(self.duration);
            self.clock.set_high();
            thread::sleep(self.duration);
        }
        debug!("");
    }

    fn update(&mut self) {
        self.latch.set_low();
        thread::sleep(self.duration);
        self.latch.set_high();
        thread::sleep(self.duration);
    }
}
