#![no_std]
#![cfg_attr(test, no_main)]     // enable `no_main` when `cargo_test`
#![feature(abi_x86_interrupt)]  // enable use of `extern "x86-interrupt"`
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]

/*!
 *  Minimal and personal OS project based on `blog_os`. Made in Rust.
 */

pub mod vga_buffer;
pub mod serial;
pub mod qemu;
pub mod interrupts;
pub mod memory;
pub mod panic;
pub mod test;

mod exceptions;
mod gdt;

/**
 *  Calls OS initialization routines.
 */
pub fn init(is_test: bool) {
    unsafe { test::ENABLED =  is_test };
    interrupts::init();
}

/**
 *  Uses `hlt` instruction to halt CPU, thus avoiding
 *  running an endless loop at full speed.
 */
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/*---------------------------------------------------------------------------*/

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

// Defines `test_kernel_main` as the testing entry point.
#[cfg(test)]
entry_point!(test_kernel_main);

/**
 *  Calls test panic handler.
 */
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic::test_handler(info);
}

/**
 *  Entry point for `cargo test`.
 */
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init(true);
    test_main();
    hlt_loop();
}
