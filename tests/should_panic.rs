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
 *  Exits successfully when test does panic.
 */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    qemu::exit(qemu::ExitCode::Success);
    loop {}
}

/**
 *  Runs assertion that should fail and panic.
 */
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(2 + 2, 5);
}

/**
 *  Runs a test that *should* panic.
 */
#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic!]");
    qemu::exit(qemu::ExitCode::Failed);
    loop {}
}
