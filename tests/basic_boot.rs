#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(moon_os::test::runner)]
#![reexport_test_harness_main = "test_main"]

/*!
 *  Basic boot testing.
 */

use core::panic::PanicInfo;

use moon_os::println;
use moon_os::panic;

/**
 *  Call test panic handler.
 */
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic::test_handler(info);
}

/**
 *  Test `println!` in a basic boot.
 */
#[test_case]
fn test_println() {
    println!("basic_boot println! test");
}

/**
 *  Entry point for `cargo test`.
 */
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}
