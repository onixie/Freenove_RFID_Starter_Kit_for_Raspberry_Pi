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

# Cross compile on x86_64 host NixOS operating system

## Setup dev environment

```sh
# for flake user
nix develop

# for non-flake user
nix-shell crossShell.nix
```

## Build the package

```sh
export PACKAGE=<package>
```
```sh
cargo build -p $PACKAGE
```

## Patch the binary
```sh
patchelf --set-interpreter /lib/ld-linux-armhf.so.3 target/*/debug/$PACKAGE
```

## Copy and execute the binary on raspberry pi
```sh
scp -r target/*/debug/$PACKAGE <user>@<pi>:~/
ssh <user>@<pi> -C "./$PACKAGE"
```