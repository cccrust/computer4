use std::fs::File;
use std::io::Read;
use crate::memory::{Bus, RAM_BASE, RAM_SIZE};

const ET_REL: u16 = 1;
const ET_EXEC: u16 = 2;
const EM_RISCV: u16 = 243;
const PT_LOAD: u32 = 1;
const SHT_RELA: u32 = 4;
const SHT_SYMTAB: u32 = 2;
const SHF_ALLOC: u64 = 2;
const SHT_STRTAB: u32 = 3;

const R_RISCV_NONE: u32 = 0;
const R_RISCV_32: u32 = 1;
const R_RISCV_64: u32 = 2;
const R_RISCV_JAL: u32 = 11;
const R_RISCV_BRANCH: u32 = 16;
const R_RISCV_CALL: u32 = 18;
const R_RISCV_CALL_PLT: u32 = 19;
const R_RISCV_HI20: u32 = 7;
const R_RISCV_LO12_I: u32 = 8;
const R_RISCV_LO12_S: u32 = 9;
const R_RISCV_PCREL_HI20: u32 = 23;
const R_RISCV_PCREL_LO12_I: u32 = 24;
const R_RISCV_PCREL_LO12_S: u32 = 25;

fn r16(data: &[u8], off: usize) -> u16 {
    data[off] as u16 | (data[off + 1] as u16) << 8
}
fn r32(data: &[u8], off: usize) -> u32 {
    data[off] as u32 | (data[off + 1] as u32) << 8 | (data[off + 2] as u32) << 16 | (data[off + 3] as u32) << 24
}
fn r64(data: &[u8], off: usize) -> u64 {
    r32(data, off) as u64 | (r32(data, off + 4) as u64) << 32
}

struct Section {
    sh_addr: u64, sh_offset: u64, sh_size: u64,
    sh_type: u32, sh_flags: u64, sh_link: u32, sh_info: u32, sh_addralign: u64,
}
struct Symbol { st_value: u64, st_shndx: u16, name: String }

/// Result of loading an ELF: entry point and whether it's a standalone program (vs kernel).
pub struct LoadResult { pub entry: u64, pub is_64: bool, pub is_standalone: bool }

pub fn load_elf(path: &str, bus: &mut Bus) -> LoadResult {
    let mut fp = File::open(path).expect("cannot open file");
    let mut buf = Vec::new();
    fp.read_to_end(&mut buf).expect("cannot read file");

    if buf.len() < 16 || buf[0] != 0x7f || buf[1] != b'E' || buf[2] != b'L' || buf[3] != b'F' {
        panic!("not an ELF file");
    }
    let is_64 = buf[4] == 2;
    let e_type = if is_64 { r16(&buf, 16) } else { r16(&buf, 16) };
    let e_machine = if is_64 { r16(&buf, 18) } else { r16(&buf, 18) };
    if e_machine != EM_RISCV { panic!("not RISC-V ELF"); }

    if is_64 {
        if e_type == ET_REL {
            load_relocatable_64(&buf, bus)
        } else if e_type == ET_EXEC {
            load_exec_64(&buf, bus)
        } else {
            panic!("unsupported ELF type {}", e_type);
        }
    } else {
        if e_type == ET_REL {
            load_relocatable_32(&buf, bus)
        } else if e_type == ET_EXEC {
            load_exec_32(&buf, bus)
        } else {
            panic!("unsupported ELF type {}", e_type);
        }
    }
}

