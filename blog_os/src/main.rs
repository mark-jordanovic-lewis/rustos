#![feature(panic_implementation)]
#![no_std]
#![no_main]

mod vga_buffer;

#[macro_use]
extern crate lazy_static;
extern crate bootloader_precompiled;
extern crate volatile;
extern crate spin;

// Handle System Panics
// - this needs to be more intelligent in a real system
use core::panic::PanicInfo;
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! { // ! is the never type, applies to fns with inf loops...
    loop {}
}

static HELLO:&[u8] = b"Hello World!";

// Handle RunTime loading
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // cast integer to raw pointer
    let vga_buffer = 0xb8000 as *mut u8;
    // iterate over byte string
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;    // string byte
            *vga_buffer.offset(i as isize * 2 + 1) = 0x7; // color byte (cyan)
        }
    }

    vga_buffer::print_something();



    loop {}
}