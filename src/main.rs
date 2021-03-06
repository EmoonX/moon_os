#![no_std]   // don't link Rust's stdlib
#![no_main]  // disable all Rust-level entry points
#![feature(core_intrinsics)]
#![feature(custom_test_frameworks)]           // replaces std test framework
#![test_runner(moon_os::test::runner)]        // defines test runner function
#![reexport_test_harness_main = "test_main"]  // replaces entry fn in testing

/*!
 *  Main project module.
 */

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

use moon_os::{println, panic};

// Defines `kernel_main` as the executable entry point.
// This guarantees the correct arguments are passed to it.
entry_point!(kernel_main);

/**
 *  Calls normal panic handler.
 */
#[cfg(not(test))]  // compiles only in `cargo run`
#[panic_handler]   // function called on panic
fn panic(info: &PanicInfo) -> ! {
    panic::handler(info);
}

/**
 *  Entry point for `cargo run`.
 */
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use moon_os::memory;

    println!("Oi!");
    println!("Hello world {} {} {} {}", 1, 2, 3, '!');
    // panic!("Some panic message");

    moon_os::init(false);

    let offset = boot_info.physical_memory_offset;
    let p4_page_table = unsafe { 
        memory::get_active_p4_page_table(offset)
    };
    for (i, entry) in p4_page_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("P4 Entry #{}:\n{:#?}", i, entry);
        }
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    moon_os::hlt_loop();
}

/*---------------------------------------------------------------------------*/

/**
 *  Calls test panic handler.
 */
#[cfg(test)]  // compiles only in `cargo test`
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic::test_handler(info);
}

/**
 *  Simply trivial assertion test that is always true.
 */
#[test_case]  // defines a test case function
fn trivial_assertion() {
    assert_eq!(2 + 2, 4);
}
