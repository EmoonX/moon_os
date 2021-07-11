#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(panic_test_runner)]
#![reexport_test_harness_main = "test_main"]

/*!
 *  Tests that should panic.
 */

use core::panic::PanicInfo;

use moon_os::{serial_print, serial_println};
use moon_os::qemu;

/**
 *  Runs tests that *should* panic.
 */
pub fn panic_test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    if tests.is_empty() {
        // Nothing to do if no tests defined
        qemu::exit(qemu::ExitCode::Success);
    }
    let test = tests[0];
    test();
    serial_println!("[test did not panic!]");
    qemu::exit(qemu::ExitCode::Failed);
}

/**
 *  Runs assertion that should fail and panic.
 */
#[test_case]
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(1, 1);
}

/**
 *  Exits successfully when test does panic.
 */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    qemu::exit(qemu::ExitCode::Success);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}