fn load_exec_64(data: &[u8], bus: &mut Bus) -> LoadResult {
    let entry = r64(data, 24);
    let phoff = r64(data, 32);
    let phentsize = r16(data, 54);
    let phnum = r16(data, 56);
    let mut standalone = false;
    for i in 0..phnum {
        let off = phoff as usize + i as usize * phentsize as usize;
        if off + 56 > data.len() { break; }
        let ptype = r32(data, off);
        if ptype != PT_LOAD { continue; }
        let paddr = r64(data, off + 16);
        let memsz = r64(data, off + 40);
        let poff = r64(data, off + 8);
        let filesz = r64(data, off + 32);
        if memsz == 0 { continue; }
        if paddr < RAM_BASE {
            standalone = true;
            let end = (paddr + memsz) as usize;
            if end > bus.ram.len() { panic!("program too large (0x{:x})", end); }
            for j in 0..filesz.min(memsz) as usize {
                if poff as usize + j < data.len() {
                    bus.ram[(paddr + j as u64) as usize] = data[poff as usize + j];
                }
            }
        } else if paddr >= RAM_BASE && paddr + memsz <= RAM_BASE + RAM_SIZE {
            let load_addr = (paddr - RAM_BASE) as usize;
            for j in 0..filesz.min(memsz) as usize {
                if poff as usize + j < data.len() {
                    bus.ram[load_addr + j] = data[poff as usize + j];
                }
            }
            for j in filesz as usize..memsz as usize {
                bus.ram[load_addr + j] = 0;
            }
        }
    }
    LoadResult { entry, is_64: true, is_standalone: standalone }
}

fn load_exec_32(data: &[u8], bus: &mut Bus) -> LoadResult {
    let entry = r32(data, 24) as u64;
    let phoff = r32(data, 28) as u64;
    let phentsize = r16(data, 42);
    let phnum = r16(data, 44);
    let mut standalone = false;
    for i in 0..phnum {
        let off = phoff as usize + i as usize * phentsize as usize;
        if off + 32 > data.len() { break; }
        let ptype = r32(data, off);
        if ptype != PT_LOAD { continue; }
        let paddr = r32(data, off + 12) as u64;
        let memsz = r32(data, off + 20) as u64;
        let poff = r32(data, off + 4) as u64;
        let filesz = r32(data, off + 16) as u64;
        if memsz == 0 { continue; }
        if paddr < RAM_BASE {
            standalone = true;
            let end = (paddr + memsz) as usize;
            if end > bus.ram.len() { panic!("program too large (0x{:x})", end); }
            for j in 0..filesz.min(memsz) as usize {
                if poff as usize + j < data.len() {
                    bus.ram[(paddr + j as u64) as usize] = data[poff as usize + j];
                }
            }
        } else if paddr >= RAM_BASE && paddr + memsz <= RAM_BASE + RAM_SIZE {
            let load_addr = (paddr - RAM_BASE) as usize;
            for j in 0..filesz.min(memsz) as usize {
                if poff as usize + j < data.len() {
                    bus.ram[load_addr + j] = data[poff as usize + j];
                }
            }
            for j in filesz as usize..memsz as usize {
                bus.ram[load_addr + j] = 0;
            }
        }
    }
    LoadResult { entry, is_64: false, is_standalone: standalone }
}

// ─── Relocatable (ET_REL) support ───

fn get_sections_64(data: &[u8], e_shoff: u64, e_shnum: u16, e_shentsize: u16) -> Vec<Section> {
    let mut secs = Vec::new();
    for i in 0..e_shnum {
        let off = (e_shoff + i as u64 * e_shentsize as u64) as usize;
        let sh_type = r32(data, off + 4);
        let sh_flags = r64(data, off + 8);
        let sh_addr = r64(data, off + 16);
        let sh_offset = r64(data, off + 24);
        let sh_size = r64(data, off + 32);
        let sh_link = r32(data, off + 40);
        let sh_info = r32(data, off + 44);
        let sh_addralign = r64(data, off + 48);
        secs.push(Section { sh_addr, sh_offset, sh_size, sh_type, sh_flags, sh_link, sh_info, sh_addralign });
    }
    secs
}

