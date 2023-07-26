mod adc;

use adc::ads7830;
use anyhow::{Context, Result};
use ctrlc;
use rppal::gpio::{Gpio, OutputPin};
use rppal::i2c::I2c;
use std::ops::Div;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const MOTOR_IN1: u8 = 27;
const MOTOR_IN2: u8 = 17;
const MOTOR_ENABLE: u8 = 22;

struct Motor {
    motor_in1: OutputPin,
    motor_in2: OutputPin,
    motor_enable: OutputPin,
}

impl Drop for Motor {
    fn drop(&mut self) {
        self.motor_in1.set_low();
        self.motor_in2.set_low();
        self.motor_enable.clear_pwm().unwrap();
        self.motor_enable.set_low();
    }
}

impl Motor {
    fn new() -> Result<Self> {
        let gpio = Gpio::new()?;
        let motor = Motor {
            motor_in1: gpio.get(MOTOR_IN1)?.into_output_low(),
            motor_in2: gpio.get(MOTOR_IN2)?.into_output_low(),
            motor_enable: gpio.get(MOTOR_ENABLE)?.into_output_low(),
        };

        Result::Ok(motor)
    }

    fn run_at(&mut self, speed: u8) -> Result<()> {
        let pivot = u8::MAX / 2;

        if speed > pivot {
            self.motor_in1.set_high();
            self.motor_in2.set_low();
            println!("turn Forward...");
        } else if speed < pivot {
            self.motor_in1.set_low();
            self.motor_in2.set_high();
            println!("turn Back...");
        } else {
            self.motor_in1.set_low();
            self.motor_in2.set_low();
            println!("Motor Stop...");
        }

        let cycle = (speed.abs_diff(pivot) as f64)
            .div(pivot as f64)
            .clamp(0.0, 1.0);

        self.motor_enable.set_pwm_frequency(1000 as f64, cycle)?;

        println!("The PWM duty cycle is {:.2}%", cycle * 100.0);
        Ok(())
    }
}

fn main() -> Result<()> {
    println!("Program is starting ... ");

    let exit = {
        let exit = Arc::new(Mutex::new(false));
        let exit_ = exit.clone();
        ctrlc::set_handler(move || {
            *exit_.lock().unwrap() = true;
        })?;
        exit
    };

    let addr = ads7830::Address::default() | ads7830::Address::A0 | ads7830::Address::A1;
    let cmd = ads7830::Command::Channel0 | ads7830::Command::AdcOn;

    let adc = {
        let mut adc = I2c::new()?;
        adc.set_slave_address(addr.bits())
            .with_context(|| format!("Not found device in addr {addr:#x}"))?;
        println!("Found device in addr {addr:#x}");
        adc
    };

    let mut motor = Motor::new()?;

    loop {
        let speed = adc.smbus_read_byte(cmd.bits())?;
        println!("ADC Value : {speed}");

        motor.run_at(speed)?;

        if *exit.lock().unwrap() {
            break Ok(());
        }

        thread::sleep(Duration::from_millis(100));
    }
}
