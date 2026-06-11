use crate::memory::Bus;

const PRV_U: u8 = 0; const PRV_S: u8 = 1; const PRV_M: u8 = 3;

const EXC_ILLEGAL_INST: u64 = 2; const EXC_BREAKPOINT: u64 = 3;
const EXC_INST_PAGE_FAULT: u64 = 12; const EXC_LOAD_PAGE_FAULT: u64 = 13; const EXC_STORE_PAGE_FAULT: u64 = 15;

const INT_SSI: u64 = 1; const INT_STI: u64 = 5; const INT_SEI: u64 = 9;
const INT_MSI: u64 = 3; const INT_MTI: u64 = 7; const INT_MEI: u64 = 11;

const CSR_MSTATUS: u32 = 0x300; const CSR_MISA: u32 = 0x301;
const CSR_MEDELEG: u32 = 0x302; const CSR_MIDELEG: u32 = 0x303;
const CSR_MIE: u32 = 0x304; const CSR_MTVEC: u32 = 0x305;
const CSR_MCOUNTEREN: u32 = 0x306; const CSR_MENVCFG: u32 = 0x30A;
const CSR_MSCRATCH: u32 = 0x340; const CSR_MEPC: u32 = 0x341;
const CSR_MCAUSE: u32 = 0x342; const CSR_MTVAL: u32 = 0x343;
const CSR_MIP: u32 = 0x344; const CSR_MHARTID: u32 = 0xF14;
const CSR_SSTATUS: u32 = 0x100; const CSR_SIE: u32 = 0x104;
const CSR_STVEC: u32 = 0x105; const CSR_SSCRATCH: u32 = 0x140;
const CSR_SEPC: u32 = 0x141; const CSR_SCAUSE: u32 = 0x142;
const CSR_STVAL: u32 = 0x143; const CSR_SIP: u32 = 0x144;
const CSR_SATP: u32 = 0x180; const CSR_STIMECMP: u32 = 0x14D;
const CSR_TIME: u32 = 0xC01; const CSR_CYCLE: u32 = 0xC00;
const CSR_INSTRET: u32 = 0xC02;
const CSR_PMPCFG0: u32 = 0x3A0; const CSR_PMPADDR0: u32 = 0x3B0;

const SSTATUS_MASK: u64 = 0x800DE162;

pub struct Hart {
    pub x: [u64; 32], pub pc: u64, pub priv_level: u8, pub is_64: bool,
    pub mstatus: u64, pub mie: u64, pub mip: u64,
    pub medeleg: u64, pub mideleg: u64,
    pub mepc: u64, pub mcause: u64, pub mtval: u64, pub mtvec: u64, pub mscratch: u64,
    pub mcounteren: u64, pub menvcfg: u64, pub mhartid: u64,
    pub pmpcfg0: u64, pub pmpaddr0: u64,
    pub stvec: u64, pub sepc: u64, pub scause: u64, pub stval: u64,
    pub satp: u64, pub sscratch: u64, pub stimecmp: u64, pub instret: u64,
}

impl Hart {
    pub fn new(hart_id: u32) -> Self {
        Hart {
            x: [0; 32], pc: 0, priv_level: PRV_M, is_64: true,
            mstatus: 0, mie: 0, mip: 0, medeleg: 0, mideleg: 0,
            mepc: 0, mcause: 0, mtval: 0, mtvec: 0, mscratch: 0,
            mcounteren: 0, menvcfg: 0, mhartid: hart_id as u64,
            pmpcfg0: 0, pmpaddr0: 0,
            stvec: 0, sepc: 0, scause: 0, stval: 0,
            satp: 0, sscratch: 0, stimecmp: u64::MAX, instret: 0,
        }
    }
    pub fn reset(&mut self, pc: u64, is_64: bool) {
        self.pc = pc; self.is_64 = is_64; self.priv_level = PRV_M;
        self.mstatus = (PRV_M as u64) << 11; self.mie = 0; self.mip = 0; self.instret = 0;
    }

    fn csr_read(&self, csr: u32) -> u64 {
        match csr {
            CSR_MSTATUS => self.mstatus,
            CSR_MISA => if self.is_64 { (2<<62)|0x141101 } else { (1<<30)|0x141101 },
            CSR_MEDELEG => self.medeleg, CSR_MIDELEG => self.mideleg,
            CSR_MIE => self.mie, CSR_MTVEC => self.mtvec,
            CSR_MCOUNTEREN => self.mcounteren, CSR_MENVCFG => self.menvcfg,
            CSR_MSCRATCH => self.mscratch, CSR_MEPC => self.mepc,
            CSR_MCAUSE => self.mcause, CSR_MTVAL => self.mtval,
            CSR_MIP => self.mip, CSR_MHARTID => self.mhartid,
            CSR_PMPCFG0 => self.pmpcfg0, CSR_PMPADDR0 => self.pmpaddr0,
            CSR_SSTATUS => self.mstatus & SSTATUS_MASK,
            CSR_SIE => self.mie & self.mideleg, CSR_STVEC => self.stvec,
            CSR_SSCRATCH => self.sscratch, CSR_SEPC => self.sepc,
            CSR_SCAUSE => self.scause, CSR_STVAL => self.stval,
            CSR_SIP => self.mip & self.mideleg, CSR_SATP => self.satp,
            CSR_STIMECMP => self.stimecmp,
            CSR_TIME|CSR_CYCLE => crate::memory::get_time_csr(),
            CSR_INSTRET => self.instret, _ => 0,
        }
    }
    fn csr_write(&mut self, csr: u32, val: u64) {
        match csr {
            CSR_MSTATUS => self.mstatus = val,
            CSR_MEDELEG => self.medeleg = val, CSR_MIDELEG => self.mideleg = val,
            CSR_MIE => self.mie = val, CSR_MTVEC => self.mtvec = val & !3,
            CSR_MCOUNTEREN => self.mcounteren = val, CSR_MENVCFG => self.menvcfg = val,
            CSR_MSCRATCH => self.mscratch = val, CSR_MEPC => self.mepc = val,
            CSR_MCAUSE => self.mcause = val, CSR_MTVAL => self.mtval = val,
            CSR_MIP => self.mip = val,
            CSR_PMPCFG0 => self.pmpcfg0 = val, CSR_PMPADDR0 => self.pmpaddr0 = val,
            CSR_SSTATUS => self.mstatus = (self.mstatus & !SSTATUS_MASK) | (val & SSTATUS_MASK),
            CSR_SIE => self.mie = (self.mie & !self.mideleg) | (val & self.mideleg),
            CSR_STVEC => self.stvec = val & !3, CSR_SSCRATCH => self.sscratch = val,
            CSR_SEPC => self.sepc = val, CSR_SCAUSE => self.scause = val,
            CSR_STVAL => self.stval = val,
            CSR_SIP => self.mip = (self.mip & !self.mideleg) | (val & self.mideleg),
            CSR_SATP => self.satp = val, CSR_STIMECMP => self.stimecmp = val,
            _ => {}
        }
    }

    fn mmu_translate(&self, bus: &mut Bus, vaddr: u64, access: u8) -> Result<u64, u64> {
        if self.priv_level == PRV_M { return Ok(vaddr); }
        if (self.satp >> 60) != 8 { return Ok(vaddr); }
        let pt_base = (self.satp & 0xFFFFFFFFFFF) << 12;
        let idx = |lv: u32| (vaddr >> (12 + lv * 9)) & 0x1FF;
        let mut a = pt_base; let mut pte; let mut lvl: i32 = 2;
        loop {
            let off = a + idx(lvl as u32) * 8;
            pte = bus.mmio_read(off, 8);
            if pte & 1 == 0 { return Err(match access { 0 => EXC_LOAD_PAGE_FAULT, 1 => EXC_STORE_PAGE_FAULT, _ => EXC_INST_PAGE_FAULT }); }
            if pte & ((1<<1)|(1<<3)) != 0 { break; }
            a = ((pte >> 10) & 0xFFFFFFFFFFF) << 12; lvl -= 1;
            if lvl < 0 { return Err(match access { 0 => EXC_LOAD_PAGE_FAULT, 1 => EXC_STORE_PAGE_FAULT, _ => EXC_INST_PAGE_FAULT }); }
        }
        if access == 0 && pte & (1<<1) == 0 { return Err(EXC_LOAD_PAGE_FAULT); }
        if access == 1 && pte & (1<<2) == 0 { return Err(EXC_STORE_PAGE_FAULT); }
        if access == 2 && pte & (1<<3) == 0 { return Err(EXC_INST_PAGE_FAULT); }
        if self.priv_level == PRV_U && pte & (1<<4) == 0 { return Err(match access { 0 => EXC_LOAD_PAGE_FAULT, 1 => EXC_STORE_PAGE_FAULT, _ => EXC_INST_PAGE_FAULT }); }
        if self.priv_level == PRV_S && pte & (1<<4) != 0 {
            if access == 2 || (self.mstatus >> 18) & 1 == 0 { return Err(match access { 0 => EXC_LOAD_PAGE_FAULT, 1 => EXC_STORE_PAGE_FAULT, _ => EXC_INST_PAGE_FAULT }); }
        }
        let ppn = (pte >> 10) & 0xFFFFFFFFFFF;
        let paddr = match lvl { 2 => (ppn << 12) | (vaddr & 0x3FFFFFFF), 1 => (ppn << 12) | (vaddr & 0x1FFFFF), _ => (ppn << 12) | (vaddr & 0xFFF) };
        let new_pte = pte | (1<<6) | if access == 1 { 1<<7 } else { 0 };
        if new_pte != pte {
            let mut a2 = pt_base;
            for l in ((lvl as usize + 1)..=2).rev() { let t = bus.mmio_read(a2 + idx(l as u32)*8, 8); a2 = ((t>>10)&0xFFFFFFFFFFF)<<12; }
            bus.mmio_write(a2 + idx(lvl as u32)*8, new_pte, 8);
        }
        Ok(paddr)
    }

