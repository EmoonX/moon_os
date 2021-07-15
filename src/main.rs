#![no_std]   // don't link Rust's stdlib
#![no_main]  // disable all Rust-level entry points
#![feature(custom_test_frameworks)]           // replaces std test framework
#![test_runner(moon_os::test::runner)]        // defines test runner function
#![reexport_test_harness_main = "test_main"]  // replaces entry fn in testing

/*!
 *  Main project module.
 */

use core::panic::PanicInfo;

use moon_os::println;
use moon_os::panic;

/**
 *  Calls normal panic handler.
 */
#[cfg(not(test))]  // compiles only in `cargo run`
#[panic_handler]   // function called on panic
fn panic(info: &PanicInfo) -> ! {
    panic::handler(info);
}

/**
 *  Entry point; linker looks for `_start` by default.
 */
#[no_mangle]  // don't mangle function's name
pub extern "C" fn _start() -> ! {    
    println!("Oi!");
    println!("Hello world {} {} {} {}", 1, 2, 3, '!');
    // panic!("Some panic message");

    // Triggers a page fault
    fn stack_overflow() {
        stack_overflow();
    }
    // Triggers a stack overflow
    moon_os::init(false);
    stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

/*---------------------------------------------------------------------------*/

/**
 *  Calls test panic handler.
 */
#[cfg(test)]  // compiles only in `cargo test`
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic::test_handler(info);
}

/**
 *  Simply trivial assertion test that is always true.
 */
#[test_case]  // defines a test case function
fn trivial_assertion() {
    assert_eq!(2 + 2, 4);
}
