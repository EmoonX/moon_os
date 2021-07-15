/*!
 *  Interrupts and CPU exceptions.
 */

use x86_64::structures::idt::{
    InterruptDescriptorTable, InterruptStackFrame
};
use x86_64::instructions::interrupts;
use pic8259::ChainedPics;
use lazy_static::lazy_static;

use crate::gdt;
use crate::println;

/**
 *  Loading and initialization procedures.
 */
 pub fn init() {
    // Loads IDT and set it on GDT's TSS
    IDT.load();
    gdt::init();

    // Initializes PICs and enable CPU listening to interrupts
    unsafe { PICS.lock().initialize() };
    interrupts::enable();
}

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

/// Primary PIC's vector number offset
const PIC_1_OFFSET: u8 = 32;

/// Secondary PIC's vector number offset
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

/**
 *  Programmable Interrupt Controllers (PICs) in chained x86 structure.
 */
pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(
    unsafe {
        ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
    }
);

/*---------------------------------------------------------------------------*/

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
