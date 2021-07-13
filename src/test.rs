/*!
 *  Automated tests, both unit and integration.
 */

use core::any::type_name;

use crate::serial_print;
use crate::serial_println;
use crate::qemu;

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
