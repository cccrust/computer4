mod memory;
mod cpu;
mod elf;

use std::env;
use memory::{Bus, RAM_SIZE};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut kernel_path = None;
    let mut disk_path = None;
    let mut smp = 1;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-kernel" => { i += 1; kernel_path = Some(args[i].clone()); }
            "-drive" => { i += 1; disk_path = Some(args[i].clone()); }
            "-smp" => { i += 1; smp = args[i].parse().unwrap_or(1); }
            _ => {}
        }
        i += 1;
    }

    let kernel_path = kernel_path.expect("Usage: rvemu4 -kernel <kernel.elf> [-drive <disk.img>] [-smp <n>]");

    let mut bus = Bus::new(RAM_SIZE as usize);
    let (entry, is_64) = elf::load_elf(&kernel_path, &mut bus);

    if let Some(dp) = disk_path {
        bus.vblk.init(&dp);
    }

    let smp = smp.max(1).min(8);
    let mut harts: Vec<cpu::Hart> = (0..smp as u32).map(|id| {
        let mut h = cpu::Hart::new(id);
        h.reset(entry, is_64);
        h
    }).collect();

    set_raw_mode(true);

    let mut poll_count: u64 = 0;
    let mut step_count: u64 = 0;
    let mut did_sched_trace = false;
    // eprintln!("Starting main loop, entry={:#x}, is_64={}, smp={}", entry, is_64, smp);
    loop {
        for h in harts.iter_mut() {
            h.check_interrupts(&mut bus);
            h.step(&mut bus);
            step_count += 1;
        }

        poll_count += 1;
        if poll_count % 1000000 == 0 {
            let pc = harts[0].pc;
            if (pc >= 0x80001d16 && pc <= 0x80001dc6) && !did_sched_trace {
                did_sched_trace = true;
                let s1 = harts[0].x[9];
                let proc0_state = 0x8000FDA0u64;
                let state_val = bus.mmio_read(proc0_state, 4);
                let state_val2 = bus.mmio_read(s1.wrapping_add(24), 4);
                eprintln!("SCHED_TRACE: steps={}K pc={:#x} x1={:#x} x9={:#x} x10={:#x} x2(sp)={:#x} satp={:#x} priv={} mstatus={:#x} sstatus.SIE={}", step_count/1000, pc, harts[0].x[1], s1, harts[0].x[10], harts[0].x[2], harts[0].satp, harts[0].priv_level, harts[0].mstatus, (harts[0].mstatus>>1)&1);
                eprintln!("SCHED_TRACE: proc[0].state at {:#x} = {} (RUNNABLE=3)", proc0_state, state_val);
                eprintln!("SCHED_TRACE: p->state at s1+24 ({:#x}) = {} (RUNNABLE=3)", s1.wrapping_add(24), state_val2);
            }
            eprintln!("PROGRESS steps={}K pc={:#x}", step_count / 1000, pc);
        }
        if poll_count % 4096 == 0 {
            check_stdin(&mut bus);
            if bus.uart_needs_pending() || (bus.uart_rx_ready() && (bus.uart_ier() & 1) != 0) {
                bus.plic.set_pending(10);
                bus.uart_clear_pending_tx_irq();
            }
        }
    }
}

fn check_stdin(bus: &mut Bus) {
    use std::io::Read;
    use std::os::fd::AsRawFd;
    let fd = std::io::stdin().as_raw_fd();
    let mut fds: libc::fd_set = unsafe { std::mem::zeroed() };
    unsafe { libc::FD_ZERO(&mut fds); }
    unsafe { libc::FD_SET(fd, &mut fds); }
    let mut tv = libc::timeval { tv_sec: 0, tv_usec: 0 };
    let ret = unsafe { libc::select(fd + 1, &mut fds, std::ptr::null_mut(), std::ptr::null_mut(), &mut tv) };
    if ret > 0 {
        let mut buf = [0u8; 1];
        if std::io::stdin().read(&mut buf).is_ok() && buf[0] != 0 {
            bus.push_uart_rx(buf[0]);
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
