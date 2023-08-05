mod ultrasonic;

use anyhow::Result;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    println!("Program is starting ...");

    let mut range_detector = ultrasonic::RangeDetector::new(23, 24, 2.2)?;

    loop {
        if let Ok(distance) = range_detector.get_distance() {
            println!("The distance is : {:.2} cm", distance * 100.0);
            thread::sleep(Duration::from_secs(1));
        }
    }
}
