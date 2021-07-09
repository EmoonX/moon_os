/**
 *  Automated unit tests module.
 */

use crate::serial_print;
use crate::serial_println;
use crate::qemu;

#[cfg(test)]  // includes test function only for testing
pub fn runner(tests: &[&dyn Fn()]) {
    /* Iterate test functions and run them. */
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    qemu::exit(qemu::ExitCode::Success);
}

#[test_case]  // defines a test case function
fn trivial_assertion() {
    /* Simply trivial assertion that is always true. */
    serial_print!("Trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}
