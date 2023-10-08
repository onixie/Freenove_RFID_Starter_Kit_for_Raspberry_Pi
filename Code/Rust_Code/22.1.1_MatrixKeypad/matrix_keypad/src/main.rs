mod keypad;
use anyhow::Result;
use keypad::Keypad;

const KEY_SYMS: [[char; 4]; 4] = [
    ['1', '2', '3', 'A'],
    ['4', '5', '6', 'B'],
    ['7', '8', '9', 'C'],
    ['*', '0', '#', 'D'],
];

const ROW_PINS: [u8; 4] = [18, 23, 24, 25];
const COL_PINS: [u8; 4] = [10, 22, 27, 17];

fn main() -> Result<()> {
    println!("Program is starting ... ");

    let mut keypad = Keypad::new(&KEY_SYMS, &ROW_PINS, &COL_PINS)?;

    keypad.add_event_listener(|key| {
        println!(
            "Keypad event: char={} code={} state={:?}",
            key.kchar, key.kcode, key.state
        );
    });

    loop {
        if let Some(key) = keypad.get_key() {
            println!("You Pressed key : {key}");
        }
    }
}
