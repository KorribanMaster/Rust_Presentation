# Blinky using HAL Crate for STM32F767ZI

This is a project that implements a custom Board Support Package (BSP) to blink the LED2 on the STM32f767ZI nucleo board.

## Project Overview

The goal of this project is to demonstrate the creation of a custom BSP to control the on-board LED2. The project includes:

- A demonstration how to use dynamic allocation required for trait objects
- A bsp library that abstract initialization of the board
- A simple program to blink the LED2 on the board.

## Dependencies

This project uses the following crates:

- `stm32f7xx-hal`: The HAL layer for STM32F767ZI.
- [cortex-m](https://crates.io/crates/cortex-m): Low level access to Cortex-M processors.
- `embedded-hal`: To abstract over the low-level hardware functions.
- [embedded-alloc](https://crates.io/crates/embedded-alloc): Implements an allocator (malloc+free)

## Run

To get started with this project, follow these steps:

1. Ensure you have Rust and the required tools from [before](../README.md) installed.
2. Navigate to the `hal` directory.
3. Run `cargo build` to build the project.
4. Connect your STM32F767ZI Nucleo board to your computer.
5. Run `cargo run` to flash the program to the board.

## Usage

Once the program is running, the LED2 on the board should start blinking. You can adjust the blink rate by modifying the delay in the source code.