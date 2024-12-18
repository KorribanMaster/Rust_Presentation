#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::asm::nop;
use panic_halt as _; // Panic handler
use stm32f7xx_hal::{
    prelude::*,
    pac,
};
#[entry]
fn main() -> ! {
    // Safety: We are ensuring we have exclusive access to the device peripherals.
    let dp = unsafe { pac::Peripherals::steal() };

    // 1. Enable GPIOB clock by setting the GPIOBEN bit in the RCC_AHB1ENR register.
    dp.RCC.ahb1enr.modify(|_, w| w.gpioben().set_bit());

    // 2. Set PB7 as output (01) in the GPIOB_MODER register.
    dp.GPIOB.moder.modify(|_, w| w.moder7().output());

    // 3. Set PB7 as push-pull (0) in the GPIOB_OTYPER register.
    dp.GPIOB.otyper.modify(|_, w| w.ot7().clear_bit());

    // 4. Set PB7 speed to medium (01) in the GPIOB_OSPEEDR register.
    dp.GPIOB.ospeedr.modify(|_, w| w.ospeedr7().medium_speed());

    // 5. Set no pull-up, no pull-down (00) in the GPIOB_PUPDR register.
    dp.GPIOB.pupdr.modify(|_, w| w.pupdr7().floating());

    // Main loop to toggle the PB7 pin indefinitely
    loop {
        // 6. Toggle PB7 by modifying the GPIOB_ODR register.
        if dp.GPIOB.odr.read().odr7().bit_is_clear() {
            // Set PB7 high
            dp.GPIOB.bsrr.write(|w| w.bs7().set_bit());
        } else {
            // Set PB7 low
            dp.GPIOB.bsrr.write(|w| w.br7().set_bit());
        }

        // 7. Perform 800,000 NOP operations (simple delay).
        for _ in 0..100_000 {
            nop()
        }
    }
}

