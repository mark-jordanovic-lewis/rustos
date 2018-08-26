// Basic standalone binary with no std lib linkage
#![feature(panic_implementation)]
#![no_std]
#![no_main]


// Handle System Panics
// - this needs to be more intelligent in a real system
// - has no support for stack unwinding (would be helpful tbh)
use core::panic::PanicInfo;
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! { // ! is the never type, applies to fns with inf loops...
    loop {}
}

// Handle RunTime loading
#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}