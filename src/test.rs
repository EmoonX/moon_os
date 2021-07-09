/**
 *  Automated unit tests module.
 */

use crate::print;
use crate::println;
use crate::qemu;

#[cfg(test)]  // includes test function only for testing
pub fn runner(tests: &[&dyn Fn()]) {
    /* Iterate test functions and run them. */
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    qemu::exit(qemu::ExitCode::Success);
}

#[test_case]  // defines a test case function
fn trivial_assertion() {
    /* Simply trivial assertion that is always true. */
    print!("Trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
