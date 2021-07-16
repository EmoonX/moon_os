/*!
*  Panic handling for testing.
*/

use core::panic::PanicInfo;

use moon_os::{serial_println, panic};
use moon_os::qemu;

/**
 *  Makes panic handler exit successfully when a panic occurs.
 */
#[allow(dead_code)]
pub fn set_success_on_panic() {
    unsafe { SUCCESS_ON_PANIC = true }
}

/**
 *  Prints a message so acknowledge absence of panic and
 *  exits QEMU with a failed exit code.
 */
#[allow(dead_code)]
pub fn failed_without_panic() -> ! {
    serial_println!("[test did not panic!]");
    qemu::exit(qemu::ExitCode::Failed);
}

/// If respective test is considered successful on panic.
static mut SUCCESS_ON_PANIC: bool = false;

/**
*  Calls test panic handler.
*/
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if unsafe { SUCCESS_ON_PANIC } {
        serial_println!("[ok]");
        qemu::exit(qemu::ExitCode::Success);
    }
    panic::test_handler(info);
}
