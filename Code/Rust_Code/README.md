# Sample code in Rust

```sh
cargo run -p hello
```

```sh
cargo run -p blink
```

```sh
cargo run -p button_led
```

```sh
# polling and detecting continous stable signals 
cargo run -p table_lamp
# or async interrupt with debounced reactive stream
cargo run -p table_lamp --features reactive
```

```sh
cargo run -p light_water
```

```sh
# software PWM
cargo run -p breathing_led
# or hardware PWM (Precondition: PWM is enabled in the kernel)
cargo run -p breathing_led --features hard
```