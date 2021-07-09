/**
 *  QEMU's shutdown procedure module.
 *  Uses port-mapping I/O to `isa-debug-exit`.
 */

use x86_64::instructions::port::Port;

#[repr(u32)]
pub enum ExitCode {
    /* Four-byte custom QEMU exit codes. */
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit(exit_code: ExitCode) {
    /* Write exit code to `isa-debug-exit` port,
        thereafter shutting down QEMU. */
    const IOBASE: u16 = 0xf4;
    let mut port = Port::new(IOBASE);
    unsafe {
        port.write(exit_code as u32);
    }
}
