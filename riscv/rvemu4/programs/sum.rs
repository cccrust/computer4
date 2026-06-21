#![no_std]
#![no_main]

include!("../lib/lib.rs");

fn sum_to(n: u64) -> u64 {
    let mut s = 0;
    let mut i = 1;
    while i <= n { s += i; i += 1; }
    s
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let n = 100;
    fmt!(b"sum(1..", n, b") = ", sum_to(n), b"\n");
    exit(0);
}
