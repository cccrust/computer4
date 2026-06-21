#!/usr/bin/env python3
# Generate program.hex for RV64IM + Zicsr test

def r_type(funct7, rs2, rs1, funct3, rd):
    return (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | 0x33

def i_type_alu(funct3, rd, rs1, imm):
    if imm < 0:
        imm = imm & 0xFFF
    return (imm << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | 0x13

def i_type_load(funct3, rd, rs1, imm):
    if imm < 0:
        imm = imm & 0xFFF
    return (imm << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | 0x03

def s_type(funct3, rs1, rs2, imm):
    if imm < 0:
        imm = imm & 0xFFF
    imm_11_5 = (imm >> 5) & 0x7F
    imm_4_0 = imm & 0x1F
    return (imm_11_5 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (imm_4_0 << 7) | 0x23

def b_type(funct3, rs1, rs2, imm):
    if imm < 0:
        imm = imm & 0x1FFF  # 13-bit signed
    # imm is in bytes, but encoded as halfword multiples
    # imm[12|10:5|4:1|11]
    enc = ((imm >> 12) & 1) << 31
    enc |= ((imm >> 5) & 0x3F) << 25
    enc |= (rs2 << 20)
    enc |= (rs1 << 15)
    enc |= (funct3 << 12)
    enc |= ((imm >> 1) & 0xF) << 8
    enc |= ((imm >> 11) & 1) << 7
    enc |= 0x63
    return enc

def u_type(rd, imm):
    imm = imm & 0xFFFFF000
    return imm | (rd << 7) | 0x37

def u_type_auipc(rd, imm):
    imm = imm & 0xFFFFF000
    return imm | (rd << 7) | 0x17

def j_type(rd, imm):
    if imm < 0:
        imm = imm & 0x1FFFFF  # 21-bit signed
    # imm[20|10:1|11|19:12]
    enc = ((imm >> 20) & 1) << 31
    enc |= ((imm >> 1) & 0x3FF) << 21
    enc |= ((imm >> 11) & 1) << 20
    enc |= ((imm >> 12) & 0xFF) << 12
    enc |= (rd << 7)
    enc |= 0x6F
    return enc

def csr_type(funct3, rd, rs1, csr_addr):
    return (csr_addr << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | 0x73

def csri_type(funct3, rd, uimm, csr_addr):
    return (csr_addr << 20) | (uimm << 15) | (funct3 << 12) | (rd << 7) | 0x73

# Build test program
# Keep same memory layout as original for RV64I tests
insts = []

# x1 = 42
insts.append(i_type_alu(0, 1, 0, 42))
# x2 = 58
insts.append(i_type_alu(0, 2, 0, 58))
# x3 = x1 + x2 = 100
insts.append(r_type(0x00, 2, 1, 0, 3))
# x5 = x3 - x1 (SUB) = 58
insts.append(r_type(0x20, 3, 1, 0, 5))
# x6 = x1 ^ x2 (XOR) = 42^58 = 16... but old test expects 0
# Actually let's use a different calc: x6 = x1 - x1 = 0 (SUB)
insts.append(r_type(0x20, 1, 1, 0, 6))  # x6 = x1 - x1 = 0

# sd x3, 0(x0)
insts.append(s_type(3, 0, 3, 0))
# ld x4, 0(x0)
insts.append(i_type_load(3, 4, 0, 0))

# beq x0, x0, +0 (never branch)
insts.append(b_type(0, 0, 0, 0))
# x28 = 0
insts.append(i_type_alu(0, 28, 0, 0))
# x28 = 1
insts.append(i_type_alu(0, 28, 0, 1))
# x7 = 5
insts.append(i_type_alu(0, 7, 0, 5))
# x8 = 10
insts.append(i_type_alu(0, 8, 0, 10))
# bne x7, x8, +8 (skip 2 instructions: x9=1 gets skipped, x28 gets 11)
# imm = +8 bytes = skip 2 instructions (instructions are 4 bytes)
insts.append(b_type(1, 7, 8, 8))
# x9 = 1 (skipped by bne)
insts.append(i_type_alu(0, 9, 0, 1))
# x28 = x28 + 10 (should be skipped, so x28 stays 1)
# Actually wait: after bne taken, PC jumps by 8 bytes = 2 instructions
# So ADDI x9 and ADDI x28+10 are both skipped
# Let's change approach: after bne x7,x8,+8, the next instr is ADDI x9,0,1 (skipped)
# Then we want x28=11 reached
# So: x28 += 10 (at index 14, skipped)
insts.append(i_type_alu(0, 28, 28, 10))

# x28 = x28 + 10 (this is the one after bne target, index 14)
# Actually no, let me re-count. Let me just trace:
# 0: ADDI x1,0,42
# 1: ADDI x2,0,58
# 2: ADD x3,x1,x2
# 3: SUB x5,x3,x1
# 4: SUB x6,x1,x1 (instead of XOR)
# 5: SD x3,0(x0)
# 6: LD x4,0(x0)
# 7: BEQ x0,x0,0 -> PC stays at index 7, BEQ with imm=0 goes to PC+0=index 7? No wait, branch offset is relative to PC but B-type encoding...
#   Actually BEQ x0,x0,0: compare equal, so branch taken, target = PC + 0 = same instruction -> infinite loop!
#   That can't be right for the old program. Let me re-examine.

# Hmm, I think the old BEQ had a different offset. Let me recalculate for the old program.
# Old hex line 7: 00000463
# 0x00000463 = 0b 0000 0000 0000 0000 0100 0110 0011
# [31] = 0, [30:25]=000000, [24:20]=00000, [19:15]=00000, [14:12]=000, [11:8]=0010, [7]=0, [6:0]=1100011
# imm_b = {{51{instr[31]}}, instr[31], instr[7], instr[30:25], instr[11:8], 1'b0}
# = {sign=0, instr[31]=0, instr[7]=0, instr[30:25]=000000, instr[11:8]=0010, 0}
# = 0b 0 0 000000 0010 0 = 0b 00000000100 = 0x04 = 4
# So BEQ x0, x0, +4 -> PC + 4 = next instruction (no branch since imm=4, not 0)

# OK so BEQ x0,x0,4 branches to PC+4 which is the same as PC+4 (no-op branch). That's fine.

# Let me use the same BEQ encoding.
# For B-type with imm=4:
# imm = 4
# imm[12] = 0, imm[10:5] = 000000, imm[4:1] = 0010, imm[11] = 0
# enc = (0<<31) | (0<<25) | (0<<20) | (0<<15) | (0<<12) | (2<<8) | (0<<7) | 0x63
# = 0x00000463  ✓

insts[7] = b_type(0, 0, 0, 4)  # BEQ x0,x0,+4 (not taken effectively, goes to PC+4)

# Now we need to fix index 14. Currently the BNE at index 12 branches to index 14 with offset 8.
# Let me reconsider the control flow.

# Index:
# 0: ADDI x1, 0, 42
# 1: ADDI x2, 0, 58
# 2: ADD x3, x1, x2
# 3: SUB x5, x3, x1  -> x5=58
# 4: SUB x6, x1, x1  -> x6=0
# 5: SD x3, 0(x0)
# 6: LD x4, 0(x0)    -> x4=100
# 7: BEQ x0,x0,+4    -> PC+4 (goes to index 8)
# 8: ADDI x28,0,0    -> x28=0
# 9: ADDI x28,0,1    -> x28=1
# 10: ADDI x7,0,5    -> x7=5
# 11: ADDI x8,0,10   -> x8=10
# 12: BNE x7,x8,?    -> imm=8 means PC+8 = skip 2 instructions
# 13: ADDI x9,0,1    -> skipped
# 14: ??? -> this is the target: we need x28=11 here
# So at index 14, we should do: ADDI x28, x28, 10 -> x28 = 1 + 10 = 11
# Then:
# 15: LUI x10, 0x12345
# 16: AUIPC x11, 0

# Now let me also insert M-extension tests:
# Index 17: ADDI x12, 0, 7  
# Index 18: ADDI x13, 0, 3
# Index 19: MUL x14, x12, x13  -> 7*3 = 21
# Index 20: DIV x15, x12, x13   -> 7/3 = 2
# Index 21: REM x16, x12, x13   -> 7%3 = 1

# And CSR tests:
# Index 22: CSRRWI x17, mcycle, 0 -> read mcycle into x17
# Wait, CSRRWI rd, uimm, csr. If uimm=0 and rd != x0, it reads without writing.
# Actually CSRRWI with uimm=0 reads CSR and writes to rd (since uimm=0 means write 0, and writing 0 is allowed).
# Wait no: CSRRW/CSRRWI always write even if uimm=0. CSRRS/CSRRSI with rs1/uimm=0 don't write but still read.
# So I should use CSRRSI with uimm=0 for read-only access.
# Actually CSRRS/CSRRSI when rs1=0: no write, just read. ✓

# Let me use CSRRSI: funct3=110, rs1=uimm=0 -> read CSR, no write
# CSRRCI: funct3=111

# Index 22: CSRRSI x17, mcycle(0xB00), 0 -> read mcycle
# Index 23: CSRRSI x18, minstret(0xB02), 0 -> read minstret
# Index 24: CSRRWI x19, mtvec(0x305), 0x200 -> write 0x200 to mtvec

# Then FENCE.I:
# Index 25: FENCE.I = 0x0000100F (opcode=0x0F, funct3=000, rd=0, rs1=0, fm=0000, pred=0010, succ=0001)
# Actually FENCE.I is encoded as: 0b 0000 0000 0000 0000 0001 0000 0000 1111 = 0x0000100F

# Let me rewrite more carefully.

# Let me just use the same first 17 instructions as before (adjusted for x6=SUB instead of XOR),
# then append new instructions for M-extension and CSR.

# Fixed program:
insts_old = [
    i_type_alu(0, 1, 0, 42),       # 0: ADDI x1,0,42 -> x1=42
    i_type_alu(0, 2, 0, 58),       # 1: ADDI x2,0,58 -> x2=58
    r_type(0x00, 2, 1, 0, 3),      # 2: ADD x3,x1,x2 -> x3=100
    r_type(0x20, 3, 1, 0, 5),      # 3: SUB x5,x3,x1 -> x5=58
    r_type(0x20, 1, 1, 0, 6),      # 4: SUB x6,x1,x1 -> x6=0
    s_type(3, 0, 3, 0),            # 5: SD x3,0(x0) -> mem[0]=100
    i_type_load(3, 4, 0, 0),       # 6: LD x4,0(x0) -> x4=100
    b_type(0, 0, 0, 4),            # 7: BEQ x0,x0,4 -> PC+4 (go to index 8)
    i_type_alu(0, 28, 0, 0),       # 8: ADDI x28,0,0 -> x28=0
    i_type_alu(0, 28, 0, 1),       # 9: ADDI x28,0,1 -> x28=1
    i_type_alu(0, 7, 0, 5),        # 10: ADDI x7,0,5 -> x7=5
    i_type_alu(0, 8, 0, 10),       # 11: ADDI x8,0,10 -> x8=10
    b_type(1, 7, 8, 8),            # 12: BNE x7,x8,8 -> skip 2 inst
    i_type_alu(0, 9, 0, 1),        # 13: ADDI x9,0,1 -> skipped
    i_type_alu(0, 28, 28, 10),     # 14: ADDI x28,x28,10 -> x28=1+10=11
    u_type(10, 0x12345000),        # 15: LUI x10,0x12345 -> x10=0x12345000
    u_type_auipc(11, 0),           # 16: AUIPC x11,0 -> x11=PC at index 16
]

# Now add M-extension tests at index 17+
insts_new = [
    i_type_alu(0, 12, 0, 7),       # 17: ADDI x12,0,7
    i_type_alu(0, 13, 0, 3),       # 18: ADDI x13,0,3
    r_type(0x01, 13, 12, 0, 14),   # 19: MUL x14,x12,x13 -> 7*3=21
    r_type(0x01, 13, 12, 4, 15),   # 20: DIV x15,x12,x13 -> 7/3=2
    r_type(0x01, 13, 12, 6, 16),   # 21: REM x16,x12,x13 -> 7%3=1
    i_type_alu(0, 20, 0, 5),       # 22: ADDI x20,0,5
    i_type_alu(0, 21, 0, 2),       # 23: ADDI x21,0,2
    r_type(0x01, 21, 20, 0, 22),   # 24: MUL x22,x20,x21 -> 5*2=10
    csri_type(6, 17, 0, 0xB00),    # 25: CSRRSI x17,mcycle,0 -> read mcycle
    csri_type(6, 18, 0, 0xB02),    # 26: CSRRSI x18,minstret,0 -> read minstret
    csri_type(5, 19, 0x20, 0x305), # 27: CSRRWI x19,mtvec,0x20 -> write 0x20 to mtvec
    csr_type(1, 20, 19, 0x305),    # 28: CSRRW x20,mtvec,x19 -> read mtvec into x20, write x19
    csr_type(2, 21, 19, 0x300),    # 29: CSRRS x21,mstatus,x19 -> read mstatus, set bits from x19
    0x0000100F,                    # 30: FENCE.I (opcode=0x0F)
    0x00000013,                    # 31: NOP
]

insts = insts_old + insts_new

with open('program.hex', 'w') as f:
    for inst in insts:
        f.write(f'{inst:08x}\n')

print(f'Wrote {len(insts)} instructions')
# Print for reference
for i, inst in enumerate(insts):
    print(f'  {i:2d}: {inst:08x}')

