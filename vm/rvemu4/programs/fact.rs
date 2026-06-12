#![no_std]
#![no_main]

include!("../lib/lib.rs");

fn fact(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * fact(n - 1) }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let n = 10;
    fmt!(b"fact(", n, b") = ", fact(n), b"\n");
    exit(0);
}
