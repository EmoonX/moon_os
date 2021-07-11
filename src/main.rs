#![no_std]   // don't link Rust's stdlib
#![no_main]  // disable all Rust-level entry points
#![feature(custom_test_frameworks)]           // replaces std test framework
#![test_runner(moon_os::test::runner)]        // defines test runner function
#![reexport_test_harness_main = "test_main"]  // replaces entry fn in testing

/**
 *  Main moon_os project module.
 */

use core::panic::PanicInfo;

use moon_os::println;

#[cfg(not(test))]  // compiles only in `cargo run`
#[panic_handler]   // function called on panic
fn panic(info: &PanicInfo) -> ! {
    /* Print panic message to host console
        and shutdown QEMU with a failed exit code. */
    use moon_os::panic;
    panic::handler(info);
}

#[no_mangle]  // don't mangle function's name
pub extern "C" fn _start() -> ! {
    /* Entry point; linker looks for `_start` by default. */
    
    println!("Oi!");
    println!("Hello world {} {} {} {}", 1, 2, 3, '!');
    panic!("Some panic message");

    #[cfg(test)]
    test_main();

    loop {}
}

/*---------------------------------------------------------------------------*/

#[cfg(test)]  // compiles only in `cargo test`
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    /* Call test panic handler. */
    use moon_os::panic;
    panic::test_handler(info);
}

#[test_case]  // defines a test case function
fn trivial_assertion() {
    /* Simply trivial assertion that is always true. */
    assert_eq!(2 + 2, 4);
}
