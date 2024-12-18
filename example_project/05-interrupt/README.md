# Blinky using HAL Crate for STM32F767ZI

This is a project that implements a button press to toggle the LED2 on the STM32f767ZI nucleo board.

## Project Overview

The goal of this project is to demonstrate the creation of a custom BSP to control the on-board LED2. The project includes:

- Setting up an interrupt for the button pin
- A simple program to toggle the LED1 on the board.

## Dependencies

This project uses the following crates:

- [cortex-m](https://docs.rs/cortex-m/0.7.7/cortex_m/): Low level access to Cortex-M processors
- [cortex-m-rt](https://docs.rs/cortex-m-rt/latest/cortex_m_rt/): Startup code and minimal runtime for Cortex-M microcontrollers
- [panic-halt](https://crates.io/crates/panic-halt): Set the panicking behavior to halt
- [stm32f7xx-hal](https://crates.io/crates/stm32f7xx-hal): HAL for the STM32F7 family of microcontrollers
- [embedded-hal](https://crates.io/crates/embedded-hal): To abstract over the low-level hardware functions.
- [embedded-alloc](https://crates.io/crates/embedded-alloc): Implements an allocator (malloc+free)

## Run

To get started with this project, follow these steps:

1. Ensure you have Rust and the required tools from [before](../README.md) installed.
2. Navigate to the `05-interrupt` directory.
3. Run `cargo build` to build the project.
4. Connect your STM32F767ZI Nucleo board to your computer.
5. Run `cargo run` to flash the program to the board.

## Usage

Once the program is running, the LED1 on the board should be off. Once you press the USER button the LED1 should toggle its state
