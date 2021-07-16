/*!
 *  Global Descriptor Table (GDT).
 *
 *  Particularly handles Task State Segment (TSS) with a Interrupt
 *  Descriptor Table (IDT) stack, which is swiched to on exceptions.
 */

use x86_64::{
    structures::{
        gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector},
        tss::TaskStateSegment,
    },
    instructions::{
        segmentation::set_cs, tables::load_tss
    },
    VirtAddr,
};
use lazy_static::lazy_static;

/**
 *  Inits GDT table + sets/loads segments.
 */
pub fn init() {
    GDT.gdt.load();
    unsafe {
        set_cs(GDT.code_selector);
        load_tss(GDT.tss_selector);
    }
}

/// Double Fault stack index at IST table. 
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    /**
     *  Global Descriptor Table (GDT);
     *
     *  Contains entries for both kernel code and TSS segments.
     */
    static ref GDT: _GDT = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(
            Descriptor::kernel_code_segment()
        );
        let tss_selector = gdt.add_entry(
            Descriptor::tss_segment(&TSS)
        );
        _GDT { gdt, code_selector, tss_selector }
    };

    /**
     *  Task State Segment (TSS) table.
     *
     *  Contains Interrupt Stack Table (IST) structure.
     */
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            // Creates a 20 KB stack in a static memory location.
            // Converts pointers to `VirtAddr` and returns stack top address.
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let ptr = unsafe { &STACK };
            let stack_start = VirtAddr::from_ptr(ptr);
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

/**
 *  Wrapper structure for GDT and its segment selectors.
 */
struct _GDT {
    /// The GDT itself.
    gdt: GlobalDescriptorTable,

    /// Kernel code segment selector.
    code_selector: SegmentSelector,

    /// TSS segment selector.
    tss_selector: SegmentSelector,
}
