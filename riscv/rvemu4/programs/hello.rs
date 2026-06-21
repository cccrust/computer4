#![no_std]
#![no_main]

include!("../lib/lib.rs");

#[no_mangle]
pub extern "C" fn _start() -> ! {
    puts(b"Hello, World!\n");
    exit(0);
}
