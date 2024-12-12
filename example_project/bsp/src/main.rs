#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
pub mod bsp;
use bsp::Board;
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    let mut board = Board::new();
    let speeds = [5_000, 10_000, 20_000];
    let mut speed_select = 0;
    loop {
        for _ in 0..speeds[speed_select] {
            board.led1.on();
            board.led2.on();
            board.led3.on();
        }
        for _ in 0..10_000 {
            board.led1.off();
            board.led2.off();
            board.led3.off();
        }
        if board.button.pressed() {
            speed_select += 1;
        }
    }
}
