use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

pub const RAM_BASE: u64 = 0x80000000;
pub const RAM_SIZE: u64 = 128 * 1024 * 1024;
const UART_BASE: u64 = 0x10000000;
const PLIC_BASE: u64 = 0x0C000000;
const PLIC_SIZE: u64 = 0x400000;
const VIRTIO0_BASE: u64 = 0x10001000;
const VIRTIO_SIZE: u64 = 0x1000;
const RTC_BASE: u64 = 0x00101000;
const CLINT_BASE: u64 = 0x02000000;
const CLINT_SIZE: u64 = 0x10000;

pub struct Uart {
    ier: u8, lcr: u8,
    rx_buf: [u8; 256], rx_head: usize, rx_tail: usize,
    tx_pending: bool, pending_tx_irq: bool,
}
impl Uart {
    pub fn new() -> Self {
        Self { ier: 0, lcr: 0, rx_buf: [0; 256], rx_head: 0, rx_tail: 0, tx_pending: false, pending_tx_irq: false }
    }
    pub fn push_rx(&mut self, c: u8) {
        let n = (self.rx_tail + 1) % 256;
        if n != self.rx_head { self.rx_buf[self.rx_tail] = c; self.rx_tail = n; }
    }
    pub fn rx_ready(&self) -> bool { self.rx_head != self.rx_tail }
    pub fn has_irq(&self) -> bool {
        (self.rx_ready() && (self.ier & 1) != 0) || (self.tx_pending && (self.ier & 2) != 0)
    }
    pub fn needs_pending(&self) -> bool { self.pending_tx_irq }
    pub fn clear_pending_tx_irq(&mut self) { self.pending_tx_irq = false; }
    pub fn read(&mut self, off: u64) -> u8 {
        match off {
            0 => {
                if self.rx_head != self.rx_tail {
                    let c = self.rx_buf[self.rx_head];
                    self.rx_head = (self.rx_head + 1) % 256;
                    return c;
                }
                0
            }
            1 => self.ier,
            2 => {
                if self.rx_ready() && (self.ier & 1) != 0 { 4 }
                else if self.tx_pending && (self.ier & 2) != 0 { self.tx_pending = false; self.pending_tx_irq = false; 2 }
                else { 1 }
            }
            3 => self.lcr,
            5 => (if self.rx_ready() { 1 } else { 0 }) | 0x20 | 0x40,
            _ => 0,
        }
    }
    pub fn write(&mut self, off: u64, val: u8) {
        match off {
            0 => {
                if self.lcr & 0x80 != 0 { return; }
                print!("{}", val as char);
                use std::io::Write;
                std::io::stdout().flush().ok();
                self.tx_pending = true;
                if (self.ier & 2) != 0 { self.pending_tx_irq = true; }
            }
            1 => self.ier = val,
            3 => self.lcr = val,
            _ => {}
        }
    }
}