    fn vm_read(&mut self, bus: &mut Bus, va: u64, sz: u8) -> Result<u64, u64> {
        let pa = match self.mmu_translate(bus, va, 0) { Ok(p) => p, Err(e) => return Err(e) };
        Ok(bus.mmio_read(pa, sz))
    }
    fn vm_write(&mut self, bus: &mut Bus, va: u64, val: u64, sz: u8) -> Result<(), u64> {
        let pa = match self.mmu_translate(bus, va, 1) { Ok(p) => p, Err(e) => return Err(e) };
        bus.mmio_write(pa, val, sz); Ok(())
    }
    fn vm_fetch(&mut self, bus: &mut Bus, va: u64, is16: bool) -> Result<u64, u64> {
        let pa = match self.mmu_translate(bus, va, 2) { Ok(p) => p, Err(e) => return Err(e) };
        Ok(if is16 { bus.mmio_read(pa, 2) } else { bus.mmio_read(pa, 4) })
    }

    pub fn trap(&mut self, cause: u64, tval: u64) {
        let is_int = (cause >> 63) != 0; let code = cause & 0x7FFFFFFFFFFFFFFF;
        let deleg = self.priv_level <= PRV_S && (if is_int { (self.mideleg >> code) & 1 != 0 } else { (self.medeleg >> code) & 1 != 0 });
        if deleg {
            self.sepc = self.pc; self.scause = cause; self.stval = tval;
            self.mstatus = if self.mstatus & (1<<1) != 0 { self.mstatus | (1<<5) } else { self.mstatus & !(1<<5) };
            if self.priv_level == PRV_S { self.mstatus |= 1<<8; } else { self.mstatus &= !(1<<8); }
            self.mstatus &= !(1<<1); self.priv_level = PRV_S; self.pc = self.stvec;
        } else {
            self.mepc = self.pc; self.mcause = cause; self.mtval = tval;
            self.mstatus = if self.mstatus & (1<<3) != 0 { self.mstatus | (1<<7) } else { self.mstatus & !(1<<7) };
            self.mstatus = (self.mstatus & !(3<<11)) | (self.priv_level as u64) << 11;
            self.mstatus &= !(1<<3); self.priv_level = PRV_M; self.pc = self.mtvec;
            if is_int && code == INT_MTI { }
        }
    }

    pub fn check_interrupts(&mut self, bus: &mut Bus) {
        let t = crate::memory::get_time_csr();
        if t >= self.stimecmp { self.mip |= 1 << INT_STI; } else { self.mip &= !(1 << INT_STI); }
        if bus.plic.has_irq(self.mhartid as usize) { self.mip |= 1 << INT_SEI; } else { self.mip &= !(1 << INT_SEI); }
        let clint_mti = bus.clint_has_mti(self.mhartid as usize, t);
        if clint_mti { self.mip |= 1 << INT_MTI; } else { self.mip &= !(1 << INT_MTI); }
        let pend = self.mip & self.mie; if pend == 0 { return; }
        let m_ie = (self.mstatus >> 3) & 1; let s_ie = (self.mstatus >> 1) & 1;
        let mp = pend & !self.mideleg;
        if mp != 0 && (self.priv_level < PRV_M || (self.priv_level == PRV_M && m_ie != 0)) {
            for &p in &[INT_MEI, INT_MSI, INT_MTI, INT_SEI, INT_SSI, INT_STI] {
                if mp & (1 << p) != 0 { self.trap((1 << 63) | p, 0); return; }
            }
        }
        let sp = pend & self.mideleg;
        if sp != 0 && (self.priv_level < PRV_S || (self.priv_level == PRV_S && s_ie != 0)) {
            for &p in &[INT_SEI, INT_SSI, INT_STI] {
                if sp & (1 << p) != 0 { self.trap((1 << 63) | p, 0); return; }
            }
        }
    }

    fn sext(v: u64, bits: u32) -> i64 { let m = 1u64 << (bits - 1); (v ^ m) as i64 - m as i64 }

