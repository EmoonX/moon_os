/**
 *  Panic (fatal error) module.
 *  Implements debug and error treatment funcions.
 */

use core::panic::PanicInfo;
use crate::println;
use crate::serial_println;
use crate::qemu;

#[cfg(not(test))]  // compiles only in `cargo run`
#[panic_handler]   // function called on panic
fn panic(info: &PanicInfo) -> ! {
    /* Just print panic message to VGA buffer. */
    println!("{}", info);
    loop {}
}

#[cfg(test)]  // compiles only in `cargo test`
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    /* Print panic message to host console
        and shutdown QEMU with a failed exit code. */
    serial_println!("[failed]");
    serial_println!("Error: {}", info);
    qemu::exit(qemu::ExitCode::Failed);
    loop {}
}
