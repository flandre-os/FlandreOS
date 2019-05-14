#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(flandre_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::entry_point;
use bootloader::BootInfo;

use flandre_os::hlt_loop;
use flandre_os::{eprint, eprintln, print, println};

entry_point!(entry);

#[allow(clippy::print_with_newline)]
fn entry(_boot_info: &'static BootInfo) -> ! {
    print!("Hello World!\n");
    print!("New line test...\r\n");
    let name = "lws";
    println!("Hello {}!", name);

    for i in 1..=20 {
        println!("{}", i);
    }

    eprintln!("FlandreOS is a toy operating system for personal learning operating system related knowledge.");
    eprint!("Please enjoy it!");

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
