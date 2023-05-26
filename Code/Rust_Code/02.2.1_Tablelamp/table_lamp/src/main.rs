use rppal::gpio::{Gpio, Level};
use std::error::Error;
use std::time::Duration;

#[cfg(feature = "reactive")]
use futures::{channel::mpsc, executor::LocalPool};
#[cfg(feature = "reactive")]
use rppal::gpio::Trigger;
#[cfg(feature = "reactive")]
use rxrust::prelude::*;
#[cfg(not(feature = "reactive"))]
use std::time::Instant;

// BCM numbers for GPIO pins
const LED_PIN: u8 = 17;
const BUTTON_PIN: u8 = 18;
const BOUNCE_DURATION: Duration = Duration::from_millis(50);

#[cfg(not(feature = "reactive"))]
fn main() -> Result<(), Box<dyn Error>> {
    println!("Program is starting...");

    let mut led_pin = Gpio::new()?.get(LED_PIN)?.into_output_low();

    let button_pin = Gpio::new()?.get(BUTTON_PIN)?.into_input_pullup();

    let mut last_change_time = Instant::now();
    let mut last_button_state = Level::High;
    let mut transiting_button_state = Level::High;

    loop {
        // read the logic level of the button pin
        let current_button_state = button_pin.read();

        // delay to admit as the last change if unstable
        if current_button_state != transiting_button_state {
            last_change_time = Instant::now();
            transiting_button_state = current_button_state;
        }

        // safe to consider it's stable if the duration
        // from now to the last change is big enough
        if Instant::now() - BOUNCE_DURATION > last_change_time {
            if last_button_state != current_button_state {
                // update the last state to the current stable state
                last_button_state = current_button_state;

                // toggle the led if button is pressed
                if last_button_state == Level::Low {
                    println!("Button is pressed!");

                    led_pin.toggle();
                    if led_pin.is_set_high() {
                        println!("turn on LED ...");
                    } else {
                        println!("turn off LED ...");
                    }
                } else {
                    println!("Button is released!");
                }
            }
        }
    }
}

#[cfg(feature = "reactive")]
fn main() -> Result<(), Box<dyn Error>> {
    println!("Program is starting...");

    let mut led_pin = Gpio::new()?.get(LED_PIN)?.into_output_low();

    let mut button_pin = Gpio::new()?.get(BUTTON_PIN)?.into_input_pullup();

    // setup async interrupt to button event
    let (mut tx, rx) = mpsc::channel(256);
    button_pin
        .set_async_interrupt(Trigger::Both, move |level| {
            // send logical level
            tx.try_send(level).unwrap();
        })
        .unwrap();

    let mut scheduler = LocalPool::new();
    // turn a receiver stream into an observable
    observable::from_stream(rx, scheduler.spawner())
        // filter out transiting level
        .debounce(BOUNCE_DURATION, scheduler.spawner())
        .subscribe(move |level| {
            // bug(rppal?): changing led_pin's logic level during
            // handling a falling edge interrupt of the button_pin
            // misfires an extra unexpected falling edge interrupt.
            //
            // doing so during a raising edge interrupt seems ok.
            if level == Level::High {
                println!("Button has been pressed!");
                led_pin.toggle();
                if led_pin.is_set_high() {
                    println!("turn on LED ...");
                } else {
                    println!("turn off LED ...");
                }
            }
        });

    // execute the scheduled observable
    scheduler.run();

    Ok(())
}
