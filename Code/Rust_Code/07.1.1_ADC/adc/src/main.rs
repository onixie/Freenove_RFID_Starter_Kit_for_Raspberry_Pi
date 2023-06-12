use anyhow::{Context, Result};
use rppal::i2c::I2c;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    println!("Program is starting ... ");

    let adc = {
        let addr_of_ads7830 = 0b_0000_0000_0100_1011;
        let mut adc = I2c::new()?;
        adc.set_slave_address(addr_of_ads7830)
            .with_context(|| format!("Not found device in address {addr_of_ads7830:#x}"))?;
        println!("Found device in address {addr_of_ads7830:#x}");
        adc
    };

    let chn: u8 = 0;
    loop {
        let cmd = 0b_1000_0100u8 | (((chn << 2 | chn >> 1) & 0x07) << 4);
        let value = adc.smbus_read_byte(cmd)?;
        let voltage = value as f64 / 255.0 * 3.3;

        println!("ADC value : {value}  ,\tVoltage : {voltage:.2}V");

        thread::sleep(Duration::from_millis(100));
    }
}
