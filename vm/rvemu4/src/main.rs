#![allow(dead_code, unused)]

mod memory;
mod cpu;
mod elf;

use std::env;
use memory::{Bus, RAM_BASE, RAM_SIZE};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    let mut kernel_path = None::<String>;
    let mut disk_path = None::<String>;
    let mut smp = 1;
    let mut standalone_path = None::<String>;
    while i < args.len() {
        match args[i].as_str() {
            "-kernel" => { i += 1; if i < args.len() { kernel_path = Some(args[i].clone()); } }
            "-drive" => { i += 1; if i < args.len() { disk_path = Some(args[i].clone()); } }
            "-smp" => { i += 1; if i < args.len() { smp = args[i].parse().unwrap_or(1); } }
            s if s.starts_with('-') => {}
            _ => { standalone_path = Some(args[i].clone()); }
        }
        i += 1;
    }

    if kernel_path.is_none() && standalone_path.is_none() {
        eprintln!("Usage: rvemu4 [-kernel <kernel.elf> [-drive <disk.img>]] [<program.elf>]");
        std::process::exit(1);
    }

    let mut bus = Bus::new(RAM_SIZE as usize);

    if let Some(ref kp) = kernel_path {
        let result = elf::load_elf(kp, &mut bus);
        if let Some(dp) = disk_path { bus.vblk.init(&dp); }
        let smp = smp.max(1).min(8);
        let mut harts: Vec<cpu::Hart> = (0..smp as u32).map(|id| {
            let mut h = cpu::Hart::new(id);
            h.reset(result.entry, result.is_64);
            h
        }).collect();

        set_raw_mode(true);

        let mut step_count: u64 = 0;
        loop {
            for h in harts.iter_mut() { h.step(&mut bus); step_count += 1; }
            if step_count % 4096 == 0 {
                check_stdin(&mut bus);
                if bus.uart_needs_pending() || (bus.uart_rx_ready() && (bus.uart_ier() & 1) != 0) {
                    bus.plic.set_pending(10);
                    bus.uart_clear_pending_tx_irq();
                }
            }
        }
    } else {
        let sp = standalone_path.unwrap();
        let result = elf::load_elf(&sp, &mut bus);
        let mut hart = cpu::Hart::new(0);
        hart.reset(result.entry, result.is_64);
        hart.offset = RAM_BASE;
        hart.x[2] = (RAM_SIZE - 0x1000) as u64; // sp (top of guest address space)
        loop {
            hart.step(&mut bus);
            if hart.halt { break; }
        }
        std::process::exit(hart.exit_code);
    }
}

fn check_stdin(bus: &mut Bus) {
    use std::io::Read;
    use std::os::fd::AsRawFd;
    use std::sync::atomic::{AtomicBool, Ordering};
    static CTRL_A: AtomicBool = AtomicBool::new(false);
    let fd = std::io::stdin().as_raw_fd();
    let mut fds: libc::fd_set = unsafe { std::mem::zeroed() };
    unsafe { libc::FD_ZERO(&mut fds); }
    unsafe { libc::FD_SET(fd, &mut fds); }
    let mut tv = libc::timeval { tv_sec: 0, tv_usec: 0 };
    let ret = unsafe { libc::select(fd + 1, &mut fds, std::ptr::null_mut(), std::ptr::null_mut(), &mut tv) };
    if ret > 0 {
        let mut buf = [0u8; 1];
        if std::io::stdin().read(&mut buf).is_ok() && buf[0] != 0 {
            if CTRL_A.load(Ordering::Relaxed) {
                CTRL_A.store(false, Ordering::Relaxed);
                if buf[0] == b'x' || buf[0] == b'X' {
                    set_raw_mode(false);
                    std::process::exit(0);
                }
            } else if buf[0] == 0x01 {
                CTRL_A.store(true, Ordering::Relaxed);
            } else {
                bus.push_uart_rx(buf[0]);
            }
        }
    }
}

fn set_raw_mode(en: bool) {
    if !en { return; }
    use std::os::fd::AsRawFd;
    let fd = std::io::stdin().as_raw_fd();
    if unsafe { libc::isatty(fd) } == 0 { return; }
    unsafe {
        let mut term: libc::termios = std::mem::zeroed();
        libc::tcgetattr(fd, &mut term);
        term.c_iflag &= !(libc::BRKINT | libc::ICRNL | libc::INPCK | libc::ISTRIP | libc::IXON);
        term.c_oflag &= !libc::OPOST;
        term.c_cflag |= libc::CS8;
        term.c_lflag &= !(libc::ECHO | libc::ICANON | libc::IEXTEN | libc::ISIG);
        term.c_cc[libc::VMIN] = 0;
        term.c_cc[libc::VTIME] = 0;
        libc::tcsetattr(fd, libc::TCSAFLUSH, &term);
    }
}
