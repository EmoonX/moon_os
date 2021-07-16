#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(moon_os::test::runner)]
#![reexport_test_harness_main = "test_main"]

/*!
 *  Stack overflow test.
 *  
 *  Tests if Double Fault handler is working correctly.
 */

mod panic;

use moon_os::serial_print;

/**
 *  Causes a stack oveflow by recursing endlessly.
 *
 *  Volatile read is used to block compiler's tail recursion
 *  optimization, thus avoiding transforming calls into a loop.
 */
#[allow(unconditional_recursion)]  // ignores stack overflow warnings
fn stack_overflow() {
    use volatile::Volatile;

    stack_overflow();
    Volatile::new(0).read();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");
    moon_os::init(true);
    panic::set_success_on_panic();
    stack_overflow();
    panic::failed_without_panic();
}
