use rand::{thread_rng, Rng};
use rppal::gpio::Gpio;
use std::error::Error;
use std::thread;
use std::time::Duration;

const PWM_FREQUENCY: f64 = 1000.0;
static RGB_PIN_ARRAY: [u8; 3] = [17, 27, 18];

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let mut rgb_pins: Vec<_> = RGB_PIN_ARRAY
        .iter()
        .map(|&pin| {
            let mut pin = Gpio::new()
                .unwrap()
                .get(pin)
                .expect("fail to get {pin}")
                .into_output_high();
            pin.set_reset_on_drop(true);
            pin
        })
        .collect();

    loop {
        let rgb: [u8; 3] = rng.gen::<[u8; 3]>();

        for (pin, intensity) in rgb_pins.iter_mut().zip(rgb) {
            pin.set_pwm_frequency(PWM_FREQUENCY, intensity as f64 / std::u8::MAX as f64).expect("fail to set soft pwm");
        }
        println!("r={},  g={},  b={}", rgb[0], rgb[1], rgb[2]);
        thread::sleep(Duration::from_millis(1000));
    }
}
