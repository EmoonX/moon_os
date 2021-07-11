/**
 *  Panic (fatal error) module.
 *  Implements debug and error treatment funcions.
 */

use core::panic::PanicInfo;

use crate::println;
use crate::serial_println;
use crate::qemu;

pub fn handler(info: &PanicInfo) -> ! {
    /* Just print panic message to VGA buffer. */
    println!("{}", info);
    loop {}
}

pub fn test_handler(info: &PanicInfo) -> ! {
    /* Print panic message to host console
        and shutdown QEMU with a failed exit code. */
    serial_println!("[failed]");
    serial_println!("Error: {}", info);
    qemu::exit(qemu::ExitCode::Failed);
    loop {}
}
