#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(moon_os::test::runner)]
#![reexport_test_harness_main = "test_main"]

/**
 *  Basic boot testing.
 */

use core::panic::PanicInfo;

use moon_os::println;
use moon_os::panic;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    /* Call test panic handler. */
    panic::test_handler(info);
}

#[test_case]
fn test_println() {
    /* Test `println!` in `basic_boot`. */
    println!("basic_boot println! test");
}

#[cfg(test)]  // includes function only for testing
#[no_mangle]
pub extern "C" fn _start() -> ! {
    /* Entry point for `cargo test`. */
    test_main();
    loop {}
}
