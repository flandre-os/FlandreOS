#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(flandre_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::entry_point;
use bootloader::BootInfo;
use core::panic::PanicInfo;
use flandre_os::{hlt_loop, println, serial_print, serial_println};

entry_point!(entry);

fn entry(_boot_info: &'static BootInfo) -> ! {
    test_main();

    hlt_loop()
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    flandre_os::test_panic_handler(panic_info)
}

#[test_case]
fn test_basic_boot_println() {
    serial_print!("test_basic_boot_println... ");
    println!("test_basic_boot_println output");
    serial_println!("[ok]");
}
