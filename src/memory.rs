/*!
 *  Memory paging management.
 */

use x86_64::{
    structures::paging::PageTable,
    VirtAddr
};

/**
 *  Gets active level 4 table (P4) page reference.
 *
 *  This is done by applying `physical_memory_offset` to P4 frame's
 *  _physical_ address (contained on CPU's c3 register), thus getting
 *  the respective P4 page's _virtual_ address.
 */
pub unsafe fn get_active_p4_page_table(
    physical_memory_offset: u64) -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    // Gets level 4 table frame (from cr3 register) and its physical address
    let (p4_frame, _) = Cr3::read();
    let p4_phys_addr = p4_frame.start_address();

    // Applies offset to get page's virtual address from frame's physical one
    let addr = p4_phys_addr.as_u64() + physical_memory_offset;
    let p4_virt_addr = VirtAddr::new(addr);

    // Gets mutable `PageTable` pointer and converts it to a reference
    let p4_page_table_ptr = p4_virt_addr.as_mut_ptr();
    let p4_page_table = &mut *p4_page_table_ptr;

    p4_page_table
}
