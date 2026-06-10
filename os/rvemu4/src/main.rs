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
    loop {
        for h in harts.iter_mut() {
            h.check_interrupts(&mut bus);
            h.step(&mut bus);
        }

        poll_count += 1;
        if poll_count % 256 == 0 {
            check_stdin(&mut bus);
            if bus.uart_has_irq() {
                bus.plic.set_pending(10);
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
