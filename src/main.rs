#![no_std]
#![no_main]

// We are not using the standard library or main function
use core::panic::PanicInfo;

// The entry point for the kernel.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Simple output
    kernel_main();
    
    // Endless loop after kernel has initialized
    loop {}
}

// Minimalist kernel functionality
fn kernel_main() {
    // For now, we’ll print something simple
    // Later we’ll add memory management and process handling
    println("Hello from Microkernel!");
}

// Handle panics (no_std requires this)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
