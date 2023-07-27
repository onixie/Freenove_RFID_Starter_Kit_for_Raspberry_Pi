use anyhow::Result;
use rppal::gpio::Gpio;
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    println!("Program is starting...");

    let mut relay_on = false;
    let mut button_off = true;
    let mut last_button_off = true;
    let mut last_change_time = Instant::now();
    let stable_duration = Duration::from_millis(20);

    let button = Gpio::new()?.get(18)?.into_input_pullup();
    let mut relay = Gpio::new()?.get(17)?.into_output_low();

    loop {
        let reading = button.is_low();

        // update the last read time if the button state isn't stable.
        if reading != last_button_off {
            last_button_off = reading;
            last_change_time = Instant::now();
        }

        // the same button state lasts for the threshold duration.
        if Instant::now() - last_change_time > stable_duration {
            if reading != button_off {
                // update the read button state
                button_off = reading;
                if button_off {
                    println!("Button is pressed!");

                    relay_on = !relay_on;
                    if relay_on {
                        println!("turn on relay ...");
                        relay.set_high();
                    } else {
                        println!("turn off relay ...");
                        relay.set_low();
                    }
                } else {
                    println!("Button is released!");
                }
            }
        }
    }
}
