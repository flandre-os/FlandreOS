#![no_std]
#![no_main]

use bootloader::entry_point;
use bootloader::BootInfo;
use core::panic::PanicInfo;
use flandre_os::hlt_loop;
use flandre_os::println;

entry_point!(entry);

fn entry(_boot_info: &'static BootInfo) -> ! {
    let user_name = "lws";
    println!("Hello, {}!", user_name);

    hlt_loop()
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    println!("{}", panic_info);
    hlt_loop()
}
