use anyhow::Result;
use rppal::gpio::{Gpio, OutputPin};
use std::thread;
use std::time::Duration;

const PINS: &'static [u8] = &[18, 23, 24, 25];
const ROUND_STEPS: usize = 32 * 64;

fn main() -> Result<()> {
    println!("Program is starting ...");

    let mut motor = {
        let gpio = Gpio::new()?;
        let motor_pins: Vec<OutputPin> = PINS
            .into_iter()
            .map(move |pin| {
                gpio.get(*pin)
                    .expect("fail to get Gpio pin {pin}")
                    .into_output_low()
            })
            .collect();
        Stepper::new(motor_pins)
    };

    loop {
        for _ in 0..=ROUND_STEPS {
            motor.step_forward();
            thread::sleep(Duration::from_millis(3));
        }

        thread::sleep(Duration::from_millis(500));

        for _ in 0..=ROUND_STEPS {
            motor.step_back();
            thread::sleep(Duration::from_millis(3));
        }
    }
}

struct Stepper {
    pins: Vec<OutputPin>,
    phase: usize,
    step: usize,
}

impl Stepper {
    fn new(pins: Vec<OutputPin>) -> Self {
        let phase = pins.len();
        let step = 0;
        let mut stepper = Self { pins, phase, step };
        stepper.update_pin_states();
        stepper
    }

    fn update_pin_states(self: &mut Self) {
        for pin_index in 0..self.phase {
            if pin_index == self.step % self.phase {
                self.pins[pin_index].set_high();
            } else {
                self.pins[pin_index].set_low();
            }
        }
    }

    fn step_forward(self: &mut Self) {
        self.step += 1;
        self.update_pin_states();
    }

    fn step_back(self: &mut Self) {
        self.step -= 1;
        self.update_pin_states();
    }
}
