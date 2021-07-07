// Don't link Rust's stdlib and disable all Rust-level entry points
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Don't mangle function's name
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Entry point; linker looks for `_start` by default
    loop {}
}
