/*!
 *  Interrupts handling.
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
 *  Prints `'.'` and notifies EOI, thus enabling Timer interrupt again.
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
 *  Receives key scancode from PS/2 data port and prints
 *  respective key, decoded from [`pc_keyboard`] crate.
 */
extern "x86-interrupt" fn keyboard_handler(
    _stack_frame: InterruptStackFrame)
{
    use x86_64::instructions::port::{Port, ReadWriteAccess};
    use pc_keyboard::{
        layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1
    };

    // Builds port connected to PS2 interface 
    const PS2_DATA_PORT: u16 = 0x60;
    static mut PORT: PortGeneric<u8, ReadWriteAccess> =
        Port::new(PS2_DATA_PORT);
    
    // Defines static `Keyboard` object.
    // Follows US layout PS/2 Set 1. Ctrl keys are ignored.
    lazy_static! {
        static ref KEYBOARD: spin::Mutex<
            Keyboard<layouts::Us104Key, ScancodeSet1>
        > =
            spin::Mutex::new(
                Keyboard::new(layouts::Us104Key,
                    ScancodeSet1, HandleControl::Ignore)
            )
        ;
    }
    // Reads key scancode from port, gets key event from it and
    // processes it afterwards, printing character or key on screen.
    let scancode = unsafe { PORT.read() };
    let mut keyboard = KEYBOARD.lock();
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    // Prints char normally if in ASCII printable range;
                    // otherwise, its code is printed instead.
                    let ascii_code = character as u8;
                    if (0x20..=0x7e).contains(&ascii_code) {
                        println!("{}", character);
                    } else {
                        println!("{:?}", character);
                    }
                }
                DecodedKey::RawKey(key) => {
                    println!("{:?}", key);
                }
            }
        }
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
