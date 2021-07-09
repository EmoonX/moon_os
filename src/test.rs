use crate::print;
use crate::println;

#[cfg(test)]  // includes test function only for testing
pub fn test_runner(tests: &[&dyn Fn()]) {
    /* Iterate test functions and run them. */
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]  // defines a test case function
fn trivial_assertion() {
    print!("Trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
