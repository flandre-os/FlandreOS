#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(flandre_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::entry_point;
use bootloader::BootInfo;

use flandre_os::hlt_loop;
use flandre_os::println;

entry_point!(entry);

fn entry(_boot_info: &'static BootInfo) -> ! {
    println!("Hello World!");

    #[cfg(test)]
    test_main();

    hlt_loop()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    println!("{}", panic_info);
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    flandre_os::test_panic_handler(panic_info)
}
