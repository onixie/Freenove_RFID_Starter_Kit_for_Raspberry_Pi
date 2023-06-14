mod adc;

use adc::ads7830;
use anyhow::{Context, Result};
use rppal::i2c::I2c;
use std::thread;
use std::time::Duration;

const VCC: f64 = 3.3;
const RESOLUTION: i32 = 2i32.pow(8) - 1;
const R: f64 = 10.0; // kiloohm
const T1: f64 = 25.0; // celcius
const K0: f64 = 273.15; // celcius
const B: f64 = 3950.0; // thermal index

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

    loop {
        let value = adc.smbus_read_byte(cmd.bits())?;
        let voltage = value as f64 / RESOLUTION as f64 * VCC;

        let rt = R * voltage / (VCC - voltage);
        // T2 = 1/(1/T1 + ln(Rt/R)/B)
        let t2 = 1.0 / (1.0 / (K0 + T1) + (rt / R).ln() / B) - K0;

        println!("ADC value : {value}  ,\tVoltage : {voltage:.2}V  ,\tTemperature : {t2:.2}C");

        thread::sleep(Duration::from_millis(30));
    }
}
