use rppal::gpio::Level;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq)]
pub enum KeyState {
    Idle,
    Pressed(/*hold timer*/ Instant),
    Hold,
    Released,
}

#[derive(Debug)]
pub struct Key {
    pub kchar: char,
    pub kcode: usize,
    pub state: KeyState,
    pub state_changed: bool,
    hold_time: Duration,
}

impl Key {
    pub fn new_with(kchar: char, kcode: usize, hold_time: Duration) -> Self {
        Key {
            kchar: kchar,
            kcode: kcode,
            state: KeyState::Idle,
            state_changed: false,
            hold_time,
        }
    }

    pub fn key_update(&mut self, kchar: char, state: KeyState, state_changed: bool) {
        self.kchar = kchar;
        self.state = state;
        self.state_changed = state_changed;
    }

    pub fn is_pressed(&self) -> bool {
        if let KeyState::Pressed(_) = self.state {
            return self.state_changed;
        }
        false
    }

    pub fn next_state(&mut self, pressed: bool) -> bool {
        self.state_changed = true;

        match self.state {
            KeyState::Idle if pressed => {
                self.state = KeyState::Pressed(Instant::now());
            }
            KeyState::Pressed(timer) if timer.elapsed() > self.hold_time => {
                self.state = KeyState::Hold;
            }
            KeyState::Pressed(_) if !pressed => {
                self.state = KeyState::Released;
            }
            KeyState::Hold if !pressed => {
                self.state = KeyState::Released;
            }
            KeyState::Released => {
                self.state = KeyState::Idle;
            }
            _ => {
                self.state_changed = false;
            }
        }

        self.state_changed
    }
}
