#![no_std]   // don't link Rust's stdlib
#![no_main]  // disable all Rust-level entry points
#![feature(custom_test_frameworks)]           // replaces std test framework
#![test_runner(crate::test::test_runner)]     // defines test runner function
#![reexport_test_harness_main = "test_main"]  // replaces entry fn in testing

use core::panic::PanicInfo;

mod vga_buffer;
mod test;

#[panic_handler]  // this function is called on panic
fn panic(info: &PanicInfo) -> ! {
    /* Just print panic message. */
    println!("{}", info);
    loop {}
}

#[no_mangle]  // don't mangle function's name
pub extern "C" fn _start() -> ! {
    /* Entry point; linker looks for `_start` by default. */
    
    println!("Hello world!");
    println!("Hello world {} {} {} {}", 1, 2, 3, '!');
    // panic!("Some panic message");

    #[cfg(test)]
    test_main();

    loop {}
}
