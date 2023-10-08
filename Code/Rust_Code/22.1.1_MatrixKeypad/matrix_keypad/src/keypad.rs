mod key;
use anyhow::Result;
use key::Key;
use rppal::gpio::{Error, Gpio, InputPin, IoPin, Mode};
use std::time::{Duration, Instant};

pub struct Keypad {
    keys: Vec<Vec<Key>>,
    row_size: usize,
    col_size: usize,
    row_pins: Vec<InputPin>,
    col_pins: Vec<IoPin>,
    start_time: Instant,
    debounce_time: Duration,
    hold_time: Duration,
    keypad_event_listener: Option<fn(&Key)>,
}

impl Keypad {
    pub fn new<const R: usize, const C: usize>(
        keys: &[[char; C]; R],
        row_pins: &[u8; R],
        col_pins: &[u8; C],
    ) -> Result<Self> {
        let default_hold_time = Duration::from_millis(500);
        let default_debounce_time = Duration::from_millis(50);

        let keys = keys
            .iter()
            .enumerate()
            .map(|(ri, cols)| {
                cols.iter()
                    .enumerate()
                    .map(|(ci, sym)| Key::new_with(*sym, ri * C + ci, default_hold_time))
                    .collect()
            })
            .collect();

        let gpio = Gpio::new()?;

        let row_pins = row_pins
            .iter()
            .map(|pin| gpio.get(*pin).map(|pin| pin.into_input_pullup()))
            .collect::<Result<Vec<_>, Error>>()?;

        let col_pins = col_pins
            .iter()
            .map(|pin| gpio.get(*pin).map(|pin| pin.into_io(Mode::Input)))
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(Keypad {
            keys,
            row_size: R,
            col_size: C,
            row_pins,
            col_pins,
            start_time: Instant::now(),
            debounce_time: default_debounce_time,
            hold_time: default_hold_time,
            keypad_event_listener: None,
        })
    }

    pub fn get_key(&mut self) -> Option<char> {
        if self.get_keys() {
            self.keys
                .iter()
                .flatten()
                .find(|key| key.is_pressed())
                .map(|key| key.kchar)
        } else {
            None
        }
    }

    pub fn get_keys(&mut self) -> bool {
        if self.start_time.elapsed() > self.debounce_time {
            self.start_time = Instant::now();
            return self.scan();
        }
        false
    }

    fn scan(&mut self) -> bool {
        let mut any_activity = false;

        for c in 0..self.col_size {
            self.col_pins[c].set_mode(Mode::Output);
            self.col_pins[c].set_low();
            for r in 0..self.row_size {
                let key = &mut self.keys[r][c];

                any_activity |= key.next_state(self.row_pins[r].is_low());

                if key.state_changed {
                    if let Some(handler) = self.keypad_event_listener {
                        handler(key);
                    }
                }
            }
            self.col_pins[c].set_high();
            self.col_pins[c].set_mode(Mode::Input);
        }

        any_activity
    }

    pub fn wait_for_key(&mut self) -> char {
        loop {
            if let Some(key) = self.get_key() {
                return key;
            }
        }
    }

    pub fn set_debounce_time(&mut self, debounce_time: Duration) {
        self.debounce_time = debounce_time;
    }

    pub fn set_hold_time(&mut self, hold_time: Duration) {
        self.hold_time = hold_time;
    }

    pub fn add_event_listener(&mut self, listener: fn(&Key)) {
        self.keypad_event_listener = Some(listener);
    }
}
