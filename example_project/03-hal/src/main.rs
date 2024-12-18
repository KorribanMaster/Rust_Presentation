#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f7xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let gpiob = p.GPIOB.split();
    let mut led = gpiob.pb7.into_push_pull_output();

    loop {
        for _ in 0..10_000 {
            led.set_high();
        }
        for _ in 0..10_000 {
            led.set_low();
        }
    }
}
