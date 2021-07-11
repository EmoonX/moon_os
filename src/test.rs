/**
 *  Automated unit tests module.
 */

use core::any::type_name;
use crate::serial_print;
use crate::serial_println;
use crate::qemu;

pub fn runner(tests: &[&dyn Testable]) {
    /* Iterate test functions and run them.
        Exit thereafter with successful exit code. */
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit(qemu::ExitCode::Success);
}

/*---------------------------------------------------------------------------*/

pub trait Testable {
    /* Trait to be given to testing functions. */
    fn run(&self) -> ();
}

impl<T> Testable for T where T: Fn() {
    fn run(&self) {
        /* Wrapper runner function. Prints function name,
            run test and then prints [ok] if successful. */
        serial_print!("{}...\t", type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}
