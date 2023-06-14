mod adc;

use adc::ads7830;
use anyhow::{Context, Result};
use rppal::gpio::Gpio;
use rppal::i2c::I2c;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    println!("Program is starting ... ");

    let addr = ads7830::Address::default() | ads7830::Address::A0 | ads7830::Address::A1;
    let y_cmd = ads7830::Command::Channel0 | ads7830::Command::AdcOn;
    let x_cmd = ads7830::Command::Channel1 | ads7830::Command::AdcOn;

    let adc = {
        let mut adc = I2c::new()?;
        adc.set_slave_address(addr.bits())
            .with_context(|| format!("Not found device in address {addr:#x}"))?;
        println!("Found device in address {addr:#x}");
        adc
    };

    let z_button = Gpio::new()?.get(18)?.into_input_pullup();

    loop {
        let x_value = adc.smbus_read_byte(x_cmd.bits())?;
        let y_value = adc.smbus_read_byte(y_cmd.bits())?;
        let z_value = z_button.read();

        println!("val_X: {x_value}  ,\tval_Y: {y_value}  ,\tval_Z: {z_value} ");
        thread::sleep(Duration::from_millis(100));
    }
}
