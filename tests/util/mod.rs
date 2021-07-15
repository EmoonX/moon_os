/*!
*  Utility procedures for testing.
*/

use core::panic::PanicInfo;

use moon_os::panic;

/**
*  Calls test panic handler.
*/
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic::test_handler(info);
}
