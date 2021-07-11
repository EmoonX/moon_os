/*!
 *  Serial port-mapping interface.
 * 
 *  Defines macros to print data from QEMU to outside host console.
 */

use uart_16550::SerialPort;
use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt::Arguments;

/**
 *  Prints to host through the serial interface.
 */
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/**
 *  Prints to host through the serial interface, appending a newline.
 */
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => (
        $crate::serial_print!(concat!($fmt, "\n"))
    );
    ($fmt:expr, $($arg:tt)*) => (
        $crate::serial_print!(concat!($fmt, "\n"), $($arg)*)
    );
}

/// Standard port number for the first serial interface.
const FIRST_SERIAL_PORT: u16 = 0x3F8;

lazy_static! {
    /// Static mutable serial port interface.
    pub static ref SERIAL_1: Mutex<SerialPort> = {
        let mut serial_port =
                unsafe { SerialPort::new(FIRST_SERIAL_PORT) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

/*---------------------------------------------------------------------------*/

/**
 *  Locks serial interface and writes formatted arguments to port.
 */
#[doc(hidden)]
pub fn _print(args: Arguments) {
    use core::fmt::Write;
    SERIAL_1.lock().write_fmt(args)
            .expect("Printing to serial failed...");
}
