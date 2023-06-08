use rppal::gpio::Gpio;
use rppal::pwm::{Channel, Polarity, Pwm};
use std::error::Error;
use std::thread;
use std::time::Duration;

const BUTTON_PIN: u8 = 17;
const FREQ_CENTER: f64 = 2000.0;
const FREQ_DEVIATION: f64 = 500.0;
const DUTY_CYCLE: f64 = 0.1;

fn siren_track() -> impl Iterator<Item = f64> {
    (0..360).cycle().map(|deg| {
        let r = deg as f64 * std::f64::consts::PI / 180.0;
        FREQ_CENTER + r.sin() * FREQ_DEVIATION
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Program is starting ... ");

    let button_pin = Gpio::new()?.get(BUTTON_PIN)?.into_input_pullup();
    let alertor = Pwm::with_frequency(
        Channel::Pwm0,
        FREQ_CENTER,
        DUTY_CYCLE,
        Polarity::Normal,
        false,
    )?;
    let mut sound = siren_track();

    loop {
        if button_pin.is_low() {
            if !alertor.is_enabled()? {
                alertor.enable()?;
            }
            let freq = sound.next().unwrap_or(FREQ_CENTER);
            alertor.set_frequency(freq, DUTY_CYCLE)?;
            println!("alertor turned on >>> ");
        } else {
            if alertor.is_enabled()? {
                alertor.disable()?;
            }
            println!("alertor turned off >>> ");
        }
        thread::sleep(Duration::from_millis(1));
    }
}
