use rppal::gpio::Gpio;
use std::error::Error;

const BUTTON_PIN: u8 = 18;
const BUZZER_PIN: u8 = 17;

fn main() -> Result<(), Box<dyn Error>> {
    let button_pin = Gpio::new()?.get(BUTTON_PIN)?.into_input_pullup();

    let mut buzzer_pin = Gpio::new()?.get(BUZZER_PIN)?.into_output_low();

    loop {
        if button_pin.is_low() {
            if buzzer_pin.is_set_low() {
                buzzer_pin.set_high();
            }
            println!("buzzer turned on >>> ");
        } else {
            if buzzer_pin.is_set_high() {
                buzzer_pin.set_low();
            }
            println!("buzzer turned off <<< ");
        }
    }
}