fn get_sections_32(data: &[u8], e_shoff: u64, e_shnum: u16, e_shentsize: u16) -> Vec<Section> {
    let mut secs = Vec::new();
    for i in 0..e_shnum {
        let off = (e_shoff + i as u64 * e_shentsize as u64) as usize;
        let sh_type = r32(data, off + 4);
        let sh_flags = r32(data, off + 8) as u64;
        let sh_addr = r32(data, off + 12) as u64;
        let sh_offset = r32(data, off + 16) as u64;
        let sh_size = r32(data, off + 20) as u64;
        let sh_link = r32(data, off + 24);
        let sh_info = r32(data, off + 28);
        let sh_addralign = r32(data, off + 32) as u64;
        secs.push(Section { sh_addr, sh_offset, sh_size, sh_type, sh_flags, sh_link, sh_info, sh_addralign });
    }
    secs
}

fn find_symtab_64(data: &[u8], secs: &[Section]) -> Option<(Vec<Symbol>, usize)> {
    for sec in secs {
        if sec.sh_type == SHT_SYMTAB {
            let n = sec.sh_size / 24;
            let mut syms = Vec::new();
            for j in 0..n {
                let off = (sec.sh_offset + j * 24) as usize;
                let st_name = r32(data, off);
                let st_shndx = r16(data, off + 6);
                let st_value = r64(data, off + 8);
                let name = if (sec.sh_link as usize) < secs.len() {
                    let strsec = &secs[sec.sh_link as usize];
                    rstr(data, strsec.sh_offset as usize + st_name as usize)
                } else { String::new() };
                syms.push(Symbol { st_value, st_shndx, name });
            }
            return Some((syms, sec.sh_link as usize));
        }
    }
    None
}

fn find_symtab_32(data: &[u8], secs: &[Section]) -> Option<(Vec<Symbol>, usize)> {
    for sec in secs {
        if sec.sh_type == SHT_SYMTAB {
            let n = sec.sh_size / 16;
            let mut syms = Vec::new();
            for j in 0..n {
                let off = (sec.sh_offset + j * 16) as usize;
                let st_name = r32(data, off);
                let st_value = r32(data, off + 4) as u64;
                let st_shndx = r16(data, off + 14);
                let name = if sec.sh_link > 0 {
                    let strsec = &secs[sec.sh_link as usize];
                    rstr(data, strsec.sh_offset as usize + st_name as usize)
                } else { String::new() };
                syms.push(Symbol { st_value, st_shndx, name });
            }
            return Some((syms, sec.sh_link as usize));
        }
    }
    None
}

fn rstr(data: &[u8], off: usize) -> String {
    let mut v = Vec::new();
    let mut i = off;
    while i < data.len() && data[i] != 0 { v.push(data[i]); i += 1; }
    String::from_utf8_lossy(&v).to_string()
}

fn find_entry(sec_addrs: &[u64], syms: &[Symbol]) -> u64 {
    for sym in syms {
        if sym.name == "_start" && sym.st_shndx > 0 {
            let si = sym.st_shndx as usize;
            if si < sec_addrs.len() { return sec_addrs[si] + sym.st_value; }
        }
    }
    0x10000
}

