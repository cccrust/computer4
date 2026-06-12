#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }

pub fn putchar(c: u8) {
    unsafe { core::arch::asm!("ecall", in("a0") c, in("a7") 1); }
}

pub fn puts(s: &[u8]) {
    unsafe { core::arch::asm!("ecall", in("a0") s.as_ptr(), in("a1") s.len(), in("a7") 2); }
}

pub fn putdec(mut n: u64) {
    let mut buf = [0u8; 20];
    let mut i = 20;
    if n == 0 { putchar(b'0'); return; }
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    puts(&buf[i..]);
}

pub fn exit(code: i32) -> ! {
    unsafe { core::arch::asm!("ecall", in("a0") code, in("a7") 0, options(noreturn)); }
}

macro_rules! fmt {
    () => {};
    ($s:expr) => { puts($s); };
    ($s:expr, $n:expr $(, $rest:expr)*) => {
        puts($s);
        putdec($n);
        fmt!($($rest),*);
    };
}
