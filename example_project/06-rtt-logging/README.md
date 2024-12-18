# Blinky using HAL Crate for STM32F767ZI

This is a project that uses RTT to print log messages to the host using the STM32f767ZI nucleo board.

## Project Overview

The goal of this project is to demonstrate the use of the HAL layer to control the on-board LED2. The project includes:

- Interfacing with the STM32F767ZI microcontroller using the `rtt-target` crate.
- A simple program to write log messages.

## Dependencies

This project uses the following crates:

- [cortex-m](https://docs.rs/cortex-m/0.7.7/cortex_m/): Low level access to Cortex-M processors
- [cortex-m-rt](https://docs.rs/cortex-m-rt/latest/cortex_m_rt/): Startup code and minimal runtime for Cortex-M microcontrollers
- [panic-halt](https://crates.io/crates/panic-halt): Set the panicking behavior to halt
- [stm32f7xx-hal](https://crates.io/crates/stm32f7xx-hal): HAL for the STM32F7 family of microcontrollers
- [embedded-hal](https://crates.io/crates/embedded-hal): To abstract over the low-level hardware functions.
- [rtt-target](https://crates.io/crates/rtt-target): Target side implementation of the RTT (Real-Time Transfer) I/O protocol

## Run

To get started with this project, follow these steps:

1. Ensure you have Rust and the required tools from [before](../README.md) installed.
2. Navigate to the `03-hal` directory.
3. Run `cargo build` to build the project.
4. Connect your STM32F767ZI Nucleo board to your computer.
5. Run `cargo run` to flash the program to the board.
6. Toggle loggging of floats using `cargo run --features float`

## Usage

Once the program is running, the log messages shall appear in the console

## Explanation

To examine the code sizes install `cargo-binutils`

```shell
cargo install cargo-binutils
```

To get the sizes

````shell
cargo size --release -- -A
cargo size --release --features float -- -A
```
