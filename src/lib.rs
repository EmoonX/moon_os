#![no_std]
#![cfg_attr(test, no_main)]  // enable `no_main` when `cargo_test`
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]

/**
 *  Library module.
 *  Defines all public modules to be visible 
 *      inside and outside moon_os crate.
 */

pub mod vga_buffer;
pub mod qemu;
pub mod serial;
pub mod panic;
pub mod test;

/*---------------------------------------------------------------------------*/

use core::panic::PanicInfo;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    /* Call test panic handler. */
    panic::test_handler(info);
}

#[cfg(test)]  // includes function only for testing
#[no_mangle]
pub extern "C" fn _start() -> ! {
    /* Entry point for `cargo test`.
        This is not called in `cargo_run`,
        as `no_main` is not set in such case. */
    test_main();
    loop {}
}
