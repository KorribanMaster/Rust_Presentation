---
marp: true
theme: baxter
class:
    - invert
paginate: true

---
<!-- _class: lead -->
<!-- paginate: skip -->

# Embedded Rust

![bg right:33% contain](./assets/Rust_programming_language_black_logo.svg)
**Introduction to Rust for C Developers**
 By: Eicke Hecht

<style>
  img {
    background-color: transparent;
  }
</style>

---

# Rust on embedded devices

- Introduction to embedded Rust development
- Setup embedded Rust tooling
- Explore various levels of abstraction within the embedded Rust ecosystem
- Logging with defmt
- Testing with defm and embedded-test

---

# Tooling

- cross compilation
- probe-rs
- flip-link

---

# Cross compilation

- Rust uses the LLVM compiler
- Cross compilation is tightly integrated into cargo

```shell
rustup target add thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf
```

---

# Automating cross compilation

Cross compilation can be setup for a crate or workspace using `rust-toolcahin.toml`

```toml
[toolchain]
channel = "1.82"
components = [ "rust-src", "rustfmt", "llvm-tools" ]
targets = [
    "thumbv7em-none-eabihf",
]
```

---

# probe-rs

- Tool to programm, erase and debug an embedded target
- Print to STDOUT via RTT and defmt encoding when using probe-rs run.
- cargo-flash can be used to just flash a target
- cargo-embed can be used to get a full RTT terminal

```shell
cargo install probe-rs-tools --locked
```

---

# probe-rs supported debuggers

- STLink
- Segger JLink
- FTDI based JTAG probes
- USB-JTAG in ESP32

[preobe-rs setup](https://probe.rs/docs/getting-started/probe-setup/)

---

# probe-rs VSCode plugin

- use probe-rs via the DAP interface from VSCode

---

# Use probe-rs with cargo run

Create a `.cargo/config.toml`

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
# replace STM32F429ZITx with your chip as listed in `probe-rs chip list`
runner = "probe-rs run --chip STM32F777ZITx"

[build]
target = "thumbv7em-none-eabihf"
```

---

# probe-rs VSCode plugin

- use probe-rs via the DAP interface from VSCode

---

# flip-link

- Used to link the applications

```shell
cargo install flip-link
```

and add this to `.cargo/config.toml`

```toml
rustflags = [
  "-C", "linker=flip-link",
  "-C", "link-arg=-Tlink.x",
  "-C", "link-arg=--nmagic",
]
```

---

## Problem

- **Bare metal Rust programs may not be memory safe in presence of stack overflows.**
- Example: Rust programs based on v0.6.x of the cortex-m-rt crate.

```rust
fn main() {
    let _x = fib(100);
}

#[inline(never)]
fn fib(n: u32) -> u32 {
    let _use_stack = [0xAA; 1024]; // allocate and initialize 4 kilobytes of stack memory

    if n < 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2) // recursion
    }
}
```

---

# Overflow

- Left: Default memory layout of ARM Cortex-M programs
- Right: Stack overflow condition

![overflow](./assets/overflow.svg)

---

# Solution: Flipped Memory Layout

- Place the stack below the .bss + .data region.
- Ensure the stack does not collide with static variables.
- Collide with the boundary of the physical RAM memory region instead.

---

# Flipped memory layout

![flipped](./assets/flipped.svg)

---

# flip-link Implementation

- Adds zero-cost stack overflow protection.
- Produces the flipped memory layout.
- Handles stack overflow conditions with a HardFault exception handler.

---

# Blinking an LED with Embedded Rust

- Goal: Blink an LED on a microcontroller
- Explore various levels of abstraction within the embedded Rust ecosystem

---

# Low-Level LED Control

- Blinking an LED is the "Hello World" of embedded programming
- We will use a **Nucleo-144** with an STM32F767 microcontroller
- To blink an LED, control pins Row 1 and Column 1 by setting digital outputs

---

# Unsafe Rust for Direct Memory Access

- Rust ensures memory safety with the **borrow checker**
- When direct memory access is needed, we use **unsafe** Rust
- Unsafe blocks are used for operations that the borrow checker cannot verify

```rust
unsafe {
    let ptr = 0x50000000 as *mut u32;
    *ptr = 0x1;  // Direct memory write
}
```

- Only use unsafe when absolutely necessary

---

# Microcontroller Pin Configuration

- Use GPIO to control pins
- Identify pins using board schematics:
  - **User LD2**: a blue user LED is connected to PB7.
- GPIO pins are configured by writing to memory-mapped registers
- Memory safety: Use Rust's `unsafe` block to write directly to registers

```rust
const GPIOB_ODR: *mut u32 = 0x4002_0414 as *mut u32; // GPIOB output data register
unsafe {
  let current_state = read_volatile(GPIOB_ODR);
  if current_state & (1 << 7) == 0 {
      // Set PB7 high
      write_volatile(GPIOB_ODR, current_state | (1 << 7));
  } else {
      // Set PB7 low
      write_volatile(GPIOB_ODR, current_state & !(1 << 7));
  }
}
```

---

# Peripheral Access Crates (PAC)

- PACs provide access to microcontroller peripherals
- These crates offer safer abstractions over low-level operations
- Automatically generated from **SVD** files (standardized peripheral descriptions)

---

# Peripheral Access Crates (PAC)

## Example

```rust
let peripherals = pac::Peripherals::take().unwrap();
loop {
        if dp.GPIOB.odr.read().odr7().bit_is_clear() {
            // Set PB7 high
            dp.GPIOB.bsrr.write(|w| w.bs7().set_bit());
        } else {
            // Set PB7 low
            dp.GPIOB.bsrr.write(|w| w.br7().set_bit());
        }
}

```

- Register access is now type-safe and more readable

---

# Hardware Abstraction Layer (HAL)

- The HAL provides a higher-level abstraction built on top of the PAC
- HALs make common tasks, such as configuring GPIO, much simpler and safer
- Example of blinking an LED using a HAL:

```rust
let mut pin = gpioa.pa5.into_push_pull_output();
loop {
    pin.set_high().unwrap();
    delay_ms(1000);
    pin.set_low().unwrap();
    delay_ms(1000);
}
```

- HAL reduces the complexity of peripheral interactions

---

# Board Support Package (BSP)

- BSPs extend the HAL and PAC by including board-specific configurations
- Simplifies working with complex boards like the Micro:bit V2
- Example using a BSP:

```rust
let mut board = bsp::Board::new;
board.led1.on();
```

- Convenient for managing multiple peripherals on a specific board

---

# Layers of Embedded Rust

![bg right contain](./assets/embedded_layers.png)

---

# Logging with defmt

---

# Testing with defmt

---

# Testing with embedded-test

---

# Conclusion: Embedded Rust

- We explored different approaches to blink an LED using Rust
- Started with low-level register manipulation using `unsafe` Rust
- Moved up to safer abstractions using PAC, HAL, and BSP
- Each layer offers more safety and ease of use, while maintaining performance
