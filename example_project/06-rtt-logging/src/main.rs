#![no_std]
#![no_main]
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rtt_init_print, rprintln};
use stm32f7xx_hal as _;
#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut i = 0;
    #[cfg(feature = "float")]
    let mut f = 0.0;
    
    rprintln!("Hello, world!");
    loop {
        rprintln!("i = {}", i);
        i += 1;
        #[cfg(feature = "float")]
        {
            f += 0.1;
            rprintln!("f = {}", f);
        }

    }
}
