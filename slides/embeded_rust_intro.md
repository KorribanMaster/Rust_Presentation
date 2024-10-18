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

<img src="./assets/baxter_logo.png" alt="Logo" class="logo">

---

# Introduction
## Blinking an LED with Embedded Rust

- Introduction to embedded Rust development
- Goal: Blink an LED on a microcontroller
- Explore various levels of abstraction within the embedded Rust ecosystem

---

# Low-Level LED Control

- Blinking an LED is the "Hello World" of embedded programming
- We will use a **Nucleo-144** with an STM32F767 microcontroller
- To blink an LED, control pins Row 1 and Column 1 by setting digital outputs

---

# Microcontroller Pin Configuration

- Use GPIO to control pins
- Identify pins using board schematics:
  - **User LD2**: a blue user LED is connected to PB7.
  - **User LD3**: a red user LED is connected to PB14
- GPIO pins are configured by writing to memory-mapped registers
- Memory safety: Use Rust's `unsafe` block to write directly to registers

```rust
unsafe {
    core::ptr::write_volatile(GPIO_PORT_21, VALUE);
}
```

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

# Peripheral Access Crates (PAC)

- PACs provide access to microcontroller peripherals
- These crates offer safer abstractions over low-level operations
- Automatically generated from **SVD** files (standardized peripheral descriptions)

Example:
```rust
let peripherals = pac::Peripherals::take().unwrap();
let gpio = &peripherals.GPIO;
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
let board = bsp::Board::take().unwrap();
let mut led = board.leds.get(0).unwrap();
led.on();
```

- Convenient for managing multiple peripherals on a specific board

---

# Layers of Embedded Rust

![bg right contain](./assets/embedded_layers.png)

---

# Conclusion: Embedded Rust

- We explored different approaches to blink an LED using Rust
- Started with low-level register manipulation using `unsafe` Rust
- Moved up to safer abstractions using PAC, HAL, and BSP
- Each layer offers more safety and ease of use, while maintaining performance