fn load_relocatable_64(data: &[u8], bus: &mut Bus) -> LoadResult {
    let e_shoff = r64(data, 40);
    let e_shnum = r16(data, 60);
    let e_shentsize = r16(data, 58);
    let secs = get_sections_64(data, e_shoff, e_shnum, e_shentsize);
    let (syms, _) = find_symtab_64(data, &secs).unwrap_or((Vec::new(), 0));

    let base: u64 = 0x10000;
    let mut cur = base;
    let mut sec_addrs = vec![0u64; secs.len()];
    for (i, s) in secs.iter().enumerate() {
        if s.sh_flags & SHF_ALLOC != 0 {
            let align = s.sh_addralign.max(1);
            let a = (cur + align - 1) & !(align - 1);
            sec_addrs[i] = a;
            cur = a + s.sh_size;
        }
    }

    let total = std::cmp::max((cur + 0xfff) & !0xfff, 0x100000);
    if total as usize > bus.ram.len() { panic!("program too large ({} bytes)", total); }

    for (i, s) in secs.iter().enumerate() {
        if s.sh_flags & SHF_ALLOC != 0 && s.sh_size > 0 {
            let a = sec_addrs[i] as usize;
            let end = s.sh_offset as usize + s.sh_size as usize;
            if end <= data.len() {
                bus.ram[a..a + s.sh_size as usize].copy_from_slice(&data[s.sh_offset as usize..end]);
            }
        }
    }

    for (i, s) in secs.iter().enumerate() {
        if s.sh_type != SHT_RELA { continue; }
        let tgt = s.sh_info as usize;
        if tgt >= secs.len() || secs[tgt].sh_flags & SHF_ALLOC == 0 { continue; }
        let tgt_base = sec_addrs[tgt];
        let n = s.sh_size / 24;

        struct R { off: u64, ty: u32, sym: u32, add: i64 }
        let mut relocs: Vec<R> = Vec::new();
        for j in 0..n {
            let ro = (s.sh_offset + j * 24) as usize;
            let r_off = r64(data, ro);
            let r_info = r64(data, ro + 8);
            let r_add = r64(data, ro + 16) as i64;
            relocs.push(R { off: r_off, ty: (r_info & 0xffffffff) as u32, sym: (r_info >> 32) as u32, add: r_add });
        }

        let sym_val = |sym_idx: u32| -> Option<u64> {
            let sym = syms.get(sym_idx as usize)?;
            let si = sym.st_shndx as usize;
            if si > 0 && si < sec_addrs.len() { Some(sec_addrs[si] + sym.st_value) } else { None }
        };

        for r in &relocs {
            let sv = sym_val(r.sym).unwrap_or(0);
            if r.ty == R_RISCV_PCREL_LO12_I || r.ty == R_RISCV_PCREL_LO12_S {
                let label_addr = sv;
                let auipc_off = label_addr.wrapping_sub(tgt_base);
                let hi_sym = relocs.iter().find(|rr| rr.off == auipc_off && rr.ty == R_RISCV_PCREL_HI20)
                    .map(|rr| rr.sym);
                let auipc_pc = tgt_base + auipc_off;
                if let Some(hs) = hi_sym {
                    let real_sv = sym_val(hs).unwrap_or(0);
                    do_reloc_64(bus, tgt_base, r.off, r.ty, r.add, real_sv, tgt_base, auipc_pc);
                } else {
                    do_reloc_64(bus, tgt_base, r.off, r.ty, r.add, sv, tgt_base, auipc_pc);
                }
            } else {
                let pc = tgt_base + r.off;
                do_reloc_64(bus, tgt_base, r.off, r.ty, r.add, sv, tgt_base, pc);
            }
        }
    }

    let entry = find_entry(&sec_addrs, &syms);
    LoadResult { entry, is_64: true, is_standalone: true }
}

