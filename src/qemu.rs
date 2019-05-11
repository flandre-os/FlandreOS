/// QEMU exit with exit status (value << 1) | 1
/// (0x10 << 1) | 1 = 0x101 = 33
/// (0x11 << 1) | 1 = 0x111 = 35
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        // 0xf4 is the iobase of the isa-debug-exit device
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