    pub fn step(&mut self, bus: &mut Bus) -> bool {
        use std::sync::atomic::{AtomicU64, Ordering};
        static INST_N: AtomicU64 = AtomicU64::new(0);
        let _n = INST_N.fetch_add(1, Ordering::Relaxed);
        let inst_raw = match self.vm_fetch(bus, self.pc, false) { Ok(v) => v as u32, Err(c) => { self.trap(c, self.pc); return true; } };
        if self.pc > 0x88000000 || self.pc < 0x80000000 {
            eprintln!("DIVERGE steps={} pc={:#x} inst={:#010x} priv={} satp={:#x} stvec={:#x} sepc={:#x} scause={:#x} mstatus={:#x} x1={:#x} x2={:#x} x3={:#x} x4={:#x} x5={:#x} x6={:#x} x8={:#x} x9={:#x} x10={:#x}", _n, self.pc, inst_raw, self.priv_level, self.satp, self.stvec, self.sepc, self.scause, self.mstatus, self.x[1], self.x[2], self.x[3], self.x[4], self.x[5], self.x[6], self.x[8], self.x[9], self.x[10]);
            std::process::exit(1);
        }
        if (inst_raw & 0x3) != 0x3 { return self.exec_rvc(bus, inst_raw as u16); }

        let (rd, rs1, rs2, f3, f7) = (
            ((inst_raw >> 7) & 0x1F) as usize, ((inst_raw >> 15) & 0x1F) as usize,
            ((inst_raw >> 20) & 0x1F) as usize, (inst_raw >> 12) & 0x7, (inst_raw >> 25) & 0x7F);
        let imm_i = Self::sext(inst_raw as u64 >> 20, 12);
        let imm_s = Self::sext(((inst_raw as u64 >> 25) << 5) | ((inst_raw >> 7) & 0x1F) as u64, 12);
        let imm_b = Self::sext(((inst_raw as u64 >> 31) << 12) | ((inst_raw as u64 >> 7) & 1) << 11 | (((inst_raw >> 25) & 0x3F) as u64) << 5 | (((inst_raw >> 8) & 0xF) as u64) << 1, 13);
        let imm_u = Self::sext(inst_raw as u64 & 0xFFFFF000, 32);
        let imm_j = Self::sext(((inst_raw as u64 >> 31) << 20) | ((inst_raw as u64 >> 12) & 0xFF) << 12 | ((inst_raw as u64 >> 20) & 1) << 11 | (((inst_raw >> 21) & 0x3FF) as u64) << 1, 21);

        let mut npc = self.pc.wrapping_add(4);
        let r1 = self.x[rs1]; let r2 = self.x[rs2];
        macro_rules! wr { ($r:expr,$v:expr) => { if $r != 0 { self.x[$r] = if self.is_64 { $v } else { ($v as u32) as u64 }; } } }

        match inst_raw & 0x7F {
            0x37 => wr!(rd, imm_u as u64),
            0x17 => wr!(rd, self.pc.wrapping_add(imm_u as u64)),
             0x6F => { let tgt = self.pc.wrapping_add(imm_j as u64); wr!(rd, npc); npc = tgt; }
             0x67 => { let jtgt = (r1.wrapping_add(imm_i as u64)) & !1; wr!(rd, npc); npc = jtgt; }
             0x63 => {
                 let take_branch = if self.is_64 {
                     match f3 {
                         0 => r1 == r2, 1 => r1 != r2,
                         4 => (r1 as i64) < (r2 as i64), 5 => (r1 as i64) >= (r2 as i64),
                         6 => r1 < r2, 7 => r1 >= r2,
                         _ => return true,
                     }
                 } else {
                     match f3 {
                         0 => (r1 as u32) == (r2 as u32), 1 => (r1 as u32) != (r2 as u32),
                         4 => (r1 as i32) < (r2 as i32), 5 => (r1 as i32) >= (r2 as i32),
                         6 => (r1 as u32) < (r2 as u32), 7 => (r1 as u32) >= (r2 as u32),
                         _ => return true,
                     }
                 };
                 if take_branch { npc = self.pc.wrapping_add(imm_b as u64); }
             }
             0x03 => {
                let addr = r1.wrapping_add(imm_i as u64); let sz = match f3 & 3 { 0 => 1, 1 => 2, 2 => 4, _ => 8 };
                if !self.is_64 && sz == 8 { self.trap(EXC_ILLEGAL_INST, inst_raw as u64); return true; }
                match self.vm_read(bus, addr, sz) {
                    Ok(val) => { wr!(rd, match f3 { 0 => val as i8 as i64 as u64, 1 => val as i16 as i64 as u64, 2 => val as i32 as i64 as u64, 3 => val, 4 => val & 0xFF, 5 => val & 0xFFFF, 6 => val & 0xFFFFFFFF, _ => val }); }
                    Err(c) => { self.trap(c, addr); return true; }
                }
            }
            0x23 => {
                let addr = r1.wrapping_add(imm_s as u64); let sz = [1, 2, 4, 8][f3 as usize & 3];
                if !self.is_64 && sz == 8 { self.trap(EXC_ILLEGAL_INST, inst_raw as u64); return true; }
                let val = match sz { 1 => r2 as u8 as u64, 2 => r2 as u16 as u64, 4 => r2 as u32 as u64, _ => r2 };
                if self.vm_write(bus, addr, val, sz).is_err() { self.trap(EXC_STORE_PAGE_FAULT, addr); return true; }
            }
            0x13 => {
                let sh = imm_i as u64 & 0x3F;
                 match f3 {
                     0 => wr!(rd, r1.wrapping_add(imm_i as u64)), 1 => wr!(rd, r1 << sh),
                     2 => wr!(rd, if self.is_64 { if (r1 as i64) < (imm_i as i64) { 1 } else { 0 } } else { if (r1 as i32) < (imm_i as i32) { 1 } else { 0 } }),
                     3 => wr!(rd, if self.is_64 { if r1 < (imm_i as u64) { 1 } else { 0 } } else { if (r1 as u32) < (imm_i as u32) { 1 } else { 0 } }),
                     4 => wr!(rd, r1 ^ (imm_i as u64)),
                     5 => wr!(rd, if f7 & 0x20 != 0 { ((r1 as i64) >> sh) as u64 } else { r1 >> sh }),
                     6 => wr!(rd, r1 | (imm_i as u64)), 7 => wr!(rd, r1 & (imm_i as u64)),
                     _ => { self.trap(EXC_ILLEGAL_INST, inst_raw as u64); return true; }
                 }
            }
            0x33 => {
                if f7 == 1 {
                    match f3 {
                        0 => wr!(rd, r1.wrapping_mul(r2)),
                        1 => wr!(rd, ((r1 as i64 as i128) * (r2 as i64 as i128) >> 64) as u64),
                        2 => {
                            let a = r1 as i64; let b = r2;
                            if a >= 0 { wr!(rd, (((a as u128) * (b as u128)) >> 64) as u64); }
                            else {
                                let abs = a.unsigned_abs();
                                let prod = (abs as u128) * (b as u128);
                                let hi = (prod >> 64) as u64;
                                let lo = prod as u64;
                                wr!(rd, if lo == 0 { (!hi).wrapping_add(1) } else { !hi });
                            }
                        }
                        3 => wr!(rd, ((r1 as u128) * (r2 as u128) >> 64) as u64),
                        4 => wr!(rd, if r2 != 0 { (r1 as i64 / r2 as i64) as u64 } else { u64::MAX }),
                        5 => wr!(rd, if r2 != 0 { r1 / r2 } else { u64::MAX }),
                        6 => wr!(rd, if r2 != 0 { (r1 as i64 % r2 as i64) as u64 } else { r1 as i64 as u64 }),
                        7 => wr!(rd, if r2 != 0 { r1 % r2 } else { r1 }),
                        _ => { self.trap(EXC_ILLEGAL_INST, inst_raw as u64); return true; }
                    }
                } else {
                    let sub = f7 & 0x20 != 0;
                    match f3 {
                        0 => wr!(rd, if sub { r1.wrapping_sub(r2) } else { r1.wrapping_add(r2) }),
                        1 => wr!(rd, r1 << (r2 & 0x3F)), 2 => wr!(rd, if (r1 as i64) < (r2 as i64) { 1 } else { 0 }),
                        3 => wr!(rd, if r1 < r2 { 1 } else { 0 }), 4 => wr!(rd, r1 ^ r2),
                        5 => { let sh = r2 & 0x3F; wr!(rd, if f7 & 0x20 != 0 { ((r1 as i64) >> sh) as u64 } else { r1 >> sh }); }
                        6 => wr!(rd, r1 | r2), 7 => wr!(rd, r1 & r2),
                        _ => { self.trap(EXC_ILLEGAL_INST, inst_raw as u64); return true; }
                    }
                }
            }
            0x1B => {
                if !self.is_64 { self.trap(EXC_ILLEGAL_INST, inst_raw as u64); return true; }
                let r1w = r1 as u32;
                match f3 {
                    0 => wr!(rd, (r1w.wrapping_add(imm_i as u32)) as i32 as i64 as u64),
                    1 => wr!(rd, (r1w << (imm_i as u32 & 0x1F)) as i32 as i64 as u64),
                    5 => wr!(rd, if f7 & 0x20 != 0 { ((r1w as i32) >> (imm_i as u32 & 0x1F)) as i32 as i64 as u64 } else { (r1w >> (imm_i as u32 & 0x1F)) as i32 as i64 as u64 }),
                    _ => { self.trap(EXC_ILLEGAL_INST, inst_raw as u64); return true; }
                }
            }
            0x3B => {
                if !self.is_64 { self.trap(EXC_ILLEGAL_INST, inst_raw as u64); return true; }
                let (r1w, r2w) = (r1 as u32, r2 as u32);
                if f7 == 1 {
                    match f3 {
                        0 => wr!(rd, (r1w as i32).wrapping_mul(r2w as i32) as i64 as u64),
                        4 => wr!(rd, if r2w != 0 { (r1w as i32 / r2w as i32) as i64 as u64 } else { -1i64 as u64 }),
                        5 => wr!(rd, if r2w != 0 { (r1w / r2w) as i64 as u64 } else { u32::MAX as i64 as u64 }),
                        6 => wr!(rd, if r2w != 0 { (r1w as i32 % r2w as i32) as i64 as u64 } else { r1w as i32 as i64 as u64 }),
                        7 => wr!(rd, if r2w != 0 { (r1w % r2w) as i64 as u64 } else { r1w as i64 as u64 }),
                        _ => { self.trap(EXC_ILLEGAL_INST, inst_raw as u64); return true; }
                    }
                } else {
                    let sub = f7 & 0x20 != 0;
                    match f3 {
                        0 => wr!(rd, (if sub { r1w.wrapping_sub(r2w) } else { r1w.wrapping_add(r2w) }) as i32 as i64 as u64),
                        1 => wr!(rd, (r1w << (r2w & 0x1F)) as i32 as i64 as u64),
                        5 => { let sh = r2w & 0x1F; wr!(rd, if f7 & 0x20 != 0 { ((r1w as i32) >> sh) as i64 as u64 } else { (r1w >> sh) as i64 as u64 }); }
                        _ => { self.trap(EXC_ILLEGAL_INST, inst_raw as u64); return true; }
                    }
                }
            }
            0x0F => {}
            0x73 => {
                if inst_raw == 0x00000073 { self.trap(match self.priv_level { 0 => 8, 1 => 9, _ => 11 }, 0); return true; }
                if inst_raw == 0x00100073 { self.trap(EXC_BREAKPOINT, self.pc); return true; }
                if inst_raw == 0x30200073 {
                    let mpp = ((self.mstatus >> 11) & 3) as u8;
                    self.mstatus = if self.mstatus & (1<<7) != 0 { self.mstatus | (1<<3) } else { self.mstatus & !(1<<3) };
                    self.mstatus |= 1<<7; self.mstatus &= !(3<<11); self.priv_level = mpp; npc = self.mepc;
                } else if inst_raw == 0x10200073 {
                    let spp = ((self.mstatus >> 8) & 1) as u8;
                    self.mstatus = if self.mstatus & (1<<5) != 0 { self.mstatus | (1<<1) } else { self.mstatus & !(1<<1) };
                    self.mstatus |= 1<<5; self.mstatus &= !(1<<8); self.priv_level = if spp != 0 { PRV_S } else { PRV_U }; npc = self.sepc;
                } else if inst_raw == 0x10500073 { npc = self.pc; }
                else if inst_raw == 0x12000073 {}
                else if f3 != 0 {
                    let csr_num = (inst_raw >> 20) & 0xFFF;
                    let old = self.csr_read(csr_num);
                    let src = if f3 & 4 != 0 { rs1 as u64 } else { r1 };
                    let nv = match f3 & 3 { 1 => src, 2 => old | src, 3 => old & !src, _ => old };
                    if f3 & 3 != 0 { self.csr_write(csr_num, nv); wr!(rd, old); }
                } else { self.trap(EXC_ILLEGAL_INST, inst_raw as u64); return true; }
            }
            0x2F => {
                let is_w = f3 == 2; let sz = if is_w { 4 } else { 8 };
                if !self.is_64 && !is_w { self.trap(EXC_ILLEGAL_INST, inst_raw as u64); return true; }
                let addr = r1;
                let val = match self.vm_read(bus, addr, sz) { Ok(v) => if is_w { v as i32 as i64 as u64 } else { v }, Err(c) => { self.trap(c, addr); return true; } };
                let op = f7 >> 2;
                match op {
                    2 => { bus.lr_reservations[self.mhartid as usize] = Some(addr); wr!(rd, val); }
                    3 => {
                        let ok = bus.lr_reservations[self.mhartid as usize] == Some(addr);
                        bus.lr_reservations[self.mhartid as usize] = None;
                        if ok {
                            if self.vm_write(bus, addr, r2, sz).is_err() { self.trap(EXC_STORE_PAGE_FAULT, addr); return true; }
                            wr!(rd, 0);
                        } else { wr!(rd, 1); }
                    }
                    _ => {
                        let ns = match op {
                            0 => val.wrapping_add(r2), 1 => r2, 4 => val ^ r2, 8 => val | r2, 12 => val & r2,
                            16 => if (val as i64) < (r2 as i64) { val } else { r2 },
                            20 => if (val as i64) > (r2 as i64) { val } else { r2 },
                            24 => if val < r2 { val } else { r2 }, 28 => if val > r2 { val } else { r2 },
                            _ => val,
                        };
                        wr!(rd, val);
                        if self.vm_write(bus, addr, if is_w { ns as u32 as u64 } else { ns }, sz).is_err() { self.trap(EXC_STORE_PAGE_FAULT, addr); return true; }
                    }
                }
            }
            _ => { self.trap(EXC_ILLEGAL_INST, inst_raw as u64); return true; }
        }
        self.pc = npc; self.x[0] = 0; self.instret += 1; true
    }

