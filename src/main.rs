#![no_std]		// don't link the Rust standard library
#![no_main]		// disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

/// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]	// don't mangle the name of the function
pub extern "C" fn _start() -> ! {
	vga_buffer::print_something();
	
	// this function is the entry point, since the linker looks for a function
	// named `_start` by default
	loop {}
}