fn load_relocatable_32(data: &[u8], bus: &mut Bus) -> LoadResult {
    let e_shoff = r32(data, 32) as u64;
    let e_shnum = r16(data, 48);
    let e_shentsize = r16(data, 46);
    let secs = get_sections_32(data, e_shoff, e_shnum, e_shentsize);
    let (syms, _) = find_symtab_32(data, &secs).unwrap_or((Vec::new(), 0));

    let base: u64 = 0x10000;
    let mut cur = base;
    let mut sec_addrs = vec![0u64; secs.len()];
    for (i, s) in secs.iter().enumerate() {
        if s.sh_flags & SHF_ALLOC != 0 {
            let align = s.sh_addralign.max(1);
            let a = (cur + align - 1) & !(align - 1);
            sec_addrs[i] = a;
            cur = a + s.sh_size;
        }
    }

    let total = std::cmp::max((cur + 0xfff) & !0xfff, 0x100000);
    if total as usize > bus.ram.len() { panic!("program too large ({} bytes)", total); }

    for (i, s) in secs.iter().enumerate() {
        if s.sh_flags & SHF_ALLOC != 0 && s.sh_size > 0 {
            let a = sec_addrs[i] as usize;
            let end = s.sh_offset as usize + s.sh_size as usize;
            if end <= data.len() {
                bus.ram[a..a + s.sh_size as usize].copy_from_slice(&data[s.sh_offset as usize..end]);
            }
        }
    }

    for (i, s) in secs.iter().enumerate() {
        if s.sh_type != SHT_RELA { continue; }
        let tgt = s.sh_info as usize;
        if tgt >= secs.len() || secs[tgt].sh_flags & SHF_ALLOC == 0 { continue; }
        let tgt_base = sec_addrs[tgt];
        let n = s.sh_size / 12;

        struct R32 { off: u64, ty: u32, sym: u32, add: i64 }
        let mut relocs: Vec<R32> = Vec::new();
        for j in 0..n {
            let ro = (s.sh_offset + j * 12) as usize;
            let r_off = r32(data, ro) as u64;
            let r_info = r32(data, ro + 4) as u64;
            relocs.push(R32 { off: r_off, ty: (r_info & 0xff) as u32, sym: (r_info >> 8) as u32, add: r32(data, ro + 8) as i64 });
        }

        let sym_val = |sym_idx: u32| -> Option<u64> {
            let sym = syms.get(sym_idx as usize)?;
            let si = sym.st_shndx as usize;
            if si > 0 && si < sec_addrs.len() { Some(sec_addrs[si] + sym.st_value) } else { None }
        };

        for r in &relocs {
            let sv = sym_val(r.sym).unwrap_or(0);
            if r.ty == R_RISCV_PCREL_LO12_I || r.ty == R_RISCV_PCREL_LO12_S {
                let label_addr = sv;
                let auipc_off = label_addr.wrapping_sub(tgt_base);
                let hi_sym = relocs.iter().find(|rr| rr.off == auipc_off && rr.ty == R_RISCV_PCREL_HI20)
                    .map(|rr| rr.sym);
                let auipc_pc = tgt_base + auipc_off;
                if let Some(hs) = hi_sym {
                    let real_sv = sym_val(hs).unwrap_or(0);
                    do_reloc_32(bus, tgt_base, r.off, r.ty, r.add, real_sv, tgt_base, auipc_pc);
                } else {
                    do_reloc_32(bus, tgt_base, r.off, r.ty, r.add, sv, tgt_base, auipc_pc);
                }
            } else {
                let pc = tgt_base + r.off;
                do_reloc_32(bus, tgt_base, r.off, r.ty, r.add, sv, tgt_base, pc);
            }
        }
    }

    let entry = find_entry(&sec_addrs, &syms);
    LoadResult { entry, is_64: false, is_standalone: true }
}