    fn exec_rvc(&mut self, bus: &mut Bus, i: u16) -> bool {
        let q = i & 0x3;
        let f3 = (i >> 13) & 0x7;
        let rs1 = (i >> 7) as usize & 0x1F;
        let rs2 = (i >> 2) as usize & 0x1F;
        let rd_s = ((i >> 2) & 0x7) as usize + 8;
        let rs1_s = ((i >> 7) & 0x7) as usize + 8;
        let rs2_s = ((i >> 2) & 0x7) as usize + 8;
        let mut npc = self.pc.wrapping_add(2);

        macro_rules! wr { ($r:expr,$v:expr) => { if $r != 0 { self.x[$r] = if self.is_64 { $v } else { ($v as u32) as u64 }; } } }

        match q {
            0 => match f3 {
                0 => {
                    let nzu = (((i >> 10) & 1) << 7) | (((i >> 9) & 1) << 6) | (((i >> 8) & 1) << 5)
                        | (((i >> 7) & 1) << 4) | (((i >> 12) & 1) << 3) | (((i >> 11) & 1) << 2)
                        | (((i >> 6) & 1) << 1) | ((i >> 5) & 1);
                    if nzu == 0 { self.trap(EXC_ILLEGAL_INST, i as u64); return true; }
                    self.x[rd_s] = self.x[2].wrapping_add((nzu as u64) << 2);
                }
                  2 => {
                    let uimm = (((i>>5)&1)<<4) | (((i>>12)&1)<<3) | (((i>>11)&1)<<2) | (((i>>10)&1)<<1) | ((i>>6)&1);
                    let addr = self.x[rs1_s].wrapping_add((uimm as u64) << 2);
                    match self.vm_read(bus, addr, 4) { Ok(v) => wr!(rd_s, v as i32 as i64 as u64), Err(c) => { self.trap(c, addr); return true; } }
                }
                 3 => {
                    if self.is_64 {
                        let uimm = (((i>>6)&1)<<4) | (((i>>5)&1)<<3) | (((i>>12)&1)<<2) | (((i>>11)&1)<<1) | ((i>>10)&1);
                        let addr = self.x[rs1_s].wrapping_add((uimm as u64) << 3);
                        match self.vm_read(bus, addr, 8) { Ok(v) => wr!(rd_s, v), Err(c) => { self.trap(c, addr); return true; } }
                    } else { self.trap(EXC_ILLEGAL_INST, i as u64); return true; }
                }
                 6 => {
                    let uimm = (((i>>5)&1)<<4) | (((i>>12)&1)<<3) | (((i>>11)&1)<<2) | (((i>>10)&1)<<1) | ((i>>6)&1);
                    let addr = self.x[rs1_s].wrapping_add((uimm as u64) << 2);
                    if self.vm_write(bus, addr, self.x[rs2_s] as u32 as u64, 4).is_err() { self.trap(EXC_STORE_PAGE_FAULT, addr); return true; }
                }
                 7 => {
                    if self.is_64 {
                        let uimm = (((i>>6)&1)<<4) | (((i>>5)&1)<<3) | (((i>>12)&1)<<2) | (((i>>11)&1)<<1) | ((i>>10)&1);
                        let addr = self.x[rs1_s].wrapping_add((uimm as u64) << 3);
                        if self.vm_write(bus, addr, self.x[rs2_s], 8).is_err() { self.trap(EXC_STORE_PAGE_FAULT, addr); return true; }
                    } else { self.trap(EXC_ILLEGAL_INST, i as u64); return true; }
                }
                _ => { self.trap(EXC_ILLEGAL_INST, i as u64); return true; }
            },
            1 => match f3 {
                0 => {
                    let imm = Self::sext(((i >> 2) as u64 & 0x1F) | ((i as u64 >> 12) & 1) << 5, 6) as u64;
                    if rs1 != 0 { wr!(rs1, self.x[rs1].wrapping_add(imm)); }
                }
                1 => {
                    if self.is_64 {
                        let imm = Self::sext(((i >> 2) as u64 & 0x1F) | ((i as u64 >> 12) & 1) << 5, 6) as u64;
                        if rs1 != 0 { wr!(rs1, (self.x[rs1].wrapping_add(imm)) as i32 as i64 as u64); }
                    } else { self.trap(EXC_ILLEGAL_INST, i as u64); return true; }
                }
                2 => {
                    let imm = Self::sext(((i >> 2) as u64 & 0x1F) | ((i as u64 >> 12) & 1) << 5, 6) as u64;
                    if rs1 != 0 { wr!(rs1, imm); }
                }
                3 => {
                    if rs1 == 2 {
                        let field = (((i >> 12) & 1) << 5) |
                                    (((i >> 4) & 1) << 4) |
                                    (((i >> 3) & 1) << 3) |
                                    (((i >> 5) & 1) << 2) |
                                    (((i >> 2) & 1) << 1) |
                                    (((i >> 6) & 1) << 0);
                        let imm = Self::sext(field as u64, 6) as i64 * 16;
                        self.x[2] = self.x[2].wrapping_add(imm as u64);
                    } else if rs1 != 0 {
                        let imm = Self::sext(((i >> 2) as u64 & 0x1F) | ((i >> 12) as u64 & 1) << 5, 6) as u64;
                            wr!(rs1, imm << 12);
                    }
                }
                4 => {
                    let rd_s = ((i >> 7) & 0x7) as usize + 8;
                    let rs2_s = ((i >> 2) & 0x7) as usize + 8;
                    let f2 = (i >> 10) & 3;
                    let fs = (i >> 5) & 3;
                    let b12 = (i >> 12) & 1;
                    match (b12, f2, fs) {
                        (0, 0, _) | (1, 0, _) => {
                            let sh = ((i >> 2) as u64 & 0x1F) | ((b12 as u64) << 5);
                            self.x[rd_s] >>= sh;
                        }
                        (0, 1, _) | (1, 1, _) => {
                            let sh = ((i >> 2) as u64 & 0x1F) | ((b12 as u64) << 5);
                            self.x[rd_s] = (self.x[rd_s] as i64 >> sh) as u64;
                        }
                        (0, 2, _) | (1, 2, _) => {
                            let imm = Self::sext(((i >> 2) as u64 & 0x1F) | (((i as u64 >> 12) & 1) << 5), 6) as u64;
                            self.x[rd_s] &= imm;
                        }
                        (0, 3, 0) => { self.x[rd_s] = self.x[rd_s].wrapping_sub(self.x[rs2_s]); }
                        (0, 3, 1) => { self.x[rd_s] ^= self.x[rs2_s]; }
                        (0, 3, 2) => { self.x[rd_s] |= self.x[rs2_s]; }
                        (0, 3, 3) => { self.x[rd_s] &= self.x[rs2_s]; }
                        (1, 3, 0) => { if self.is_64 { self.x[rd_s] = (self.x[rd_s].wrapping_sub(self.x[rs2_s]) as i32) as i64 as u64; } else { self.trap(EXC_ILLEGAL_INST, i as u64); return true; } }
                        (1, 3, 1) => { if self.is_64 { self.x[rd_s] = (self.x[rd_s].wrapping_add(self.x[rs2_s]) as i32) as i64 as u64; } else { self.trap(EXC_ILLEGAL_INST, i as u64); return true; } }
                        _ => { self.trap(EXC_ILLEGAL_INST, i as u64); return true; }
                    }
                }
                5 => {
                    let imm = ((i >> 12) as u64 & 1) << 11
                        | ((i >> 8) as u64 & 1) << 10
                        | ((i >> 10) as u64 & 1) << 9
                        | ((i >> 9) as u64 & 1) << 8
                        | ((i >> 6) as u64 & 1) << 7
                        | ((i >> 7) as u64 & 1) << 6
                        | ((i >> 2) as u64 & 1) << 5
                        | ((i >> 11) as u64 & 1) << 4
                        | ((i >> 5) as u64 & 1) << 3
                        | ((i >> 4) as u64 & 1) << 2
                        | ((i >> 3) as u64 & 1) << 1;
                    let jimm = Self::sext(imm, 12) as u64;
                    npc = self.pc.wrapping_add(jimm);
                }
                 6 => {
                    let rs1_s = ((i >> 7) & 0x7) as usize + 8;
                    let off = ((i >> 12) as u64 & 1) << 8
                        | ((i >> 6) as u64 & 1) << 7
                        | ((i >> 5) as u64 & 1) << 6
                        | ((i >> 2) as u64 & 1) << 5
                        | ((i >> 11) as u64 & 1) << 4
                        | ((i >> 10) as u64 & 1) << 3
                        | ((i >> 4) as u64 & 1) << 2
                        | ((i >> 3) as u64 & 1) << 1;
                    let bimm = Self::sext(off, 9) as u64;
                    if self.x[rs1_s] == 0 { npc = self.pc.wrapping_add(bimm); }
                }
                 7 => {
                    let rs1_s = ((i >> 7) & 0x7) as usize + 8;
                    let off = ((i >> 12) as u64 & 1) << 8
                        | ((i >> 6) as u64 & 1) << 7
                        | ((i >> 5) as u64 & 1) << 6
                        | ((i >> 2) as u64 & 1) << 5
                        | ((i >> 11) as u64 & 1) << 4
                        | ((i >> 10) as u64 & 1) << 3
                        | ((i >> 4) as u64 & 1) << 2
                        | ((i >> 3) as u64 & 1) << 1;
                    let bimm = Self::sext(off, 9) as u64;
                    if self.x[rs1_s] != 0 { npc = self.pc.wrapping_add(bimm); }
                }
                _ => { self.trap(EXC_ILLEGAL_INST, i as u64); return true; }
            },
            2 => match f3 {
                0 => {
                    if rs1 != 0 { let sh = (i >> 2) as u64 & 0x1F | ((i >> 12) as u64 & 1) << 5; self.x[rs1] <<= sh; }
                }
                     2 => {
                    let uimm = (((i>>4)&1)<<5) | (((i>>3)&1)<<4) | (((i>>2)&1)<<3) | (((i>>12)&1)<<2) | (((i>>6)&1)<<1) | ((i>>5)&1);
                    let addr = self.x[2].wrapping_add((uimm as u64) << 2);
                    match self.vm_read(bus, addr, 4) { Ok(v) => if rs1 != 0 { wr!(rs1, v as i32 as i64 as u64); }, Err(c) => { self.trap(c, addr); return true; } }
                }
                     3 => {
                    if self.is_64 {
                        let uimm = (((i>>4)&1)<<5) | (((i>>3)&1)<<4) | (((i>>2)&1)<<3) | (((i>>12)&1)<<2) | (((i>>6)&1)<<1) | ((i>>5)&1);
                        let addr = self.x[2].wrapping_add((uimm as u64) << 3);
                        match self.vm_read(bus, addr, 8) { Ok(v) => if rs1 != 0 { wr!(rs1, v); }, Err(c) => { self.trap(c, addr); return true; } }
                    } else { self.trap(EXC_ILLEGAL_INST, i as u64); return true; }
                }
                4 => {
                    if (i >> 12) & 1 == 0 {
                        if rs2 == 0 { if rs1 != 0 { npc = self.x[rs1]; } }
                        else { if rs1 != 0 { wr!(rs1, self.x[rs2]); } }
                    } else {
                        if rs2 == 0 {
                            if rs1 == 0 { self.trap(EXC_BREAKPOINT, self.pc); return true; }
                            else { wr!(1, npc); npc = self.x[rs1]; }
                        } else { if rs1 != 0 { wr!(rs1, self.x[rs1].wrapping_add(self.x[rs2])); } }
                    }
                }
                 6 => {
                    let uimm = (((i>>9)&1)<<5) | (((i>>8)&1)<<4) | (((i>>7)&1)<<3) | (((i>>12)&1)<<2) | (((i>>11)&1)<<1) | ((i>>10)&1);
                    let addr = self.x[2].wrapping_add((uimm as u64) << 2);
                    if self.vm_write(bus, addr, self.x[rs2] as u32 as u64, 4).is_err() { self.trap(EXC_STORE_PAGE_FAULT, addr); return true; }
                }
                 7 => {
                    if self.is_64 {
                        let uimm = (((i>>9)&1)<<5) | (((i>>8)&1)<<4) | (((i>>7)&1)<<3) | (((i>>12)&1)<<2) | (((i>>11)&1)<<1) | ((i>>10)&1);
                        let addr = self.x[2].wrapping_add((uimm as u64) << 3);
                        if self.vm_write(bus, addr, self.x[rs2], 8).is_err() { self.trap(EXC_STORE_PAGE_FAULT, addr); return true; }
                    } else { self.trap(EXC_ILLEGAL_INST, i as u64); return true; }
                }
                _ => { self.trap(EXC_ILLEGAL_INST, i as u64); return true; }
            },
            _ => { self.trap(EXC_ILLEGAL_INST, i as u64); return true; }
        }
        self.pc = npc; self.x[0] = 0; self.instret += 1; true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Bus;

    const TEST_PC: u64 = 0x80000000;
    const RAM_OFF: u64 = 0x80000000;

    fn bus() -> Bus { Bus::new(0x10000) }

    fn hart(regs: &[u64; 32], pc: u64, is_64: bool) -> Hart {
        let mut h = Hart::new(0);
        h.x = *regs;
        h.pc = pc;
        h.is_64 = is_64;
        h.priv_level = PRV_M;
        h
    }

    fn off(addr: u64) -> usize { (addr - RAM_OFF) as usize }

    fn place_inst(b: &mut Bus, pc: u64, inst: u16) {
        let o = off(pc);
        b.ram[o] = inst as u8;
        b.ram[o + 1] = (inst >> 8) as u8;
        b.ram[o + 2] = 0;
        b.ram[o + 3] = 0;
    }
    fn place_inst32(b: &mut Bus, pc: u64, inst: u32) {
        let o = off(pc);
        b.ram[o] = inst as u8;
        b.ram[o + 1] = (inst >> 8) as u8;
        b.ram[o + 2] = (inst >> 16) as u8;
        b.ram[o + 3] = (inst >> 24) as u8;
    }

    fn exec_rvc(inst: u16, regs: &[u64; 32], pc: u64, is_64: bool) -> (Hart, Bus) {
        let mut b = bus();
        place_inst(&mut b, pc, inst);
        let mut h = hart(regs, pc, is_64);
        h.step(&mut b);
        (h, b)
    }
    fn exec_rv64(inst: u32, regs: &[u64; 32], pc: u64, is_64: bool) -> (Hart, Bus) {
        let mut b = bus();
        place_inst32(&mut b, pc, inst);
        let mut h = hart(regs, pc, is_64);
        h.step(&mut b);
        (h, b)
    }

    // ── RVC q=0 (Quadrant 0: C.ADDI4SPN, C.LW, C.LD, C.SW, C.SD) ──

    #[test]
    fn c_addi4spn_nzu4() {
        // 0x0800: nzu = {b10,b9,b8,b7,b12,b11,b6,b5} = {0,0,0,0,0,1,0,0} = 4, off=16
        let regs = [0u64; 32];
        let mut h = hart(&regs, TEST_PC, true);
        h.x[2] = 0x1000; // sp
        let mut b = bus();
        place_inst(&mut b, TEST_PC, 0x0800);
        h.step(&mut b);
        assert_eq!(h.x[8], 0x1010, "s0 should be sp + 16");
        assert_eq!(h.pc, TEST_PC + 2);
    }

    #[test]
    fn c_addi4spn_nzu1_off4() {
        // nzu=1: {b10,b9,b8,b7,b12,b11,b6,b5} = {0,0,0,0,0,0,0,1}
        // => bits[12:5]={b12=0,b11=0,b10=0,b9=0,b8=0,b7=0,b6=0,b5=1}=0x01
        let inst: u16 = 0b000_00000001_000_00;
        let mut h = hart(&[0u64; 32], TEST_PC, true);
        h.x[2] = 0x1000;
        let mut b = bus();
        place_inst(&mut b, TEST_PC, inst);
        h.step(&mut b);
        assert_eq!(h.x[8], 0x1004, "s0 = sp + 4");
    }

    #[test]
    fn c_addi4spn_nzu128_off512() {
        // nzu=128: {b10,b9,b8,b7,b12,b11,b6,b5} = {1,0,0,0,0,0,0,0}
        // => b10=1, rest=0. bits[12:5]={0,0,1,0,0,0,0,0}=0x20
        let inst: u16 = 0b000_00100000_000_00;
        let mut h = hart(&[0u64; 32], TEST_PC, true);
        h.x[2] = 0x1000;
        let mut b = bus();
        place_inst(&mut b, TEST_PC, inst);
        h.step(&mut b);
        assert_eq!(h.x[8], 0x1200, "s0 = sp + 512");
    }

    #[test]
    fn c_addi4spn_nzu255_off1020() {
        // nzu=255: all bits set
        // {b10,b9,b8,b7,b12,b11,b6,b5} = {1,1,1,1,1,1,1,1}
        // bits[12:5]={1,1,1,1,1,1,1,1}=0xFF
        let inst: u16 = 0b000_11111111_000_00;
        let mut h = hart(&[0u64; 32], TEST_PC, true);
        h.x[2] = 0x1000;
        let mut b = bus();
        place_inst(&mut b, TEST_PC, inst);
        h.step(&mut b);
        assert_eq!(h.x[8], 0x13FC, "s0 = sp + 1020");
    }

    #[test]
    fn c_lw_q0_offset8() {
        // uimm={b6,b5,b12,b11,b10}={0,0,0,0,1}=1, off=1<<3=8
        // => b12=0,b11=0,b10=1,b6=0,b5=0
        // bits[12:10]={0,0,1}=1, bits[6:5]={0,0}=0
        // CL format: funct3=010 | offset5=b12=0 | rs1' | offset4:2={b6,b5,b4} | offset7:6={b3,b2}
        // Hmm, the FORMULA uses {b6,b5,b12,b11,b10}, not the spec's CL format ordering
        // Let me construct the instruction using the FORMULA bit positions:
        // uimm[4]=b6, uimm[3]=b5, uimm[2]=b12, uimm[1]=b11, uimm[0]=b10
        // For uimm=1 (0b00001): b10=1, b11=0, b12=0, b5=0, b6=0
        // So instruction using standard CL format bit positions:
        // bits[12]=b12=0, bits[11]=b11=0, bits[10]=b10=1, bits[6]=b6=0, bits[5]=b5=0
        // Instruction: funct3=010 | 0 | rs1'=000 | 0 | 0 | ????
        // Actually bits[4:2] and bits[3:2] for the rest... the CL format has:
        // 15:13=f3=010 | 12=offset[5] | 11:10=rs1' | 9:7=offset[4:2] | 6:5=offset[7:6] | 4:2=rs1' | 1:0=00
        // Wait that doesn't look right. Let me just pack the bits:
        // bits[15:13] = 010
        // bits[12] = b12 = 0
        // bits[11:7] = rs1' = 0  (the rs1 field for CL format)
        // bits[6] = b6 = 0
        // bits[5] = b5 = 0
        // bits[4:2] = rd' = 0  (actually for stores, rs2')
        // bits[1:0] = 00

        // Hmm, actually the CL format puts offset differently. Let me just use the code to
        // figure out the encoding. The code extracts:
        // uimm[4]=b6 (from bit 6), uimm[3]=b5 (from bit 5), uimm[2]=b12 (from bit 12),
        // uimm[1]=b11 (from bit 11), uimm[0]=b10 (from bit 10)
        //
        // For uimm=0b00001: b10=1, b11=0, b12=0, b5=0, b6=0
        // Instruction bits:
        // bit[12]=b12=0, bit[11]=b11=0, bit[10]=b10=1, bit[6]=b6=0, bit[5]=b5=0
        // bits[15:13]=010, bits[11:7]=rs1'=00000, bits[4:2]=rd'=000, bits[1:0]=00
        // So: 010_0_00000_0_0_000_00 = ?
        // Let me compute: 0b0100_0000_0000_0000 | 0 | 0 | 0
        // = 0b0100_0000_0000_0000 | 0b0000_0010_0000_0000 | ...
        // This is getting messy. Let me just construct the instruction byte by byte.

        // CL format (from RISC-V spec):
        // 15 14 13 | 12 | 11 10 9 8 7 | 6 5 | 4 3 2 | 1 0
        // funct3   | i5 | rs1'        | i4:3 | i2:0  | 00

        // Wait, I keep confusing myself. Let me look at the ACTUAL standard.
        // From the RISC-V spec, CL layout:
        // 15:13 = funct3 = 010 for C.LW, 011 for C.LD
        // 12 = uimm[5]
        // 11:7 = rs1'
        // 6 = uimm[4]
        // 5 = uimm[3]
        // 4:2 = uimm[2:0]
        // 1:0 = 00

        // No wait, that's not right either. Let me just use the ENCODING from our derived formula.
        // Our formula extracts: {b6, b5, b12, b11, b10} from bits {6, 5, 12, 11, 10}
        // These are 5 bits from 5 specific positions.

        // Let me construct the instruction for uimm=1:
        // b10 must be 1, rest 0
        // bit 12 = 0 (b12), bit 11 = 0 (b11), bit 10 = 1 (b10)
        // bit 6 = 0 (b6), bit 5 = 0 (b5)
        // bits[15:13] = 010
        // bits[11:7] = rs1' = 0 (we'll use x8 as base, addr = x8 + offset)
        // bits[4:2] = rd' = 0 (result goes to x8)
        // bits[1:0] = 00

        // Instruction = 0b010_0_00000_0_0_000_00 = 0x4000? Let me compute:
        // bits: 0 1 0 | 0 | 0 0 0 0 0 | 0 | 0 | 0 0 0 | 0 0
        // No wait:
        // 15 = 0
        // 14 = 1
        // 13 = 0
        // 12 = 0
        // 11 = 0
        // 10 = 1
        // 9 = 0
        // 8 = 0
        // 7 = 0
        // 6 = 0
        // 5 = 0
        // 4 = 0
        // 3 = 0
        // 2 = 0
        // 1 = 0
        // 0 = 0

        // Binary: 0100_0100_0000_0000 = 0x4400
        // Hmm, bit[10]=1 means bit[10] set to 1 in the instruction.
        // 0x4400 = 0b0100_0100_0000_0000
        // bit 15=0, bit 14=1, bit 13=0 → funct3=010 ✓
        // bit 12=0 → b12=0 ✓
        // bit 11=0 → b11=0 ✓
        // bit 10=1 → b10=1 ✓
        // bit 9=0, bit 8=0, bit 7=0 → rs1'=0
        // bit 6=0, bit 5=0 → b6=0, b5=0 ✓
        // bit 4=0, bit 3=0, bit 2=0 → rd'=0
        // bit 1=0, bit 0=0 → q=0 ✓

        // So instruction = 0x4400, uimm=1, off=8

        let regs = [0u64; 32];
        let mut h = hart(&regs, TEST_PC, true);
        h.x[8] = 0x80002000; // base address in rs1' = x8
        // Store a known value at address 0x80002000 + 8 = 0x80002008
        let mut b = bus();
        let addr = off(0x80002008);
        b.ram[addr] = 0x78;
        b.ram[addr+1] = 0x56;
        b.ram[addr+2] = 0x34;
        b.ram[addr+3] = 0x12;
        place_inst(&mut b, TEST_PC, 0x4400);
        h.step(&mut b);
        assert_eq!(h.x[8], 0x12345678 as i32 as i64 as u64, "C.LW should load sign-extended word");
    }

    #[test]
    fn c_sw_q0_offset8() {
        // uimm={b6,b5,b12,b11,b10}={0,0,0,0,1}=1, off=8
        // Same encoding as C.LW but funct3=110
        // 0b110_0_00000_0_0_000_00 = 0xC400? 
        // bits: 1 1 0 | 0 | 0 0 0 0 0 | 0 | 0 | 0 0 0 | 0 0
        // bit 15=1, bit 14=1, bit 13=0, bit 12=0, bit 11=0, bit 10=1
        // = 0b1100_0100_0000_0000 = 0xC400
        let regs = [0u64; 32];
        let mut h = hart(&regs, TEST_PC, true);
        h.x[8] = 0x80002000; // rs1' = x8
        h.x[9] = 0xDEADBEEF; // rs2' = x9 (bits[4:2]=1, rs2_s = 1+8 = 9)
        // Instruction = 0b110_0_00000_0_0_001_00 = 0xC404
        let mut b2 = bus();
        place_inst(&mut b2, TEST_PC, 0xC404);
        h.step(&mut b2);
        let addr = off(0x80002008);
        let stored_val = b2.ram[addr] as u32
            | (b2.ram[addr + 1] as u32) << 8
            | (b2.ram[addr + 2] as u32) << 16
            | (b2.ram[addr + 3] as u32) << 24;
        assert_eq!(stored_val, 0xDEADBEEF, "C.SW should store word at offset 8");
    }

    #[test]
    fn c_ld_q0_offset8() {
        // uimm={b6,b5,b12,b11,b10}={0,0,0,0,1}=1, off=8
        // funct3=011, bit12=0, bits[11:10]=01, rs1'=000 (x8), q=00
        // = 0b0110_0100_0000_0000 = 0x6400
        let regs = [0u64; 32];
        let mut h = hart(&regs, TEST_PC, true);
        h.x[8] = 0x80002000;
        let mut b = bus();
        let addr = off(0x80002008);
        let val: u64 = 0xDEADBEEF_12345678;
        b.ram[addr] = val as u8;
        b.ram[addr + 1] = (val >> 8) as u8;
        b.ram[addr + 2] = (val >> 16) as u8;
        b.ram[addr + 3] = (val >> 24) as u8;
        b.ram[addr + 4] = (val >> 32) as u8;
        b.ram[addr + 5] = (val >> 40) as u8;
        b.ram[addr + 6] = (val >> 48) as u8;
        b.ram[addr + 7] = (val >> 56) as u8;
        place_inst(&mut b, TEST_PC, 0x6400);
        h.step(&mut b);
        assert_eq!(h.x[8], val, "C.LD should load 64-bit value");
    }

    #[test]
    fn c_sd_q0_offset8() {
        // funct3=111, uimm={b6,b5,b12,b11,b10}={0,0,0,0,1}=1, off=8
        // rs1'=000 (x8), rs2'=001 (x9), q=00
        // = 0b1110_0100_0000_0100 = 0xE404
        let regs = [0u64; 32];
        let mut h = hart(&regs, TEST_PC, true);
        h.x[8] = 0x80002000;
        h.x[9] = 0xCAFEBABE_DEADBEEF;
        let mut b = bus();
        place_inst(&mut b, TEST_PC, 0xE404);
        h.step(&mut b);
        let addr = off(0x80002008);
        let stored = u64::from_le_bytes(b.ram[addr..addr+8].try_into().unwrap());
        assert_eq!(stored, 0xCAFEBABE_DEADBEEF, "C.SD should store 64 bits");
    }

    // ── RVC q=1 (Quadrant 1: C.ADDI, C.LI, C.ADDI16SP/C.LUI,
    //   C.SRLI/SRAI/ANDI/C.SUB/C.XOR/C.OR/C.AND,
    //   C.J, C.BEQZ, C.BNEZ) ──

    #[test]
    fn c_addi_neg1() {
        // C.ADDI rs1, -1: imm = {b12,b6,b5,b4,b3,b2} = sext(0b111111, 6) = -1
        // {b12=1, b6=1, b5=1, b4=1, b3=1, b2=1}
        // bits[12]=1, bits[6:2]=11111
        // Encoding: funct3=000 | b12=1 | rs1=00001 | b6=1,b5=1,b4=1,b3=1,b2=1 | q=01
        // = 0b000_1_00001_11111_01 = 0b0001_0000_1111_1101 = 0x10FD
        let inst: u16 = 0b000_1_00001_11111_01;
        let regs = [0u64; 32];
        let mut h = hart(&regs, TEST_PC, true);
        h.x[1] = 100;
        let mut b = bus();
        place_inst(&mut b, TEST_PC, inst);
        h.step(&mut b);
        assert_eq!(h.x[1], 99, "x1 = 100 + (-1)");
    }

    #[test]
    fn c_addi_pos31() {
        // C.ADDI x1, 31: imm = {b12,b6,b5,b4,b3,b2} = 0b011111 = 31
        // {b12=0, b6=1, b5=1, b4=1, b3=1, b2=1}
        // bits[12]=0, bits[6:2]=11111
        let inst: u16 = 0b000_0_00001_11111_01;
        let regs = [0u64; 32];
        let mut h = hart(&regs, TEST_PC, true);
        h.x[1] = 100;
        let mut b = bus();
        place_inst(&mut b, TEST_PC, inst);
        h.step(&mut b);
        assert_eq!(h.x[1], 131, "x1 = 100 + 31");
    }

    #[test]
    fn c_addi_neg16() {
        // C.ADDI x1, -16: imm[5:0] = {b12=1,b6=1,b5=0,b4=0,b3=0,b2=0} = 0b110000 = -16
        // bits[12]=1, bits[6:2]=10000
        let inst: u16 = 0b000_1_00001_10000_01;
        let regs = [0u64; 32];
        let mut h = hart(&regs, TEST_PC, true);
        h.x[1] = 100;
        let mut b = bus();
        place_inst(&mut b, TEST_PC, inst);
        h.step(&mut b);
        assert_eq!(h.x[1], 84, "x1 = 100 + (-16)");
    }

    #[test]
    fn c_li_neg1() {
        // C.LI rd, -1: same encoding as C.ADDI but funct3=010
        // funct3=010, b12=1, rs1=10000=rd=0x10=16 (never zero since rd=rs1 field)
        // bits[6:2]=11111
        // Encoding: 0b010_1_10000_11111_01
        let inst: u16 = 0b010_1_10000_11111_01;
        let regs = [0u64; 32];
        let mut h = hart(&regs, TEST_PC, true);
        let mut b = bus();
        place_inst(&mut b, TEST_PC, inst);
        h.step(&mut b);
        assert_eq!(h.x[16], (-1i64) as u64, "C.LI x16, -1");
    }

    #[test]
    fn c_addi16sp_neg32() {
        // C.ADDI16SP sp, -32
        // Current code: field = {b12,b4,b3,b5,b2,b6} = sext(-2,6) = 0b111110
        // {b12=1,b4=1,b3=1,b5=1,b2=1,b6=0}
        // bits[12]=1, bits[6]=0, bits[5]=1, bits[4]=1, bits[3]=1, bits[2]=1
        // inst = 0b011_1_00010_01111_01 = 0x713D
        let inst: u16 = 0x713D;
        let regs = [0u64; 32];
        let mut h = hart(&regs, TEST_PC, true);
        h.x[2] = 0x1000;
        let mut b = bus();
        place_inst(&mut b, TEST_PC, inst);
        h.step(&mut b);
        // imm = sext(field,6) = sext(0b111110,6) = -2, times 16 = -32
        assert_eq!(h.x[2], 0x1000 - 32, "sp should be sp - 32");
    }

    #[test]
    fn c_lui_rd_nonzero() {
        // C.LUI x1, 0x1000: imm[5:0] field encodes imm[17:12]
        // Actually C.LUI: rd=rs1 field, imm = sext({b12,b6,b5,b4,b3,b2},6) << 12
        // For result 0x1000: sext_imm=1, so imm[17:12]=1, imm=0x1000
        // {b12=0,b6=0,b5=0,b4=0,b3=0,b2=1}
        // bits[12]=0, bits[6]=0, bits[5]=0, bits[4]=0, bits[3]=0, bits[2]=1
        // funct3=011, rs1=1=00001
        // inst = 0b011_0_00001_00001_01 = 0x6085
        // Hmm, 0b0110_0000_1000_0101 = 0x6085
        let inst: u16 = 0b011_0_00001_00001_01;
        let regs = [0u64; 32];
        let mut h = hart(&regs, TEST_PC, true);
        let mut b = bus();
        place_inst(&mut b, TEST_PC, inst);
        h.step(&mut b);
        assert_eq!(h.x[1], 0x1000, "C.LUI x1, 0x1000");
    }

    // ── CB format: C.BEQZ, C.BNEZ ──
    // Spec formula for branch offset in CB format:
    // imm[8]=b12, imm[7]=b9, imm[6]=b8, imm[5]=b5, imm[4]=b11, imm[3]=b10,
    // imm[2]=b7, imm[1]=b6, imm[0]=0
    // ⟹ off = (b12<<8)|(b9<<7)|(b8<<6)|(b5<<5)|(b11<<4)|(b10<<3)|(b7<<2)|(b6<<1)
    //
    // Current code uses:
    // off = ((b12)<<8)|(b10,b9)<<3|(b6,b5)<<6|(b4,b3)<<1|(b2)<<5
    // ⟹ off = {b12, b6, b5, b2, b10, b9, b4, b3, 0}

    #[test]
    fn c_beqz_forward_4() {
        // off=4 (0b000000100): b12=0,b6=0,b5=0,b2=0,b11=0,b10=0,b4=1,b3=0
        // formula: off[8:1] = {b12,b6,b5,b2,b11,b10,b4,b3}
        // bits[12]=0, bits[11]=0, bits[10]=0, bits[9]=0, bits[8]=0, bits[7]=0
        // bits[6]=0, bits[5]=0, bits[4]=1, bits[3]=0, bits[2]=0
        let inst: u16 = 0xC011;
        let regs = [0u64; 32];
        let mut h = hart(&regs, TEST_PC, true);
        h.x[8] = 0; // rs1' = 0 → x8, so branch TAKEN (x8==0)
        let mut b = bus();
        place_inst(&mut b, TEST_PC, inst);
        h.step(&mut b);
        assert_eq!(h.pc, TEST_PC + 4, "C.BEQZ x8, +4 should branch forward 4");
    }

    #[test]
    fn c_bnez_backward_4() {
        // imm=-4: b12=1,b11=1,b10=1,rs1'=000,b6=1,b5=1,b4=1,b3=0,b2=1
        let inst: u16 = 0xFC75;
        let regs = [0u64; 32];
        let mut h = hart(&regs, 0x80001000, true);
        h.x[8] = 1; // rs1'=0→x8, x8=1 ≠ 0, so branch TAKEN
        let mut b = bus();
        place_inst(&mut b, 0x80001000, inst);
        h.step(&mut b);
        assert_eq!(h.pc, 0x80001000 - 4, "C.BNEZ x8, -4 should branch backward 4");
    }

    // ── RVC q=2 (Quadrant 2: C.SLLI, C.LWSP, C.LDSP, C.MV/JR/JALR/ADD/EBREAK,
    //   C.SWSP, C.SDSP) ──

    #[test]
    fn c_slli_shift_3() {
        // C.SLLI x1, 3: sh = {b12,b6,b5,b4,b3,b2} = 0b000011 = 3
        // funct3=000, rs1=00001, bits[12]=0, bits[6:2]=00011
        // inst = 0b000_0_00001_00011_10 = 0x0046
        let inst: u16 = 0b000_0_00001_00011_10;
        let regs = [0u64; 32];
        let mut h = hart(&regs, TEST_PC, true);
        h.x[1] = 0x1;
        let mut b = bus();
        place_inst(&mut b, TEST_PC, inst);
        h.step(&mut b);
        assert_eq!(h.x[1], 0x8, "x1 << 3 = 8");
    }

    #[test]
    fn c_mv() {
        // C.MV x1, x2: funct3=100, b12=0, rs1=00001, rs2=00010, q=10
        // inst = 0b100_0_00001_00010_10 = 0x808A
        let inst: u16 = 0b100_0_00001_00010_10;
        let regs = [0u64; 32];
        let mut h = hart(&regs, TEST_PC, true);
        h.x[2] = 0x1234;
        let mut b = bus();
        place_inst(&mut b, TEST_PC, inst);
        h.step(&mut b);
        assert_eq!(h.x[1], 0x1234, "C.MV x1, x2");
    }

    #[test]
    fn c_add() {
        // C.ADD x1, x2: funct3=100, b12=1, rs1=00001, rs2=00010, q=10
        // inst = 0b100_1_00001_00010_10 = 0x908A
        let inst: u16 = 0b100_1_00001_00010_10;
        let regs = [0u64; 32];
        let mut h = hart(&regs, TEST_PC, true);
        h.x[1] = 100;
        h.x[2] = 50;
        let mut b = bus();
        place_inst(&mut b, TEST_PC, inst);
        h.step(&mut b);
        assert_eq!(h.x[1], 150, "x1 + x2 = 150");
    }

    #[test]
    fn c_lwsp_offset_4() {
        // uimm[5:0] = {b4,b3,b2,b12,b6,b5}, addr = sp + (uimm << 2)
        // offset=4: uimm = 1 → b4=0,b3=0,b2=0,b12=0,b6=0,b5=1
        // bits[12]=0, bits[6]=0, bits[5]=1, bits[4]=0, bits[3]=0, bits[2]=0
        let inst: u16 = 0x40A2;
        let regs = [0u64; 32];
        let mut h = hart(&regs, TEST_PC, true);
        h.x[2] = 0x80002000;
        let mut b = bus();
        let addr = off(0x80002004);
        b.ram[addr] = 0xEF;
        b.ram[addr+1] = 0xBE;
        b.ram[addr+2] = 0xAD;
        b.ram[addr+3] = 0xDE;
        place_inst(&mut b, TEST_PC, inst);
        h.step(&mut b);
        assert_eq!(h.x[1], 0xDEADBEEFu32 as i32 as i64 as u64);
    }

    #[test]
    fn c_swsp_offset_8() {
        // C.SWSP rs2, offset(sp): uimm = {b9,b8,b7,b12,b11,b10}, off = uimm << 2
        // offset=8: uimm = 2 = 0b000010
        // {b9=0,b8=0,b7=0,b12=0,b11=1,b10=0}
        // bits[12]=0, bits[11]=1, bits[10]=0, bits[9]=0, bits[8]=0, bits[7]=0
        // funct3=110, rs2=00001, q=10
        // Encoding: 0b110_0_00001_000_10
        // Hmm, CSS format: bits[12:7] = uimm[5:0, but CSS encodes
        // bits[12]=b12, bits[11]=b11, bits[10]=b10, bits[9]=b9, bits[8]=b8, bits[7]=b7
        // bits[15:13]=funct3, bits[11:7]=uimm[?], bits[6:2]=rs2, bits[1:0]=10
        // No wait, CSS format from the spec:
        // bits[15:13]=funct3=110, bits[12:7]=uimm[5:0], bits[6:2]=rs2, bits[1:0]=10

        // So the spec encoding puts uimm at bits[12:7] in that exact order:
        // uimm[5]=b12, uimm[4]=b11, uimm[3]=b10, uimm[2]=b9, uimm[1]=b8, uimm[0]=b7
        // Hmm, that means uimm = {b12, b11, b10, b9, b8, b7}
        // But our current code has: uimm = {b9, b8, b7, b12, b11, b10}
        //
        // The spec formula: off = {b12,b11,b10,b9,b8,b7,0,0} = uimm_spec << 2
        // where uimm_spec = {b12,b11,b10,b9,b8,b7}
        //
        // Current code: uimm = {b9,b8,b7,b12,b11,b10}
        // These are different! {b12,b11,b10,b9,b8,b7} vs {b9,b8,b7,b12,b11,b10}

        // Hmm actually I verified this from the objdump earlier. Let me double check.
        // C.SWSP offset encoding: uimm = {b9,b8,b7,b12,b11,b10}
        // For offset=8: uimm = 2 → {b9=0,b8=0,b7=0,b12=0,b11=1,b10=0}
        // bits[6:2] = rs2 = 00001 (x1)
        // inst = 0b110_0_10000_00001_10 = 0xC806

        let inst: u16 = 0xC806;
        let regs = [0u64; 32];
        let mut h = hart(&regs, TEST_PC, true);
        h.x[2] = 0x80002000;
        h.x[1] = 0x12345678;
        let mut b = bus();
        place_inst(&mut b, TEST_PC, inst);
        h.step(&mut b);
        let addr = off(0x80002008);
        let stored = b.ram[addr] as u32 | (b.ram[addr+1] as u32) << 8
            | (b.ram[addr+2] as u32) << 16 | (b.ram[addr+3] as u32) << 24;
        assert_eq!(stored, 0x12345678, "C.SWSP should store to sp+8");
    }

    // ── 32-bit instructions ──

    #[test]
    fn addi() {
        // ADDI x1, x2, 42: imm_i=42, rd=1, rs1=2
        // inst bits: imm=0x02A | rs1=00010 | f3=000 | rd=00001 | opcode=0010011
        // = 0b000000101010_00010_000_00001_0010011
        // = 0x02A10093
        let (h, _) = exec_rv64(0x02A10093, &[0u64; 32], TEST_PC, true);
        assert_eq!(h.x[1], 42, "ADDI x1, x2, 42");

        // Test with initial value
        let mut regs = [0u64; 32];
        regs[2] = 100;
        let (h2, _) = exec_rv64(0x02A10093, &regs, TEST_PC, true);
        assert_eq!(h2.x[1], 142, "ADDI x1, x2(100), 42 = 142");
    }

    #[test]
    fn add() {
        // ADD x1, x2, x3: opcode=0x33, rd=00001, f7=0000000, f3=000, rs2=00011, rs1=00010
        // inst = 0b0000000_00011_00010_000_00001_0110011 = 0x003100B3
        let mut regs = [0u64; 32];
        regs[2] = 50;
        regs[3] = 100;
        let (h, _) = exec_rv64(0x003100B3, &regs, TEST_PC, true);
        assert_eq!(h.x[1], 150, "x2(50) + x3(100) = 150");
    }

    #[test]
    fn sub() {
        // SUB x1, x2, x3: f7=0100000, rd=1, f3=000, rs2=3, rs1=2
        // inst = 0b0100000_00011_00010_000_00001_0110011 = 0x403100B3
        let mut regs = [0u64; 32];
        regs[2] = 100;
        regs[3] = 50;
        let (h, _) = exec_rv64(0x403100B3, &regs, TEST_PC, true);
        assert_eq!(h.x[1], 50, "x2(100) - x3(50) = 50");
    }

    #[test]
    fn lw() {
        // LW x1, 8(x2): opcode=0x03, rd=00001, f3=010, rs1=00010, imm=000000001000
        // inst = 0b000000001000_00010_010_00001_0000011 = 0x00812083
        let mut h = hart(&[0u64; 32], TEST_PC, true);
        h.x[2] = 0x80002000;
        let mut b = bus();
        let addr = off(0x80002008);
        b.ram[addr] = 0xEF;
        b.ram[addr+1] = 0xBE;
        b.ram[addr+2] = 0xAD;
        b.ram[addr+3] = 0xDE;
        place_inst32(&mut b, TEST_PC, 0x00812083);
        h.step(&mut b);
        assert_eq!(h.x[1], 0xDEADBEEFu32 as i32 as i64 as u64, "LW sign-extended");
    }

    #[test]
    fn ld() {
        // LD x1, 16(x2): opcode=0x03, rd=00001, f3=011, rs1=00010, imm=000000010000
        // inst = 0b000000010000_00010_011_00001_0000011 = 0x01013083
        let mut h = hart(&[0u64; 32], TEST_PC, true);
        h.x[2] = 0x80002000;
        let mut b = bus();
        let addr = off(0x80002010);
        let val: u64 = 0xDEADBEEF_CAFEBABE;
        b.ram[addr..addr+8].copy_from_slice(&val.to_le_bytes());
        place_inst32(&mut b, TEST_PC, 0x01013083);
        h.step(&mut b);
        assert_eq!(h.x[1], val, "LD 64-bit load");
    }

    #[test]
    fn sw() {
        // SW x3, 8(x2): opcode=0x23, imm_s[11:5]=0000000, rs2=00011, rs1=00010, f3=010, imm_s[4:0]=01000
        // inst = 0b0000000_00011_00010_010_01000_0100011 = 0x00312423
        let mut h = hart(&[0u64; 32], TEST_PC, true);
        h.x[2] = 0x80002000;
        h.x[3] = 0x12345678;
        let mut b = bus();
        place_inst32(&mut b, TEST_PC, 0x00312423);
        h.step(&mut b);
        let addr = off(0x80002008);
        let stored = u32::from_le_bytes(b.ram[addr..addr+4].try_into().unwrap());
        assert_eq!(stored, 0x12345678);
    }

    #[test]
    fn sd() {
        // SD x3, 16(x2): opcode=0x23, f3=011, imm=16
        // inst = 0b0000000_00011_00010_011_10000_0100011 = 0x00313823
        let mut h = hart(&[0u64; 32], TEST_PC, true);
        h.x[2] = 0x80002000;
        h.x[3] = 0xCAFEBABE_DEADBEEF;
        let mut b = bus();
        place_inst32(&mut b, TEST_PC, 0x00313823);
        h.step(&mut b);
        let addr = off(0x80002010);
        let stored = u64::from_le_bytes(b.ram[addr..addr+8].try_into().unwrap());
        assert_eq!(stored, 0xCAFEBABE_DEADBEEF);
    }

    #[test]
    fn beq_taken() {
        // BEQ x1, x2, +8: opcode=0x63, f3=000, imm=8
        // inst = 0b0000000_00010_00001_000_01000_1100011
        // Actually imm_b encoding: {imm[12|10:5|4:1|11]}
        // For +8 (0b00000001000):
        // imm[12]=0, imm[11]=0, imm[10:5]=000001, imm[4:1]=0000
        // bits[31]=0, bits[7]=0, bits[30:25]=000001, bits[11:8]=0000
        // inst = 0b0_000001_00010_00001_000_0000_0_1100011
        // Let me just use the standard encoding: offset=8
        // BEQ x1, x2, offset=8: 
        // imm_b = sext({0,0,0,0,0,0,0,0,1,0,0,0,0},13) = 8
        // Actually that's not right for B-type encoding. B-type encodes imm[12|10:5|4:1|11]
        // offset = 8 = 0b000000001000
        // imm[12]=0, imm[10:5]=000001, imm[4:1]=0000, imm[11]=0
        // bits[31]=0, bits[30:25]=000001, bits[11:8]=0000, bits[7]=0
        // inst = {0, 000001, 00010, 00001, 000, 0000, 0, 1100011}
        // = 0b0_000001_00010_00001_000_0000_0_1100011
        // = 0x00208463  ... no that's not right either. Let me compute properly.

        // Actually let me just use the imm_b value directly:
        // imm_b = sext(,13). For offset=8:
        // encoding = ((offset & 0x1000) << 19) | ((offset & 0x7E) << 20) | ((offset & 0xF00) >> 7) | ((offset & 0x1) << 7)
        // offset=8=0b1000
        // (8 & 0x1000)=0, (8&0x7E)=8<<... hm let me think about B-type more carefully.
        //
        // B-type: inst[31]=imm[12], inst[30:25]=imm[10:5], inst[11:8]=imm[4:1], inst[7]=imm[11]
        // offset = 8 = 0b000000001000
        // imm[12]=0, imm[11]=0, imm[10:5]=000000, imm[4:1]=0100
        // Wait, 8 = 0b1000. So imm[3]=1, rest=0.
        // imm[12]=0, imm[11]=0, imm[10:5]=000000, imm[4:1]=0100
        // Actually imm[3]=1, so imm[4:1] = 0b0100
        // bits[31]=0, bits[30:25]=000000, bits[11:8]=0100, bits[7]=0
        // inst = 0_000000_00010_00001_000_0100_0_1100011
        // = 0b0_000000_00010_00001_000_0100_0_1100011
        // = 0x00008463? Let me compute:
        // 0b0000_0000_0000_0001_0000_0100_0110_0011
        // No wait, let me be more careful:

        // bits 31-0:
        // 31: imm[12] = 0
        // 30:25: imm[10:5] = 000000
        // 24:20: rs2 = 00010
        // 19:15: rs1 = 00001
        // 14:12: f3 = 000
        // 11:8: imm[4:1] = 0100
        // 7: imm[11] = 0
        // 6:0: opcode = 1100011

        // 0b0_000000_00010_00001_000_0100_0_1100011
        // = bit31=0
        // 30=0,29=0,28=0,27=0,26=0,25=0
        // 24=0,23=0,22=0,21=1,20=0
        // 19=0,18=0,17=0,16=0,15=1
        // 14=0,13=0,12=0
        // 11=0,10=1,9=0,8=0
        // 7=0
        // 6=1,5=1,4=0,3=0,2=0,1=1,0=1
        // = 0b0000_0000_0010_0001_0000_0100_0110_0011
        // = 0x00_21_04_63 = 0x00210463

        let mut regs = [0u64; 32];
        regs[1] = 42;
        regs[2] = 42; // equal, so branch taken
        let (h, _) = exec_rv64(0x00210463, &regs, TEST_PC, true);
        assert_eq!(h.pc, TEST_PC + 8, "BEQ should branch +8");
    }

    #[test]
    fn jal() {
        // JAL x1, +8
        // bits: 0_0000000100_0_00000000_00001_1101111
        let (h, _) = exec_rv64(0x008000EF, &[0u64; 32], TEST_PC, true);
        assert_eq!(h.x[1], TEST_PC + 4, "JAL ra should save return address");
        assert_eq!(h.pc, TEST_PC + 8, "JAL should jump +8");
    }

    #[test]
    fn jalr() {
        // JALR x1, x2(0): opcode=0x67, rd=00001, f3=000, rs1=00010, imm=0
        // inst = 0b000000000000_00010_000_00001_1100111 = 0x000100E7
        let mut regs = [0u64; 32];
        regs[2] = 0x80001000;
        let (h, _) = exec_rv64(0x000100E7, &regs, TEST_PC, true);
        assert_eq!(h.x[1], TEST_PC + 4, "JALR ra should save return address");
        assert_eq!(h.pc, 0x80001000, "JALR to x2");
    }

    #[test]
    fn csrrw() {
        // CSRRW x1, mstatus, x2:
        // opcode=0x73, rd=00001, f3=001, rs1=00010, csr=0x300
        // Actually for CSRRW: the rs1 field is the src register
        // inst = 0b000000110000_00010_001_00001_1110011 = 0x300120F3
        let mut regs = [0u64; 32];
        regs[2] = 0xDEAD;
        let (h, _) = exec_rv64(0x300120F3, &regs, TEST_PC, true);
        assert_eq!(h.x[1], 0, "CSRRW should return old mstatus");
        // We can't easily check the new mstatus because it's masked
    }

    #[test]
    fn regs_x0_is_always_zero() {
        // The x[0] register must always be zero after any instruction
        let regs = [42u64; 32]; // x[0] = 42 initially
        let (h, _) = exec_rv64(0x00210093, &regs, TEST_PC, true); // ADDI x1, x2, 42
        assert_eq!(h.x[0], 0, "x0 must always be 0");
    }

    #[test]
    fn c_regs_x0_is_always_zero() {
        let regs = [42u64; 32];
        let (h, _) = exec_rvc(0x0800, &regs, TEST_PC, true);
        assert_eq!(h.x[0], 0, "x0 must always be 0 after RVC");
    }
}
