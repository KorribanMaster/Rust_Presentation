#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f7xx_hal::{
    prelude::*,
    pac,
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let gpioc = p.GPIOB.split();
    let mut led = gpioc.pb7.into_push_pull_output();

    loop {
        for _ in 0..10_000 {
            led.set_high();
        }
        for _ in 0..10_000 {
            led.set_low();
        }
    }
}