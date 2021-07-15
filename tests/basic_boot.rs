#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(moon_os::test::runner)]
#![reexport_test_harness_main = "test_main"]

/*!
 *  Basic boot testing.
 */

mod util;

use moon_os::println;

/**
 *  Tests `println!` in a basic boot.
 */
#[test_case]
fn test_println() {
    println!("basic_boot println! test");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    moon_os::hlt_loop();
}
