#![no_std]   // don't link Rust's stdlib
#![no_main]  // disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]  // this function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]  // don't mangle function's name
pub extern "C" fn _start() -> ! {
    /* Entry point; linker looks for `_start` by default. */
    
    println!("Hello world!");
    println!("Hello world {} {} {} {}", 1, 2, 3, '!');
    loop {}
}
