#![no_std]
#![no_main]

use bootloader::entry_point;
use bootloader::BootInfo;
use core::panic::PanicInfo;
use flandre_os::println;

entry_point!(entry);

fn entry(boot_info: &'static BootInfo) -> ! {
    println!("{:?}\n", boot_info);

    let user_name = "lws";
    println!("Hello, {}!", user_name);

    hlt();
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    println!("{}", panic_info);
    hlt();
}

fn hlt() -> ! {
    x86_64::instructions::hlt();
    #[allow(clippy::empty_loop)]
    loop {}
}
