use crate::memory::Bus;

const PRV_U: u8 = 0; const PRV_S: u8 = 1; const PRV_M: u8 = 3;

const EXC_ILLEGAL_INST: u64 = 2; const EXC_BREAKPOINT: u64 = 3;
const EXC_ECALL_U: u64 = 8; const EXC_ECALL_S: u64 = 9; const EXC_ECALL_M: u64 = 11;
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

const SSTATUS_MASK: u64 = 0x800DE162;

pub struct Hart {
    pub x: [u64; 32], pub pc: u64, pub priv_level: u8, pub is_64: bool,
    pub mstatus: u64, pub mie: u64, pub mip: u64,
    pub medeleg: u64, pub mideleg: u64,
    pub mepc: u64, pub mcause: u64, pub mtval: u64, pub mtvec: u64, pub mscratch: u64,
    pub mcounteren: u64, pub menvcfg: u64, pub mhartid: u64,
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
        let mut a = pt_base; let mut pte = 0u64; let mut lvl: i32 = 2;
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
        let inst_raw = match self.vm_fetch(bus, self.pc, false) { Ok(v) => v as u32, Err(c) => { self.trap(c, self.pc); return true; } };
        if (inst_raw & 0x3) != 0x3 { return self.exec_rvc(bus, inst_raw as u16); }

        let (rd, rs1, rs2, f3, f7) = (
            ((inst_raw >> 7) & 0x1F) as usize, ((inst_raw >> 15) & 0x1F) as usize,
            ((inst_raw >> 20) & 0x1F) as usize, (inst_raw >> 12) & 0x7, (inst_raw >> 25) & 0x7F);
        let imm_i = Self::sext(inst_raw as u64 >> 20, 12);
        let imm_s = Self::sext((((inst_raw as u64 >> 25) << 5) | ((inst_raw >> 7) & 0x1F) as u64), 12);
        let imm_b = Self::sext((((inst_raw as u64 >> 31) << 12) | ((inst_raw as u64 >> 7) & 1) << 11 | (((inst_raw >> 25) & 0x3F) as u64) << 5 | (((inst_raw >> 8) & 0xF) as u64) << 1), 13);
        let imm_u = Self::sext(inst_raw as u64 & 0xFFFFF000, 32);
        let imm_j = Self::sext((((inst_raw as u64 >> 31) << 20) | ((inst_raw as u64 >> 12) & 0xFF) << 12 | ((inst_raw as u64 >> 20) & 1) << 11 | (((inst_raw >> 21) & 0x3FF) as u64) << 1), 21);

        let mut npc = self.pc.wrapping_add(4);
        let r1 = self.x[rs1]; let r2 = self.x[rs2];
        macro_rules! wr { ($r:expr,$v:expr) => { if $r != 0 { self.x[$r] = if self.is_64 { $v } else { ($v as u32) as u64 }; } } }

        match inst_raw & 0x7F {
            0x37 => wr!(rd, imm_u as u64),
            0x17 => wr!(rd, self.pc.wrapping_add(imm_u as u64)),
            0x6F => { wr!(rd, npc); npc = self.pc.wrapping_add(imm_j as u64); }
            0x67 => { wr!(rd, npc); npc = (r1.wrapping_add(imm_i as u64)) & !1; }
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
                    Ok(val) => wr!(rd, match f3 { 0 => val as i8 as i64 as u64, 1 => val as i16 as i64 as u64, 2 => val as i32 as i64 as u64, 3 => val, 4 => val & 0xFF, 5 => val & 0xFFFF, 6 => val & 0xFFFFFFFF, _ => val }),
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
                } else if inst_raw == 0x10500073 || inst_raw == 0x12000073 {}
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
                    2 => wr!(rd, val),
                    3 => { if self.vm_write(bus, addr, r2, sz).is_err() { self.trap(EXC_STORE_PAGE_FAULT, addr); return true; } wr!(rd, 0); }
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
        let rs1_s = rd_s;
        let rs2_s = ((i >> 7) & 0x7) as usize + 8;
        let mut npc = self.pc.wrapping_add(2);

        macro_rules! wr { ($r:expr,$v:expr) => { if $r != 0 { self.x[$r] = if self.is_64 { $v } else { ($v as u32) as u64 }; } } }