fn do_reloc_64(bus: &mut Bus, tgt_base: u64, r_off: u64, r_ty: u32, r_add: i64, sym_val: u64, _tgt: u64, pc: u64) {
    let addr = (tgt_base + r_off) as usize;
    match r_ty {
        R_RISCV_NONE => {}
        R_RISCV_64 => bus.ram[addr..addr + 8].copy_from_slice(&sym_val.wrapping_add(r_add as u64).to_le_bytes()),
        R_RISCV_32 => bus.ram[addr..addr + 4].copy_from_slice(&(sym_val.wrapping_add(r_add as u64) as u32).to_le_bytes()),
        R_RISCV_HI20 => {
            let v = (sym_val.wrapping_add(r_add as u64) >> 12) as u32;
            let insn = r32(&bus.ram, addr) & 0x00000fff;
            bus.ram[addr..addr + 4].copy_from_slice(&(insn | (v & 0xfffff) << 12).to_le_bytes());
        }
        R_RISCV_LO12_I => {
            let v = (sym_val.wrapping_add(r_add as u64) & 0xfff) as u32;
            let insn = r32(&bus.ram, addr) & 0x000fffff;
            bus.ram[addr..addr + 4].copy_from_slice(&(insn | v << 20).to_le_bytes());
        }
        R_RISCV_LO12_S => {
            let v = (sym_val.wrapping_add(r_add as u64) & 0xfff) as u32;
            let insn = r32(&bus.ram, addr) & 0x01fff07f;
            bus.ram[addr..addr + 4].copy_from_slice(&(insn | (v >> 5) << 25 | (v & 0x1f) << 7).to_le_bytes());
        }
        R_RISCV_PCREL_HI20 => {
            let diff = (sym_val.wrapping_add(r_add as u64) as i64).wrapping_sub(pc as i64);
            let v = ((diff + 0x800) >> 12) as u32;
            let insn = r32(&bus.ram, addr) & 0x00000fff;
            bus.ram[addr..addr + 4].copy_from_slice(&(insn | (v & 0xfffff) << 12).to_le_bytes());
        }
        R_RISCV_PCREL_LO12_I => {
            let diff = (sym_val.wrapping_add(r_add as u64) as i64).wrapping_sub(pc as i64);
            let v = (diff & 0xfff) as u32;
            let insn = r32(&bus.ram, addr) & 0x000fffff;
            bus.ram[addr..addr + 4].copy_from_slice(&(insn | v << 20).to_le_bytes());
        }
        R_RISCV_PCREL_LO12_S => {
            let diff = (sym_val.wrapping_add(r_add as u64) as i64).wrapping_sub(pc as i64);
            let v = (diff & 0xfff) as u32;
            let insn = r32(&bus.ram, addr) & 0x01fff07f;
            bus.ram[addr..addr + 4].copy_from_slice(&(insn | (v >> 5) << 25 | (v & 0x1f) << 7).to_le_bytes());
        }
        R_RISCV_JAL => {
            let diff = (sym_val.wrapping_add(r_add as u64) as i64).wrapping_sub(pc as i64);
            encode_j(bus, addr, diff as i32);
        }
        R_RISCV_BRANCH => {
            let diff = (sym_val.wrapping_add(r_add as u64) as i64).wrapping_sub(pc as i64);
            encode_b(bus, addr, diff as i32);
        }
        R_RISCV_CALL | R_RISCV_CALL_PLT => {
            let diff = (sym_val.wrapping_add(r_add as u64) as i64).wrapping_sub(pc as i64);
            let hi = ((diff + 0x800) >> 12) as u32;
            let insn = r32(&bus.ram, addr) & 0x00000fff;
            bus.ram[addr..addr + 4].copy_from_slice(&(insn | (hi & 0xfffff) << 12).to_le_bytes());
            if addr + 4 < bus.ram.len() {
                let lo = (diff & 0xfff) as u32;
                let insn2 = r32(&bus.ram, addr + 4) & 0x000fffff;
                bus.ram[addr + 4..addr + 8].copy_from_slice(&(insn2 | lo << 20).to_le_bytes());
            }
        }
        _ => eprintln!("rvemu4: unhandled relocation {}", r_ty),
    }
}

