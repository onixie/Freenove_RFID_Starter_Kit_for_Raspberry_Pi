use std::error::Error;
use rppal::gpio::Gpio;

// BCM numbers for the GPIO pins
const LED_PIN: u8 = 17_u8;
const BUTTON_PIN: u8 = 18_u8;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Program is starting ... ");

    let mut led_pin = Gpio::new()?
        .get(LED_PIN)?
        .into_output();

    let button_pin = Gpio::new()?
        .get(BUTTON_PIN)?
        .into_input_pullup();

    loop {
        if button_pin.is_low() {
            led_pin.set_high();
            println!("Button is pressed, led turned on >>>");
        } else {
            led_pin.set_low();
            println!("Button is released, led turned off <<<");
        }
    }
}
