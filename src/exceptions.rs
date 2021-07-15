use x86_64::structures::idt::InterruptStackFrame;

use crate::println;
use crate::serial_println;
use crate::qemu;

/**
 *  Breakpoint exception handler.
 *
 *  Triggered by execution of an INT3 instruction.
 */
pub extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    println!("[EXCEPTION] BREAKPOINT\n{:#?}", stack_frame);
}

/**
 *  Double Fault exception handler.
 *
 *  Triggered when an exception occurs when handling
 *  a previous exception.
 */
pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    if crate::test::is_enabled() {
        serial_println!("[ok]");
        qemu::exit(qemu::ExitCode::Success);
    }
    panic!("[EXCEPTION] DOUBLE FAULT\n{:#?}", stack_frame);
}
