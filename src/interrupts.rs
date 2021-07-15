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
use crate::exceptions;
use crate::print;

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

/*---------------------------------------------------------------------------*/

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

/**
 *  PIC interrupt indices, starting by default at 32.
 */
#[repr(u8)]
enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

lazy_static! {
    /**
     *  Interrupt Descriptor Table (IDT).
     *
     *  Should be available for the complete program runtime
     *  (thus `static`) and set up all interrupt handlers on init.
     */
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(exceptions::breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(exceptions::double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer as usize]
            .set_handler_fn(timer_interrupt_handler);
        idt
    };
}

/*---------------------------------------------------------------------------*/

/**
 *  Prints dot and notifies EOI, thus enabling interrupt again.
 */
extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    print!(".");
    unsafe {
        PICS.lock().notify_end_of_interrupt(
            InterruptIndex::Timer as u8);
    }
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
