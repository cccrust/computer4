use std::fs::File;
use std::io::Read;
use crate::memory::Bus;

pub fn load_elf(path: &str, bus: &mut Bus) -> (u64, bool) {
    let mut fp = File::open(path).expect("cannot open kernel file");
    let mut buf = Vec::new();
    fp.read_to_end(&mut buf).expect("cannot read kernel file");

    if buf.len() < 16 { panic!("file too small for ELF"); }
    if buf[0] != 0x7f || buf[1] != b'E' || buf[2] != b'L' || buf[3] != b'F' {
        panic!("not an ELF file");
    }

    let is_64 = buf[4] == 2;
    let entry: u64;
    let phoff: u64;
    let phentsize: u16;
    let phnum: u16;

    if is_64 {
        if buf.len() < 64 { panic!("ELF64 header too short"); }
        entry = u64::from_le_bytes(buf[24..32].try_into().unwrap());
        phoff = u64::from_le_bytes(buf[32..40].try_into().unwrap());
        phentsize = u16::from_le_bytes(buf[54..56].try_into().unwrap());
        phnum = u16::from_le_bytes(buf[56..58].try_into().unwrap());
        for i in 0..phnum {
            let off = phoff as usize + i as usize * phentsize as usize;
            if off + 56 > buf.len() { break; }
            let ptype = u32::from_le_bytes(buf[off..off+4].try_into().unwrap());
            if ptype != 1 { continue; }
            let paddr = u64::from_le_bytes(buf[off+16..off+24].try_into().unwrap());
            let memsz = u64::from_le_bytes(buf[off+40..off+48].try_into().unwrap());
            if memsz == 0 { continue; }
            let poff = u64::from_le_bytes(buf[off+8..off+16].try_into().unwrap());
            let filesz = u64::from_le_bytes(buf[off+32..off+40].try_into().unwrap());
            if paddr >= crate::memory::RAM_BASE && paddr + memsz <= crate::memory::RAM_BASE + crate::memory::RAM_SIZE {
                let load_addr = (paddr - crate::memory::RAM_BASE) as usize;
                for j in 0..filesz.min(memsz) as usize {
                    if poff as usize + j < buf.len() {
                        bus.ram[load_addr + j] = buf[poff as usize + j];
                    }
                }
                for j in filesz as usize..memsz as usize {
                    bus.ram[load_addr + j] = 0;
                }
            }
        }
    } else {
        if buf.len() < 52 { panic!("ELF32 header too short"); }
        entry = u32::from_le_bytes(buf[24..28].try_into().unwrap()) as u64;
        phoff = u32::from_le_bytes(buf[28..32].try_into().unwrap()) as u64;
        phentsize = u16::from_le_bytes(buf[42..44].try_into().unwrap());
        phnum = u16::from_le_bytes(buf[44..46].try_into().unwrap());
        for i in 0..phnum {
            let off = phoff as usize + i as usize * phentsize as usize;
            if off + 32 > buf.len() { break; }
            let ptype = u32::from_le_bytes(buf[off..off+4].try_into().unwrap());
            if ptype != 1 { continue; }
            let paddr = u32::from_le_bytes(buf[off+12..off+16].try_into().unwrap()) as u64;
            let memsz = u32::from_le_bytes(buf[off+20..off+24].try_into().unwrap()) as u64;
            if memsz == 0 { continue; }
            let poff = u32::from_le_bytes(buf[off+4..off+8].try_into().unwrap()) as u64;
            let filesz = u32::from_le_bytes(buf[off+16..off+20].try_into().unwrap()) as u64;
            if paddr >= crate::memory::RAM_BASE && paddr + memsz <= crate::memory::RAM_BASE + crate::memory::RAM_SIZE {
                let load_addr = (paddr - crate::memory::RAM_BASE) as usize;
                for j in 0..filesz.min(memsz) as usize {
                    if poff as usize + j < buf.len() {
                        bus.ram[load_addr + j] = buf[poff as usize + j];
                    }
                }
                for j in filesz as usize..memsz as usize {
                    bus.ram[load_addr + j] = 0;
                }
            }
        }
    }

    (entry, is_64)
}
