// Don't link Rust's stdlib and disable all Rust-level entry points
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Define constant string "Hello World!"
static HELLO: &[u8] = b"Hello World!";

// Don't mangle function's name
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Entry point; linker looks for `_start` by default
    
    // Define pointer to VGA buffer location in memory
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
