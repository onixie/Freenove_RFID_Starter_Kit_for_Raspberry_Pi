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
    let r_cmd = ads7830::Command::Channel0 | ads7830::Command::AdcOn;
    let g_cmd = ads7830::Command::Channel1 | ads7830::Command::AdcOn;
    let b_cmd = ads7830::Command::Channel2 | ads7830::Command::AdcOn;

    let adc = {
        let mut adc = I2c::new()?;
        adc.set_slave_address(addr.bits())
            .with_context(|| format!("Not found device in address {addr:#x}"))?;
        println!("Found device in address {addr:#x}");
        adc
    };

    let mut r_led = Gpio::new()?.get(17)?.into_output_high();
    let mut g_led = Gpio::new()?.get(27)?.into_output_high();
    let mut b_led = Gpio::new()?.get(22)?.into_output_high();

    loop {
        let r_value = adc.smbus_read_byte(r_cmd.bits())?;
        let g_value = adc.smbus_read_byte(g_cmd.bits())?;
        let b_value = adc.smbus_read_byte(b_cmd.bits())?;
        println!(
            "ADC Value value_Red: {r_value} ,\tvalue_Green: {g_value} ,\tvalue_Blue: {b_value}"
        );

        r_led.set_pwm_frequency(PWM_FREQUENCY, 1.0 - r_value as f64 / RESOLUTION as f64)?;
        g_led.set_pwm_frequency(PWM_FREQUENCY, 1.0 - g_value as f64 / RESOLUTION as f64)?;
        b_led.set_pwm_frequency(PWM_FREQUENCY, 1.0 - b_value as f64 / RESOLUTION as f64)?;

        thread::sleep(Duration::from_millis(30));
    }
}
