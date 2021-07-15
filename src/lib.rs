#![no_std]
#![cfg_attr(test, no_main)]     // enable `no_main` when `cargo_test`
#![feature(abi_x86_interrupt)]  // enable use of `extern "x86-interrupt"`
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]

/*!
 *  Minimal and personal OS project based on `blog_os`. Made in Rust.
 */

pub mod vga_buffer;
pub mod serial;
pub mod qemu;
pub mod interrupts;
pub mod gdt;
pub mod panic;
pub mod test;


/**
 *  Calls OS initialization routines.
 */
pub fn init(is_test: bool) {
    unsafe { test::ENABLED =  is_test };
    interrupts::init();
}

/*---------------------------------------------------------------------------*/

/**
 *  Calls test panic handler.
 */
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic::test_handler(info);
}

/**
 *  Entry point for `cargo test`.
 *  
 *  Not called in `cargo_run`, as `no_main` is not set in such case.
 */
#[cfg(test)]  // includes function only for testing
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init(true);
    test_main();
    loop {}
}