pub struct Plic {
    priority: [u32; 32], pub pending: u32,
    senable: [u32; 8], spriority: [u32; 8],
}
impl Plic {
    pub fn new() -> Self { Self { priority: [0; 32], pending: 0, senable: [0; 8], spriority: [0; 8] } }
    pub fn set_pending(&mut self, irq: u32) {
        use std::sync::atomic::{AtomicU64, Ordering};
        static PEND_N: AtomicU64 = AtomicU64::new(0);
        let n = PEND_N.fetch_add(1, Ordering::Relaxed);
        
        self.pending |= 1 << irq;
    }
    pub fn has_irq(&self, hart: usize) -> bool {
        let en = self.senable[hart];
        let pend = self.pending;
        for i in 1..32 {
            if (en & (1 << i)) != 0 && (pend & (1 << i)) != 0 && self.priority[i as usize] > 0 {
                return true;
            }
        }
        false
    }
    pub fn read(&mut self, off: u64) -> u32 {
        if off < 0x80 { return self.priority[off as usize / 4]; }
        if off >= 0x1000 && off < 0x1004 { return self.pending; }
        if off >= 0x2080 && off < 0x2080 + 8 * 0x100 {
            return self.senable[((off - 0x2080) / 0x100) as usize];
        }
        if off >= 0x201000 && off < 0x201000 + 8 * 0x2000 {
            let context_off = off - 0x201000;
            let hart = (context_off / 0x2000) as usize;
            if hart < 8 {
                if context_off % 0x2000 == 0 { return self.spriority[hart]; }
                if context_off % 0x2000 == 4 {
                    let mut best = 0u32;
                    let en = self.senable[hart];
                    for i in 1..32 {
                        if (en & (1 << i)) != 0 && (self.pending & (1 << i)) != 0 && self.priority[i] > 0 {
                            best = i as u32; break;
                        }
                    }
                     if best != 0 { self.pending &= !(1 << best); }
                    return best;
                }
            }
        }
        0
    }
    pub fn write(&mut self, off: u64, val: u32) {
        if off < 0x80 { self.priority[off as usize / 4] = val; return; }
        if off >= 0x2080 && off < 0x2080 + 8 * 0x100 {
            self.senable[((off - 0x2080) / 0x100) as usize] = val; return;
        }
        if off >= 0x201000 && off < 0x201000 + 8 * 0x2000 {
            let context_off = off - 0x201000;
            let hart = (context_off / 0x2000) as usize;
            if hart < 8 {
                if context_off % 0x2000 == 0 { self.spriority[hart] = val; }
            }
        }
    }
}

pub struct Clint { pub mtimecmp: [u64; 8] }
impl Clint {
    pub fn new() -> Self { Self { mtimecmp: [0; 8] } }
}

pub struct VirtioBlk {
    pub disk: Option<File>, pub disk_size: u64,
    driver_features: u32, queue_sel: u32, queue_num: u32, pub queue_ready: u32,
    pub interrupt_status: u32, pub status: u32,
    pub queue_desc: u64, pub queue_avail: u64, pub queue_used: u64,
}
impl VirtioBlk {
    pub fn new() -> Self {
        Self { disk: None, disk_size: 0, driver_features: 0, queue_sel: 0, queue_num: 0,
            queue_ready: 0, interrupt_status: 0, status: 0,
            queue_desc: 0, queue_avail: 0, queue_used: 0 }
    }
    pub fn init(&mut self, path: &str) {
        let f = OpenOptions::new().read(true).write(true).open(path).expect("cannot open disk");
        self.disk_size = f.metadata().unwrap().len();
        self.disk = Some(f);
    }
    pub fn process_queue(&mut self, ram: &mut [u8]) -> bool {
        if self.queue_ready == 0 || self.disk.is_none() { return false; }
        let desc_base = self.queue_desc;
        let avail_base = self.queue_avail;
        let used_base = self.queue_used;
        let ai = r16(ram, avail_base + 2);
        let ui = r16(ram, used_base + 2);
        if ui == ai { return false; }
        let di = r16(ram, avail_base + 4 + (ui as u64 % 8) * 2);
        let d0a = r64(ram, desc_base + di as u64 * 16);
        let d0n = r16(ram, desc_base + di as u64 * 16 + 14);
        let typ = r32(ram, d0a);
        let sec = r64(ram, d0a + 8);
        let d1a = r64(ram, desc_base + d0n as u64 * 16);
        let d1l = r32(ram, desc_base + d0n as u64 * 16 + 8);
        let d1n = r16(ram, desc_base + d0n as u64 * 16 + 14);
        let d2a = r64(ram, desc_base + d1n as u64 * 16);
        if let Some(ref f) = self.disk {
            let mut file = f.try_clone().unwrap();
            if typ == 0 {
                let mut buf = vec![0u8; d1l as usize];
                file.seek(SeekFrom::Start(sec * 512)).ok();
                file.read_exact(&mut buf).ok();
                for i in 0..d1l as usize { w8(ram, d1a + i as u64, buf[i]); }
            } else if typ == 1 {
                let mut buf = vec![0u8; d1l as usize];
                for i in 0..d1l as usize { buf[i] = r8(ram, d1a + i as u64); }
                file.seek(SeekFrom::Start(sec * 512)).ok();
                file.write_all(&buf).ok();
                file.flush().ok();
            }
        }
        w8(ram, d2a, 0);
        let ue = ui as u64 % 8;
        w32(ram, used_base + 4 + ue * 8, di as u32);
        w32(ram, used_base + 4 + ue * 8 + 4, 0);
        w16(ram, used_base + 2, ui.wrapping_add(1));
        self.interrupt_status |= 1;
        true
    }
}

