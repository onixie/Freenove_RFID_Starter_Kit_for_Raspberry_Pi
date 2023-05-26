
use std::error::Error;
use rppal::gpio::Gpio;
use std::time::Duration;
use std::thread;

// BCM numbers for the GPIO pins
const LED_PIN: u8 = 17;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Program is starting ...");

    let mut led_pin = Gpio::new()?
        .get(LED_PIN)?
        .into_output();

    println!("Using pin {}", PIN);
    loop {
        led_pin.set_high();
        println!("led turned on >>>");
        thread::sleep(Duration::from_millis(1000));

        led_pin.set_low();
        println!("led turned off <<<");
        thread::sleep(Duration::from_millis(1000));
    }
}
