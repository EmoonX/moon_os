/*!
 *  Interrupt handlers for catching CPU exceptions.
 */

use x86_64::structures::idt::{
    InterruptDescriptorTable, InterruptStackFrame
};
use lazy_static::lazy_static;

use crate::gdt;
use crate::println;

lazy_static! {
    /**
     *  Interrupt Descriptor Table (IDT).
     *
     *  Should be available for the complete program runtime
     *  (thus `static`) and set up all exception handlers on init.
     */
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

/**
 *  Loads IDT to CPU.
 */
pub fn init_idt() {
    IDT.load();
}

/**
 *  Breakpoint exception handler.
 *
 *  Triggered by execution of an INT3 instruction.
 */
extern "x86-interrupt" fn breakpoint_handler(
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
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    if crate::test::is_enabled() {
        serial_println!("[ok]");
        qemu::exit(qemu::ExitCode::Success);
        loop {}
    }
    panic!("[EXCEPTION] DOUBLE FAULT\n{:#?}", stack_frame);
}

/*---------------------------------------------------------------------------*/

use crate::serial_println;
use crate::qemu;

/**
 *  Executes an INT3 instruction, thus triggering
 *  a breakpoint exception.
 */
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
