/*!
 *  QEMU's shutdown procedures.
 * 
 *  Uses port-mapping I/O to `isa-debug-exit`.
 */

use x86_64::instructions::port::Port;

/**
 *  Four-byte custom QEMU exit code.
 */
#[repr(u32)]
pub enum ExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/** 
 *  Writes exit code to `isa-debug-exit` port,
 *  thereafter shutting down QEMU.
 */
pub fn exit(exit_code: ExitCode) -> ! {
    const IOBASE: u16 = 0xf4;
    let mut port = Port::new(IOBASE);
    unsafe {
        port.write(exit_code as u32);
    }
    loop {}
}
