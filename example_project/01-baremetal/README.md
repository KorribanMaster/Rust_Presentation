# Blinky baremetal for STM32F767ZI

This is a project that implements a blinking the LED2 on the STM32f767ZI nucleo board.

## Project Overview

The goal of this project is to demonstrate the creation of a custo BSP to control the on-board LED2. The project includes:

- A simple program to blink the LED2 on the board.

## Dependencies

This project uses the following crates:
- [cortex-m](https://docs.rs/cortex-m/0.7.7/cortex_m/): Low level access to Cortex-M processors
- [cortex-m-rt](https://docs.rs/cortex-m-rt/latest/cortex_m_rt/): Startup code and minimal runtime for Cortex-M microcontrollers
- [panic-halt](https://crates.io/crates/panic-halt): Set the panicking behavior to halt

## Run

To get started with this project, follow these steps:

1. Ensure you have Rust and the required tools from [before](../README.md) installed.
2. Navigate to the `01-baremetal` directory.
3. Run `cargo build` to build the project.
4. Connect your STM32F767ZI Nucleo board to your computer.
5. Run `cargo run` to flash the program to the board.

## Usage

Once the program is running, the LED2 on the board should start blinking. You can adjust the blink rate by modifying the delay in the source code.

## Explanation

Modifies the registers associated with GPIO PB7 directly using unsafe rust