fn r8(ram: &[u8], a: u64) -> u8 { if a < ram.len() as u64 { ram[a as usize] } else { 0 } }
fn r16(ram: &[u8], a: u64) -> u16 { r8(ram, a) as u16 | (r8(ram, a+1) as u16) << 8 }
fn r32(ram: &[u8], a: u64) -> u32 { r8(ram, a) as u32 | (r8(ram, a+1) as u32) << 8 | (r8(ram, a+2) as u32) << 16 | (r8(ram, a+3) as u32) << 24 }
fn r64(ram: &[u8], a: u64) -> u64 { r8(ram, a) as u64 | (r8(ram, a+1) as u64) << 8 | (r8(ram, a+2) as u64) << 16 | (r8(ram, a+3) as u64) << 24 | (r8(ram, a+4) as u64) << 32 | (r8(ram, a+5) as u64) << 40 | (r8(ram, a+6) as u64) << 48 | (r8(ram, a+7) as u64) << 56 }
fn w8(ram: &mut [u8], a: u64, v: u8) { if a < ram.len() as u64 { ram[a as usize] = v; } }
fn w16(ram: &mut [u8], a: u64, v: u16) { w8(ram, a, v as u8); w8(ram, a+1, (v>>8) as u8); }
fn w32(ram: &mut [u8], a: u64, v: u32) { w8(ram, a, v as u8); w8(ram, a+1, (v>>8) as u8); w8(ram, a+2, (v>>16) as u8); w8(ram, a+3, (v>>24) as u8); }
fn w64(ram: &mut [u8], a: u64, v: u64) { for i in 0..8 { w8(ram, a+i, (v>>(i*8)) as u8); } }

use std::sync::OnceLock;
static BOOT_TIME: OnceLock<std::time::Instant> = OnceLock::new();
pub fn boot_time_ns() -> u64 {
    let start = BOOT_TIME.get_or_init(|| std::time::Instant::now());
    start.elapsed().as_nanos() as u64
}
pub fn get_time_ns() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64
}
pub fn get_time_csr() -> u64 { boot_time_ns() / 100 }

pub struct Bus {
    pub ram: Vec<u8>,
    uart: Uart,
    pub plic: Plic,
    pub clint: Clint,
    pub vblk: VirtioBlk,
    pub lr_reservations: [Option<u64>; 8],
}
impl Bus {
    pub fn new(ram_size: usize) -> Self {
        Self { ram: vec![0; ram_size], uart: Uart::new(), plic: Plic::new(), clint: Clint::new(), vblk: VirtioBlk::new(), lr_reservations: [None; 8] }
    }

