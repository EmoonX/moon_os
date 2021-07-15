/*!
 *  CPU exceptions handling.
 */

use x86_64::structures::idt::{
    InterruptStackFrame, PageFaultErrorCode
};

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
 *  Triggered when an exception occurs whilst
 *  handling a previous exception.
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

/**
 *  Page Fault exception handler.
 *
 *  Additionally prints fault's accessed address
 *  and specific PF error code.
 */
 pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    pf_error_code: PageFaultErrorCode)
{
    use x86_64::registers::control::Cr2;

    println!("\n[EXCEPTION] PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", pf_error_code);
    panic!("{:#?}", stack_frame);
}


/*---------------------------------------------------------------------------*/

/**
 *  Executes an INT3 instruction, thus triggering
 *  a breakpoint exception.
 */
 #[test_case]
 fn test_breakpoint_exception() {
     x86_64::instructions::interrupts::int3();
 }
