use rppal::gpio::Gpio;
use std::error::Error;
use std::thread;
use std::time::Duration;

const LED_PINS: &[u8; 10] = &[17, 18, 27, 22, 23, 24, 25, 2, 3, 8];

fn main() -> Result<(), Box<dyn Error>> {
    let mut led_pins: Vec<_> = LED_PINS
        .iter()
        .map(|&pin| Gpio::new().unwrap().get(pin).unwrap().into_output_high())
        .collect();

    loop {
        for led in led_pins.iter_mut() {
            led.set_low();
            thread::sleep(Duration::from_millis(100));
            led.set_high();
        }
        for led in led_pins.iter_mut().rev() {
            led.set_low();
            thread::sleep(Duration::from_millis(100));
            led.set_high();
        }
    }
}
