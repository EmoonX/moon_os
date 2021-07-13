/*!
 *  Interrupt handlers for catching CPU exceptions.
 */

use x86_64::structures::idt::{
    InterruptDescriptorTable, InterruptStackFrame
};
use lazy_static::lazy_static;

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
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

/**
 *  Executes an INT3 instruction, thus triggering
 *  a breakpoint exception.
 */
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