fn do_reloc_32(bus: &mut Bus, tgt_base: u64, r_off: u64, r_ty: u32, r_add: i64, sym_val: u64, _tgt: u64, pc: u64) {
    let addr = (tgt_base + r_off) as usize;
    match r_ty {
        R_RISCV_NONE => {}
        R_RISCV_32 => bus.ram[addr..addr + 4].copy_from_slice(&(sym_val.wrapping_add(r_add as u64) as u32).to_le_bytes()),
        R_RISCV_HI20 => {
            let v = (sym_val.wrapping_add(r_add as u64) >> 12) as u32;
            let insn = r32(&bus.ram, addr) & 0x00000fff;
            bus.ram[addr..addr + 4].copy_from_slice(&(insn | (v & 0xfffff) << 12).to_le_bytes());
        }
        R_RISCV_LO12_I => {
            let v = (sym_val.wrapping_add(r_add as u64) & 0xfff) as u32;
            let insn = r32(&bus.ram, addr) & 0x000fffff;
            bus.ram[addr..addr + 4].copy_from_slice(&(insn | v << 20).to_le_bytes());
        }
        R_RISCV_LO12_S => {
            let v = (sym_val.wrapping_add(r_add as u64) & 0xfff) as u32;
            let insn = r32(&bus.ram, addr) & 0x01fff07f;
            bus.ram[addr..addr + 4].copy_from_slice(&(insn | (v >> 5) << 25 | (v & 0x1f) << 7).to_le_bytes());
        }
        R_RISCV_PCREL_HI20 => {
            let diff = (sym_val.wrapping_add(r_add as u64) as i64).wrapping_sub(pc as i64);
            let v = ((diff + 0x800) >> 12) as u32;
            let insn = r32(&bus.ram, addr) & 0x00000fff;
            bus.ram[addr..addr + 4].copy_from_slice(&(insn | (v & 0xfffff) << 12).to_le_bytes());
        }
        R_RISCV_PCREL_LO12_I => {
            let diff = (sym_val.wrapping_add(r_add as u64) as i64).wrapping_sub(pc as i64);
            let v = (diff & 0xfff) as u32;
            let insn = r32(&bus.ram, addr) & 0x000fffff;
            bus.ram[addr..addr + 4].copy_from_slice(&(insn | v << 20).to_le_bytes());
        }
        R_RISCV_PCREL_LO12_S => {
            let diff = (sym_val.wrapping_add(r_add as u64) as i64).wrapping_sub(pc as i64);
            let v = (diff & 0xfff) as u32;
            let insn = r32(&bus.ram, addr) & 0x01fff07f;
            bus.ram[addr..addr + 4].copy_from_slice(&(insn | (v >> 5) << 25 | (v & 0x1f) << 7).to_le_bytes());
        }
        R_RISCV_JAL => { let diff = (sym_val.wrapping_add(r_add as u64) as i64).wrapping_sub(pc as i64); encode_j(bus, addr, diff as i32); }
        R_RISCV_BRANCH => { let diff = (sym_val.wrapping_add(r_add as u64) as i64).wrapping_sub(pc as i64); encode_b(bus, addr, diff as i32); }
        R_RISCV_CALL | R_RISCV_CALL_PLT => {
            let diff = (sym_val.wrapping_add(r_add as u64) as i64).wrapping_sub(pc as i64);
            let hi = ((diff + 0x800) >> 12) as u32;
            let insn = r32(&bus.ram, addr) & 0x00000fff;
            bus.ram[addr..addr + 4].copy_from_slice(&(insn | (hi & 0xfffff) << 12).to_le_bytes());
            if addr + 4 < bus.ram.len() {
                let lo = (diff & 0xfff) as u32;
                let insn2 = r32(&bus.ram, addr + 4) & 0x000fffff;
                bus.ram[addr + 4..addr + 8].copy_from_slice(&(insn2 | lo << 20).to_le_bytes());
            }
        }
        _ => eprintln!("rvemu4: unhandled relocation {}", r_ty),
    }
}

fn encode_j(bus: &mut Bus, addr: usize, imm: i32) {
    let v = imm as u32;
    let encoded = ((v >> 20) & 1) << 31
                | ((v >> 1) & 0x3ff) << 21
                | ((v >> 11) & 1) << 20
                | ((v >> 12) & 0xff) << 12;
    let insn = r32(&bus.ram, addr) & 0x00000fff;
    bus.ram[addr..addr + 4].copy_from_slice(&(insn | encoded).to_le_bytes());
}

fn encode_b(bus: &mut Bus, addr: usize, imm: i32) {
    let v = imm as u32;
    let encoded = ((v >> 12) & 1) << 31
                | ((v >> 5) & 0x3f) << 25
                | ((v >> 1) & 0xf) << 8
                | ((v >> 11) & 1) << 7;
    let insn = r32(&bus.ram, addr) & 0x01fff07f;
    bus.ram[addr..addr + 4].copy_from_slice(&(insn | encoded).to_le_bytes());
}
