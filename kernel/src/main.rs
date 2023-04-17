#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};
use core::arch::asm;
use core::panic::PanicInfo;

mod vga_buffer;

entry_point!(kernel_main);

/// https://github.com/rust-osdev/bootloader
fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    todo!();
}

// TODO: Fix duplicate panic_handler.
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("hlt", options(noreturn)) }
    }
}