    pub fn load8(&mut self, addr: u64) -> u8 {
        if addr >= RAM_BASE && addr < RAM_BASE + self.ram.len() as u64 { return self.ram[(addr - RAM_BASE) as usize]; }
        if addr >= UART_BASE && addr < UART_BASE + 8 { return self.uart.read(addr - UART_BASE); }
        if addr >= PLIC_BASE && addr < PLIC_BASE + PLIC_SIZE { return (self.plic.read(addr - PLIC_BASE) >> ((addr & 3) * 8)) as u8; }
        if addr >= CLINT_BASE && addr < CLINT_BASE + CLINT_SIZE {
            let off = addr - CLINT_BASE;
            if off >= 0xBFF8 && off < 0xC000 { return (get_time_csr() >> ((off - 0xBFF8) * 8)) as u8; }
            if off >= 0x4000 && off < 0x4000 + 64 {
                let hart = ((off - 0x4000) / 8) as usize;
                if hart < 8 { return (self.clint.mtimecmp[hart] >> ((off - 0x4000) % 8 * 8)) as u8; }
            }
        }
        if addr >= RTC_BASE && addr < RTC_BASE + 8 {
            let t = get_time_ns();
            return (t >> ((addr & 7) * 8)) as u8;
        }
        if addr >= VIRTIO0_BASE && addr < VIRTIO0_BASE + VIRTIO_SIZE {
            let off = addr - VIRTIO0_BASE;
            let reg = off & !3;
            let byte = off & 3;
            let val: u32 = match reg {
                0x000 => 0x74726976, 0x004 => 2, 0x008 => 2, 0x00C => 0x554d4551,
                0x010 => 0, 0x034 => 256,
                0x044 => self.vblk.queue_ready, 0x060 => self.vblk.interrupt_status,
                0x070 => self.vblk.status,
                _ => 0,
            };
            return (val >> (byte * 8)) as u8;
        }
        0
    }
    pub fn load16(&mut self, a: u64) -> u16 { self.load8(a) as u16 | (self.load8(a+1) as u16) << 8 }
    pub fn load32(&mut self, a: u64) -> u32 {
        self.load8(a) as u32 | (self.load8(a+1) as u32) << 8 | (self.load8(a+2) as u32) << 16 | (self.load8(a+3) as u32) << 24
    }
    pub fn load64(&mut self, a: u64) -> u64 {
        self.load8(a) as u64 | (self.load8(a+1) as u64) << 8 | (self.load8(a+2) as u64) << 16 | (self.load8(a+3) as u64) << 24
            | (self.load8(a+4) as u64) << 32 | (self.load8(a+5) as u64) << 40 | (self.load8(a+6) as u64) << 48 | (self.load8(a+7) as u64) << 56
    }

