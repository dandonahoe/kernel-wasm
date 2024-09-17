#![no_std]
#![no_main]

mod vga_buffer;  // For screen output

use core::panic::PanicInfo;

// Entry point for the kernel
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello from Microkernel!");

    loop {}
}

// Panic handler for no_std environment
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
