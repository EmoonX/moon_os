/*!
 *  Panic treatment and debug.
 */

use core::panic::PanicInfo;

use crate::println;
use crate::serial_println;
use crate::qemu;

/**
 *  Prints panic message to VGA buffer.
 */
pub fn handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    crate::hlt_loop();
}

/**
 *  Prints panic message to host console
 *  and exits QEMU with failed exit code.
 */
pub fn test_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("Error: {}", info);
    qemu::exit(qemu::ExitCode::Failed);
}
