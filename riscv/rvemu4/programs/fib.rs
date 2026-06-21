#![no_std]
#![no_main]

include!("../lib/lib.rs");

fn fib(n: u64) -> u64 {
    if n <= 1 { return n; }
    let mut a = 0u64;
    let mut b = 1u64;
    let mut i = 2;
    while i <= n {
        let t = a + b;
        a = b;
        b = t;
        i += 1;
    }
    b
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let n = 20;
    fmt!(b"fibonacci(", n, b") = ", fib(n), b"\n");
    exit(0);
}
