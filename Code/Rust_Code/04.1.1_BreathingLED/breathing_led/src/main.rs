use std::error::Error;
use std::thread;
use std::time::Duration;

#[cfg(not(feature = "hard"))]
use rppal::gpio::Gpio;
#[cfg(feature = "hard")]
use rppal::pwm::{Channel, Polarity, Pwm};

// PWM period = 1 milliseconds
const PWM_FREQUENCY: f64 = 1000.0;

#[cfg(not(feature = "hard"))]
fn main() -> Result<(), Box<dyn Error>> {
    println!("Program is starting ... ");

    // Use GPIO18 as normal output pin with software pwm
    let mut led_pin = Gpio::new()?.get(18)?.into_output_low();
    led_pin.set_reset_on_drop(true);

    loop {
        for duty_cycle in 0..=100 {
            // 100 x 20 milliseconds from 0v ~ full voltage
            // increase the duty_cycle therefore voltage is increased
            led_pin.set_pwm_frequency(PWM_FREQUENCY, (duty_cycle as f64) / 100.0)?;
            thread::sleep(Duration::from_millis(20));
        }
        std::thread::sleep(Duration::from_millis(300));
        for duty_cycle in 0..=100 {
            // 100 x 20 milliseconds from full voltage ~ 0v
            // decrease the duty_cycle therefore voltage is decreased
            led_pin.set_pwm_frequency(PWM_FREQUENCY, (100.0 - duty_cycle as f64) / 100.0)?;
            thread::sleep(Duration::from_millis(20));
        }
    }
}

#[cfg(feature = "hard")]
fn main() -> Result<(), Box<dyn Error>> {
    println!("Program is starting ... ");

    // GPIO18 is PWM Channel 0 when hardware PWM is enabled
    let mut led_pin =
        Pwm::with_frequency(Channel::Pwm0, PWM_FREQUENCY, 0.0, Polarity::Normal, true)?;
    led_pin.set_reset_on_drop(true);

    loop {
        for duty_cycle in 0..=100 {
            led_pin.set_duty_cycle((duty_cycle as f64) / 100.0)?;
            thread::sleep(Duration::from_millis(20));
        }
        std::thread::sleep(Duration::from_millis(300));
        for duty_cycle in 0..=100 {
            led_pin.set_duty_cycle((100.0 - duty_cycle as f64) / 100.0)?;
            thread::sleep(Duration::from_millis(20));
        }
    }
}
