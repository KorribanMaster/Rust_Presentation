#![no_main]
#![no_std]

use defmt_example as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("main");

    defmt::panic!()
}
