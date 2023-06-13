mod adc;

use adc::ads7830;
use anyhow::{Context, Result};
use rppal::gpio::Gpio;
use rppal::i2c::I2c;
use std::thread;
use std::time::Duration;

const PWM_FREQUENCY: f64 = 2000.0;
const VCC: f64 = 3.3;
const RESOLUTION: i32 = 2i32.pow(8) - 1;

fn main() -> Result<()> {
    println!("Program is starting ... ");

    let addr = ads7830::Address::default() | ads7830::Address::A0 | ads7830::Address::A1;
    let cmd = ads7830::Command::Channel0 | ads7830::Command::AdcOn;

    let adc = {
        let mut adc = I2c::new()?;
        adc.set_slave_address(addr.bits())
            .with_context(|| format!("Not found device in address {addr:#x}"))?;
        println!("Found device in address {addr:#x}");
        adc
    };

    let mut led = Gpio::new()?.get(17)?.into_output_low();

    loop {
        let value = adc.smbus_read_byte(cmd.bits())?;
        let voltage = value as f64 / RESOLUTION as f64 * VCC;
        println!("ADC value : {value}  ,\tVoltage : {voltage:.2}V");

        led.set_pwm_frequency(PWM_FREQUENCY, value as f64 / RESOLUTION as f64)?;

        thread::sleep(Duration::from_millis(30));
    }
}
