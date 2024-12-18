
# Example project

This example project is provided alongside the slides with the aim to provide learning material alongside my talks.

The code is organized in a [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) and is meant to run on a `STM32F767ZI` Nucleo board though this is adaptable to other MCUs see []

The following crates are provided with this example

- [baremetal](./baremetal/): A baremetal blinky project
- [pac](./pac/): A blinky application using the Peripheral Access Crate (PAC)
- [hal](./hal/): A blinky application useing the Hardware Abstraction Layer (HAL)
- [defmt-example](./defmt-example/): Example code demonstrating the use of [defmt](https://defmt.ferrous-systems.com/) for logging and testing


## Quick Setup

> Quickly set up a [`probe-rs`] + [`defmt`] + [`flip-link`] embedded project

[`probe-rs`](https://crates.io/crates/probe-rs)
[`defmt`](https://github.com/knurling-rs/defmt)
[`flip-link`](https://github.com/knurling-rs/flip-link)

## Dependencies

### 1. `flip-link`:

```bash
cargo install flip-link
```

### 2. `probe-rs`:

Install probe-rs by following the instructions at <https://probe.rs/docs/getting-started/installation/>.

## Run

You are now all set to `cargo-run` your first Rust-powered application!
Go to the crate/directory you want to run.

- [baremetal](./baremetal/): A baremetal blinky project
- [pac](./pac/): A blinky application using the Peripheral Access Crate (PAC)
- [hal](./hal/): A blinky application useing the Hardware Abstraction Layer (HAL)
- [defmt-example](./defmt-example/): Example code demonstrating the use of [defmt](https://defmt.ferrous-systems.com/) for logging and testing

Start by `cargo run`-ning your chosen crate and reading its respective documentation

## VSCode

If you want to work with VSCode as your IDE i recommend to install the following extensions.

- [rust-analyzer](https://rust-analyzer.github.io/)
- [probe-rs](https://github.com/probe-rs/vscode)

Further reading:

- [RA docs](https://rust-analyzer.github.io/manual.html#configuration)
  
## Setup for a different target MCU

### 1. Set `probe-rs` chip

Pick a chip from ` probe-rs chip list` and enter it into `.cargo/config.toml` of the respective crate you ar working on.

If, for example, you have a nRF52840 Development Kit, replace `STM32F767ZI` with `nRF52840_xxAA`.

```diff
 # .cargo/config.toml
 [target.'cfg(all(target_arch = "arm", target_os = "none"))']
-runner = "probe-rs run --chip {{chip}}"
+runner = "probe-rs run --chip nRF52840_xxAA"
```

### 3. Adjust the compilation target

In `.cargo/config.toml`, pick the right compilation target for your board.

```diff
 # .cargo/config.toml
 [build]
-target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
-# target = "thumbv7m-none-eabi"    # Cortex-M3
-# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
-# target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
+target = "thumbv7em-none-eabihf" # Cortex-M4F (with FPU)
```

Add the target with `rustup`.

```bash
rustup target add thumbv7em-none-eabihf
```

### Add a HAL as a dependency

In `Cargo.toml`, list the Hardware Abstraction Layer (HAL) for your board as a dependency.

For the nRF52840 you'll want to use the [`nrf52840-hal`].

[`nrf52840-hal`]: https://crates.io/crates/nrf52840-hal

```diff
 # Cargo.toml
 [dependencies]
-stm32f7xx-hal = {version = "0.8.0", features = ["stm32f767"]}
+nrf52840-hal = "0.14.0"
```

⚠️ Note for RP2040 users ⚠️

You will need to not just specify the `rp-hal` HAL, but a BSP (board support crate) which includes a second stage bootloader. Please find a list of available BSPs [here](https://github.com/rp-rs/rp-hal-boards#packages).

### Import your HAL

Now that you have selected a HAL, fix the HAL import by searching for 

```diff
 // my-app/src/lib.rs
-use stm32f7xx_hal as _;
+use nrf52840_hal as _;
```

### (Get a linker script)

Some HAL crates require that you manually copy over a file called `memory.x` from the HAL to the root of your project. For nrf52840-hal, this is done automatically so no action is needed. For other HAL crates, you can get it from your local Cargo folder, the default location is under:

```text
~/.cargo/registry/src/
```

Not all HALs provide a `memory.x` file, you may need to write it yourself. Check the documentation for the HAL you are using.


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.