        match q {
            0 => match f3 {
                0 => {
                    let nzu = ((i >> 4) as u64 & 3) << 4 | ((i >> 5) as u64 & 1) << 3 | ((i >> 6) as u64 & 1) << 2
                        | ((i >> 7) as u64 & 1) << 6 | ((i >> 8) as u64 & 1) << 5 | ((i >> 9) as u64 & 1) << 1;
                    if nzu != 0 { self.x[rd_s] = self.x[2].wrapping_add(nzu); }
                }
                2 => {
                    let off = ((i >> 5) as u64 & 3) << 6 | ((i >> 6) as u64 & 1) << 2 | ((i >> 10) as u64 & 3) << 4 | ((i >> 12) as u64 & 1) << 3;
                    let addr = self.x[rs1_s].wrapping_add(off);
                    match self.vm_read(bus, addr, 4) { Ok(v) => wr!(rd_s, v as i32 as i64 as u64), Err(c) => { self.trap(c, addr); return true; } }
                }
                3 => {
                    if self.is_64 {
                        let off = ((i >> 5) as u64 & 3) << 6 | ((i >> 6) as u64 & 1) << 2 | ((i >> 10) as u64 & 3) << 4 | ((i >> 12) as u64 & 1) << 3;
                        let addr = self.x[rs1_s].wrapping_add(off);
                        match self.vm_read(bus, addr, 8) { Ok(v) => wr!(rd_s, v), Err(c) => { self.trap(c, addr); return true; } }
                    } else { self.trap(EXC_ILLEGAL_INST, i as u64); return true; }
                }
                6 => {
                    let off = ((i >> 5) as u64 & 3) << 6 | ((i >> 6) as u64 & 1) << 2 | ((i >> 10) as u64 & 3) << 4 | ((i >> 12) as u64 & 1) << 3;
                    let addr = self.x[rs1_s].wrapping_add(off);
                    if self.vm_write(bus, addr, self.x[rs2_s] as u32 as u64, 4).is_err() { self.trap(EXC_STORE_PAGE_FAULT, addr); return true; }
                }
                7 => {
                    if self.is_64 {
                        let off = ((i >> 5) as u64 & 3) << 6 | ((i >> 6) as u64 & 1) << 2 | ((i >> 10) as u64 & 3) << 4 | ((i >> 12) as u64 & 1) << 3;
                        let addr = self.x[rs1_s].wrapping_add(off);
                        if self.vm_write(bus, addr, self.x[rs2_s], 8).is_err() { self.trap(EXC_STORE_PAGE_FAULT, addr); return true; }
                    } else { self.trap(EXC_ILLEGAL_INST, i as u64); return true; }
                }
                _ => { self.trap(EXC_ILLEGAL_INST, i as u64); return true; }
            },
            1 => match f3 {
                0 => {
                    let imm = Self::sext((((i >> 7) as u64 & 0x1F) | ((i as u64 >> 12) & 1) << 5), 6) as u64;
                    if rs1 != 0 { wr!(rs1, self.x[rs1].wrapping_add(imm)); }
                }
                1 => {
                    if self.is_64 {
                        let imm = Self::sext((((i >> 7) as u64 & 0x1F) | ((i as u64 >> 12) & 1) << 5), 6) as u64;
                        if rs1 != 0 { wr!(rs1, (self.x[rs1].wrapping_add(imm)) as i32 as i64 as u64); }
                    } else { self.trap(EXC_ILLEGAL_INST, i as u64); return true; }
                }
                2 => {
                    let imm = Self::sext((((i >> 7) as u64 & 0x1F) | ((i as u64 >> 12) & 1) << 5), 6) as u64;
                    if rs1 != 0 { wr!(rs1, imm); }
                }
                3 => {
                    if rs1 == 2 {
                        let imm = Self::sext(((((i >> 2) as u64 & 0x1F) | ((i >> 7) as u64 & 1) << 5 | ((i >> 8) as u64 & 3) << 6 | ((i >> 12) as u64 & 1) << 4)), 9) as u64;
                        self.x[2] = self.x[2].wrapping_add(imm);
                    } else if rs1 != 0 {
                        let imm = Self::sext((((i >> 7) as u64 & 0x1F) | ((i >> 12) as u64 & 1) << 5), 6) as u64;
                        wr!(rs1, imm << 12);
                    }
                }
                4 => {
                    let rd_s = ((i >> 2) & 0x7) as usize + 8;
                    let sh = (i >> 7) as u64 & 0x1F | ((i >> 12) as u64 & 1) << 5;
                    if (i >> 11) & 1 == 0 {
                        self.x[rd_s] >>= sh;
                    } else {
                        self.x[rd_s] = ((self.x[rd_s] as i64) >> sh) as u64;
                    }
                }
                5 => {
                    let imm = ((i >> 2) as u64 & 0x7) << 1 | ((i >> 3) as u64 & 1) << 8 | ((i >> 4) as u64 & 1) << 9
                        | ((i >> 5) as u64 & 3) << 6 | ((i >> 7) as u64 & 1) << 10 | ((i >> 8) as u64 & 1) << 2
                        | ((i >> 9) as u64 & 1) << 3 | ((i >> 10) as u64 & 1) << 7 | ((i >> 11) as u64 & 1) << 4
                        | ((i >> 12) as u64 & 1) << 11;
                    let jimm = Self::sext(imm, 12) as u64;
                    npc = self.pc.wrapping_add(jimm);
                }
                6 => {
                    let rs1_s = ((i >> 7) & 0x7) as usize + 8;
                    let off = ((i >> 2) as u64 & 0x7) << 1 | ((i >> 3) as u64 & 1) << 5 | ((i >> 4) as u64 & 1) << 2
                        | ((i >> 5) as u64 & 3) << 3 | ((i >> 10) as u64 & 1) << 6 | ((i >> 11) as u64 & 1) << 7
                        | ((i >> 12) as u64 & 1) << 8;
                    let bimm = Self::sext(off, 9) as u64;
                    if self.x[rs1_s] == 0 { npc = self.pc.wrapping_add(bimm); }
                }
                7 => {
                    let rs1_s = ((i >> 7) & 0x7) as usize + 8;
                    let off = ((i >> 2) as u64 & 0x7) << 1 | ((i >> 3) as u64 & 1) << 5 | ((i >> 4) as u64 & 1) << 2
                        | ((i >> 5) as u64 & 3) << 3 | ((i >> 10) as u64 & 1) << 6 | ((i >> 11) as u64 & 1) << 7
                        | ((i >> 12) as u64 & 1) << 8;
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
                    let off = ((i >> 2) as u64 & 3) << 2 | ((i >> 3) as u64 & 1) << 5 | ((i >> 4) as u64 & 1) << 6
                        | ((i >> 5) as u64 & 3) << 3 | ((i >> 12) as u64 & 1) << 4;
                    let addr = self.x[2].wrapping_add(off);
                    match self.vm_read(bus, addr, 4) { Ok(v) => if rs1 != 0 { wr!(rs1, v as i32 as i64 as u64); }, Err(c) => { self.trap(c, addr); return true; } }
                }
                3 => {
                    if self.is_64 {
                        let off = ((i >> 2) as u64 & 3) << 3 | ((i >> 3) as u64 & 1) << 5 | ((i >> 4) as u64 & 1) << 6
                            | ((i >> 5) as u64 & 3) << 4 | ((i >> 12) as u64 & 1);
                        let addr = self.x[2].wrapping_add(off);
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
                    let off = ((i >> 2) as u64 & 3) << 2 | ((i >> 3) as u64 & 1) << 5 | ((i >> 4) as u64 & 1) << 6
                        | ((i >> 5) as u64 & 3) << 3 | ((i >> 6) as u64 & 1) << 7 | ((i >> 12) as u64 & 1) << 4;
                    let addr = self.x[2].wrapping_add(off);
                    if self.vm_write(bus, addr, self.x[rs2] as u32 as u64, 4).is_err() { self.trap(EXC_STORE_PAGE_FAULT, addr); return true; }
                }
                7 => {
                    if self.is_64 {
                        let off = ((i >> 2) as u64 & 3) << 3 | ((i >> 3) as u64 & 1) << 5 | ((i >> 4) as u64 & 1) << 6
                            | ((i >> 5) as u64 & 3) << 4 | ((i >> 6) as u64 & 1) << 7 | ((i >> 12) as u64 & 1);
                        let addr = self.x[2].wrapping_add(off);
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
