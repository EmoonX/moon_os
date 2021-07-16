/*!
 *  Automated tests, both unit and integration.
 */

use core::any::type_name;

use crate::serial_print;
use crate::serial_println;
use crate::qemu;

/**
 *  Enables (additional) testing procedures.
 */
 pub fn enable() {
    unsafe { ENABLED = true };
}

/**
 *  Returns if test is enabled (that is, `cargo test` is being run).
 */
 pub fn is_enabled() -> bool {
    unsafe { ENABLED }
}

/** 
 *  Iterates test functions and runs them.
 * 
 *  Exits thereafter with a successful exit code.
 */
pub fn runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit(qemu::ExitCode::Success);
}

/*---------------------------------------------------------------------------*/

/// Globally indicates whether testing is being run or not.
static mut ENABLED: bool = false;

/**
 *  Trait to be given to testing functions.
 */
pub trait Testable {
    /** 
     *  Wrapper runner function. Prints function name,
     *  runs test and then prints `[ok]` if successful.
     */
    fn run(&self) -> ();
}

impl<T> Testable for T where T: Fn() {
    fn run(&self) {
        serial_print!("{}...\t", type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}
