use anyhow::Result;
use rppal::pwm::{Channel, Polarity, Pwm};
use std::ops::Range;
use std::thread;
use std::time::Duration;

const OPERATING_PERIOD: Duration = Duration::from_millis(20);
const OPREATING_RANGE: Range<u64> = 500..2500; // 0..180 degree
const OPREATING_RESOLUTION: usize = 500;

fn main() -> Result<()> {
    println!("Program is starting ...");

    let servo = Pwm::with_period(
        Channel::Pwm0,
        OPERATING_PERIOD,
        Duration::from_micros(OPREATING_RANGE.start),
        Polarity::Normal,
        true,
    )?;

    let clockwise = OPREATING_RANGE;
    let counter_clockwise = OPREATING_RANGE.rev();

    for value in clockwise
        .chain(counter_clockwise)
        .step_by(OPREATING_RESOLUTION)
        .map(Duration::from_micros)
        .cycle()
    {
        servo.set_pulse_width(value)?;
        println!(
            "period: {:?},\tpulse width: {:?}",
            servo.period()?,
            servo.pulse_width()?
        );
        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
