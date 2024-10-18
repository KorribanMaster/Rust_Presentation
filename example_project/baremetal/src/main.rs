#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _; // Panic handler
use core::ptr::{read_volatile, write_volatile};

const RCC_AHB1ENR: *mut u32 = 0x4002_3830 as *mut u32; // RCC AHB1 peripheral clock enable register
const GPIOB_MODER: *mut u32 = 0x4002_0400 as *mut u32; // GPIOB mode register
const GPIOB_OTYPER: *mut u32 = 0x4002_0404 as *mut u32; // GPIOB output type register
const GPIOB_OSPEEDR: *mut u32 = 0x4002_0408 as *mut u32; // GPIOB output speed register
const GPIOB_PUPDR: *mut u32 = 0x4002_040C as *mut u32; // GPIOB pull-up/pull-down register
const GPIOB_ODR: *mut u32 = 0x4002_0414 as *mut u32; // GPIOB output data register

#[entry]
fn main() -> ! {
    unsafe {
        // 1. Enable GPIOB clock by setting the GPIOBEN bit in the RCC_AHB1ENR register.
        write_volatile(RCC_AHB1ENR, read_volatile(RCC_AHB1ENR) | (1 << 1));

        // 2. Set PB7 as output (01) in the GPIOB_MODER register.
        write_volatile(GPIOB_MODER, read_volatile(GPIOB_MODER) & !(3 << (2 * 7))); // Clear the mode bits
        write_volatile(GPIOB_MODER, read_volatile(GPIOB_MODER) | (1 << (2 * 7)));  // Set to output mode (01)

        // 3. Set PB7 as push-pull (0) in the GPIOB_OTYPER register.
        write_volatile(GPIOB_OTYPER, read_volatile(GPIOB_OTYPER) & !(1 << 7));

        // 4. Set PB7 speed to medium (01) in the GPIOB_OSPEEDR register.
        write_volatile(GPIOB_OSPEEDR, read_volatile(GPIOB_OSPEEDR) & !(3 << (2 * 7))); // Clear speed bits
        write_volatile(GPIOB_OSPEEDR, read_volatile(GPIOB_OSPEEDR) | (1 << (2 * 7)));  // Set speed to medium

        // 5. Set no pull-up, no pull-down (00) in the GPIOB_PUPDR register.
        write_volatile(GPIOB_PUPDR, read_volatile(GPIOB_PUPDR) & !(3 << (2 * 7)));

        // Main loop to toggle the PB7 pin indefinitely
        loop {
            // 6. Toggle PB7 by modifying the GPIOB_ODR register.
            let current_state = read_volatile(GPIOB_ODR);
            if current_state & (1 << 7) == 0 {
                // Set PB7 high
                write_volatile(GPIOB_ODR, current_state | (1 << 7));
            } else {
                // Set PB7 low
                write_volatile(GPIOB_ODR, current_state & !(1 << 7));
            }

            // 7. Perform 800,000 NOP operations (simple delay).
            for _ in 0..100_000 {
                nop();
            }
        }
    }
}