    pub fn store8(&mut self, addr: u64, val: u8) {
        if addr >= RAM_BASE && addr < RAM_BASE + self.ram.len() as u64 { self.ram[(addr - RAM_BASE) as usize] = val; self.clear_reservations(addr); return; }
        if addr >= UART_BASE && addr < UART_BASE + 8 { self.uart.write(addr - UART_BASE, val); return; }
    }
    pub fn store16(&mut self, addr: u64, val: u16) {
        if addr >= RAM_BASE && addr < RAM_BASE + self.ram.len() as u64 {
            let idx = (addr - RAM_BASE) as usize;
            self.ram[idx] = val as u8;
            self.ram[idx + 1] = (val >> 8) as u8;
            self.clear_reservations(addr);
            return;
        }
    }
    pub fn store32(&mut self, addr: u64, val: u32) {
        if addr >= RAM_BASE && addr < RAM_BASE + self.ram.len() as u64 {
            w32(&mut self.ram, addr - RAM_BASE, val); self.clear_reservations(addr); return;
        }
        if addr >= VIRTIO0_BASE && addr < VIRTIO0_BASE + VIRTIO_SIZE {
            let off = addr - VIRTIO0_BASE;
            match off {
                0x020 => self.vblk.driver_features = val,
                0x030 => self.vblk.queue_sel = val,
                0x038 => self.vblk.queue_num = val,
                0x044 => self.vblk.queue_ready = val,
                0x050 => { if self.vblk.process_queue(&mut self.ram) { self.plic.set_pending(1); } }
                0x064 => self.vblk.interrupt_status &= !val,
                0x070 => self.vblk.status = val,
                0x080 => self.vblk.queue_desc = (self.vblk.queue_desc & 0xFFFFFFFF00000000) | val as u64,
                0x084 => self.vblk.queue_desc = (self.vblk.queue_desc & 0xFFFFFFFF) | (val as u64) << 32,
                0x090 => self.vblk.queue_avail = (self.vblk.queue_avail & 0xFFFFFFFF00000000) | val as u64,
                0x094 => self.vblk.queue_avail = (self.vblk.queue_avail & 0xFFFFFFFF) | (val as u64) << 32,
                0x0a0 => self.vblk.queue_used = (self.vblk.queue_used & 0xFFFFFFFF00000000) | val as u64,
                0x0a4 => self.vblk.queue_used = (self.vblk.queue_used & 0xFFFFFFFF) | (val as u64) << 32,
                _ => {}
            }
            return;
        }
        if addr >= CLINT_BASE && addr < CLINT_BASE + CLINT_SIZE {
            let off = addr - CLINT_BASE;
            if off >= 0x4000 && off < 0x4000 + 64 {
                let hart = ((off - 0x4000) / 8) as usize;
                if hart < 8 {
                    let is_hi = off % 8 >= 4;
                    if is_hi { self.clint.mtimecmp[hart] = (self.clint.mtimecmp[hart] & 0xFFFFFFFF) | (val as u64) << 32; }
                    else { self.clint.mtimecmp[hart] = (self.clint.mtimecmp[hart] & 0xFFFFFFFF00000000) | val as u64; }
                }
            }
            return;
        }
        if addr >= PLIC_BASE && addr < PLIC_BASE + PLIC_SIZE {
            self.plic.write(addr - PLIC_BASE, val); return;
        }
    }
    pub fn store64(&mut self, addr: u64, val: u64) {
        if addr >= RAM_BASE && addr < RAM_BASE + self.ram.len() as u64 { w64(&mut self.ram, addr - RAM_BASE, val); self.clear_reservations(addr); return; }
        if addr >= CLINT_BASE && addr < CLINT_BASE + CLINT_SIZE {
            let off = addr - CLINT_BASE;
            if off >= 0x4000 && off < 0x4000 + 64 {
                let hart = ((off - 0x4000) / 8) as usize;
                if hart < 8 { self.clint.mtimecmp[hart] = val; }
            }
            return;
        }
    }

    pub fn mmio_read(&mut self, addr: u64, size: u8) -> u64 {
        match size { 1 => self.load8(addr) as u64, 2 => self.load16(addr) as u64, 4 => self.load32(addr) as u64, 8 => self.load64(addr) as u64, _ => 0 }
    }
    pub fn mmio_write(&mut self, addr: u64, val: u64, size: u8) {
        match size { 1 => self.store8(addr, val as u8), 2 => self.store16(addr, val as u16), 4 => self.store32(addr, val as u32), 8 => self.store64(addr, val), _ => {} }
    }

    pub fn uart_has_irq(&self) -> bool { self.uart.has_irq() }
    pub fn uart_needs_pending(&self) -> bool { self.uart.needs_pending() }
    pub fn uart_clear_pending_tx_irq(&mut self) { self.uart.clear_pending_tx_irq(); }
    pub fn uart_rx_ready(&self) -> bool { self.uart.rx_ready() }
    pub fn uart_ier(&self) -> u8 { self.uart.ier }
    pub fn clear_reservations(&mut self, addr: u64) {
        for r in self.lr_reservations.iter_mut() {
            if *r == Some(addr) { *r = None; }
        }
    }
    pub fn push_uart_rx(&mut self, c: u8) { self.uart.push_rx(c); }
    pub fn clint_has_mti(&self, hart: usize, now: u64) -> bool {
        hart < 8 && self.clint.mtimecmp[hart] <= now
    }
}
