/*!
 *  Interrupts and CPU exceptions.
 */

use x86_64::instructions::port::PortGeneric;
use x86_64::structures::idt::{
    InterruptDescriptorTable, InterruptStackFrame
};
use x86_64::instructions::interrupts;
use pic8259::ChainedPics;
use lazy_static::lazy_static;

use crate::gdt;
use crate::exceptions;
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
    Keyboard,
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

        // Exception handlers
        idt.breakpoint.set_handler_fn(exceptions::breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(exceptions::double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        // Normal interrupt handlers
        idt[InterruptIndex::Timer as usize]
            .set_handler_fn(timer_handler);
        idt[InterruptIndex::Keyboard as usize]
            .set_handler_fn(keyboard_handler);
        
        idt
    };
}

/*---------------------------------------------------------------------------*/

/**
 *  Timer interrupt handler, called on each timer tick.
 *
 *  Prints dot and notifies EOI, thus enabling interrupt again.
 */
extern "x86-interrupt" fn timer_handler(
    _stack_frame: InterruptStackFrame)
{
    //print!(".");
    unsafe {
        PICS.lock().notify_end_of_interrupt(
            InterruptIndex::Timer as u8);
    }
}

/**
 *  Keyboard interrupt handler, called on key presses.
 *
 *  Gets key scancode from PS2 data port and prints it.
 */
extern "x86-interrupt" fn keyboard_handler(
    _stack_frame: InterruptStackFrame)
{
    use x86_64::instructions::port::{Port, ReadWriteAccess};

    // Builds port connected to PS2 interface 
    const PS2_DATA_PORT: u16 = 0x60;
    static mut PORT: PortGeneric<u8, ReadWriteAccess> =
        Port::new(PS2_DATA_PORT);
    
    // Reads key scancode from port and gets respective key name
    let scancode = unsafe { PORT.read() };
    let key = match scancode {
        0x02 => "1",
        0x03 => "2",
        0x04 => "3",
        0x05 => "4",
        0x06 => "5",
        0x07 => "6",
        0x08 => "7",
        0x09 => "8",
        0x0a => "9",
        0x0b => "0",
        _ => "None",
    };
    // If key is a valid mapped one, prints it
    if key != "None" {
        println!("{}", key);
    }

    // Notifies EOI for re-enabling key presses
    unsafe {
        PICS.lock().notify_end_of_interrupt(
            InterruptIndex::Keyboard as u8);
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
