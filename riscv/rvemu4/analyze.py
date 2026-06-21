#!/usr/bin/env python3
"""Analyze RISC-V compressed instruction encoding by comparing assembler output."""

# Data from objdump: (mnemonic, operands, encoding)
instructions = [
    # CL format - loads (q=0)
    ("lw", "s0,0(s0)",  0x4000),
    ("lw", "s0,4(s0)",  0x4040),
    ("lw", "s0,8(s0)",  0x4400),
    ("lw", "s0,12(s0)", 0x4440),
    ("lw", "s0,16(s0)", 0x4800),
    ("lw", "s0,20(s0)", 0x4840),
    ("lw", "s0,24(s0)", 0x4c00),
    ("lw", "s0,28(s0)", 0x4c40),
    ("lw", "s0,32(s0)", 0x5000),
    ("lw", "s0,64(s0)", 0x4020),
    ("lw", "s0,124(s0)", 0x5c60),
    # CL format - loads (q=0, f3=3)
    ("ld", "s0,0(s0)",  0x6000),
    ("ld", "s0,8(s0)",  0x6400),
    ("ld", "s0,16(s0)", 0x6800),
    ("ld", "s0,24(s0)", 0x6c00),
    ("ld", "s0,32(s0)", 0x7000),
    ("ld", "s0,40(s0)", 0x7400),
    ("ld", "s0,64(s0)", 0x6020),
    ("ld", "s0,128(s0)", 0x6040),
    # CL format - stores (q=0, f3=6)
    ("sw", "s0,0(s0)",  0xc000),
    ("sw", "s0,8(s0)",  0xc400),
    ("sw", "s0,16(s0)", 0xc800),
    ("sw", "s0,64(s0)", 0xc020),
    # CL format - stores (q=0, f3=7)
    ("sd", "s0,0(s0)",  0xe000),
    ("sd", "s0,8(s0)",  0xe400),
    ("sd", "s0,16(s0)", 0xe800),
    ("sd", "s0,64(s0)", 0xe020),
    ("sd", "s0,128(s0)", 0xe040),
    # CSS format - stores (q=2)
    ("sw", "ra,0(sp)",  0xc006),
    ("sw", "ra,4(sp)",  0xc206),
    ("sw", "ra,8(sp)",  0xc406),
    ("sw", "ra,12(sp)", 0xc606),
    ("sw", "ra,16(sp)", 0xc806),
    ("sw", "ra,20(sp)", 0xca06),
    ("sw", "ra,24(sp)", 0xcc06),
    ("sw", "ra,64(sp)", 0xc086),
    ("sw", "ra,124(sp)", 0xde86),
    # CSS format - stores (q=2, f3=7)
    ("sd", "ra,0(sp)",  0xe006),
    ("sd", "ra,8(sp)",  0xe406),
    ("sd", "ra,16(sp)", 0xe806),
    ("sd", "ra,24(sp)", 0xec06),
    ("sd", "ra,32(sp)", 0xf006),
    ("sd", "ra,128(sp)", 0xe106),
    ("sd", "ra,248(sp)", 0xfd86),
    # CIW format - loads (q=2)
    ("lw", "ra,0(sp)",  0x4082),
    ("lw", "ra,4(sp)",  0x4092),
    ("lw", "ra,8(sp)",  0x40a2),
    ("lw", "ra,12(sp)", 0x40b2),
    # CIW format - loads (q=2, f3=3)
    ("ld", "ra,0(sp)",  0x6082),
    ("ld", "ra,8(sp)",  0x60a2),
    ("ld", "ra,16(sp)", 0x60c2),
    ("ld", "ra,128(sp)", 0x608a),
]

def extract_offset(op_str):
    """Extract the numeric offset from e.g. 'ra,4(sp)' or 's0,0(s0)'"""
    paren = op_str.find('(')
    before = op_str[:paren]
    comma = before.find(',')
    off_str = before[comma+1:].strip()
    if not off_str:
        return 0
    return int(off_str)

def decode_inst(enc):
    """Decode instruction fields."""
    q = enc & 3
    f3 = (enc >> 13) & 7
    rs1 = (enc >> 7) & 0x1f
    rs2 = (enc >> 2) & 0x1f
    rd = (enc >> 7) & 0x1f
    
    # CL format bits
    b12 = (enc >> 12) & 1
    b11 = (enc >> 11) & 1
    b10 = (enc >> 10) & 1
    b9 = (enc >> 9) & 1
    b8 = (enc >> 8) & 1
    b7 = (enc >> 7) & 1
    b6 = (enc >> 6) & 1
    b5 = (enc >> 5) & 1
    b4 = (enc >> 4) & 1
    b3 = (enc >> 3) & 1
    b2 = (enc >> 2) & 1
    
    return q, f3, b12, b11, b10, b9, b8, b7, b6, b5, b4, b3, b2

print("=" * 120)
print(f"{'Inst':30s} {'enc':6s} {'q':2s} {'f3':2s} {'bits[12:7]':14s} {'off':5s}")
print("=" * 120)

for mnemonic, ops, enc in instructions:
    offset = extract_offset(ops)
    q, f3, b12, b11, b10, b9, b8, b7, b6, b5, b4, b3, b2 = decode_inst(enc)
    bits_str = f"{b12}{b11}{b10}{b9}{b8}{b7}"
    
    # Format: "mnemonic reg, offset(sp)"
    parts = ops.split(',')
    reg = parts[0]
    full_ops = ops
    
    print(f"{mnemonic:4s} {full_ops:24s} {enc:06x} {q:2d} {f3:2d} {bits_str:14s} {offset:5d}")
    
    # For CL format (q=0)
    if q == 0:
        print(f"  CL-LOAD uimm={b5}{b12}{b11}{b10}{b6} = {(b5<<4)|(b12<<3)|(b11<<2)|(b10<<1)|b6:3d}  off={((b5<<4)|(b12<<3)|(b11<<2)|(b10<<1)|b6)<<2}")
        print(f"  CL-STORE uimm={b6}{b5}{b12}{b11}{b10} = {(b6<<4)|(b5<<3)|(b12<<2)|(b11<<1)|b10:3d}  off={((b6<<4)|(b5<<3)|(b12<<2)|(b11<<1)|b10)<<3}")
    
    # For CIW format - C.LWSP (q=2, f3=2)
    if q == 2 and f3 == 2:
        uimm_old = (b4 << 5) | (b3 << 4) | (b2 << 3) | (b12 << 2) | (b6 << 1) | b5
        uimm_new = (b12 << 5) | (b6 << 4) | (b5 << 3) | (b4 << 2) | (b3 << 1) | b2
        print(f"  CIW-LW old=({b4}{b3}{b2}{b12}{b6}{b5}={uimm_old:3d},off={uimm_old<<2:4d})  new=({b12}{b6}{b5}{b4}{b3}{b2}={uimm_new:3d},off={uimm_new:4d})")
    
    # For CSS format - C.SWSP (q=2, f3=6)
    if q == 2 and f3 == 6:
        spec = (b12 << 5) | (b11 << 4) | (b10 << 3) | (b9 << 2) | (b8 << 1) | b7
        old = (b9 << 5) | (b8 << 4) | (b7 << 3) | (b12 << 2) | (b11 << 1) | b10
        new = (b12 << 5) | (b11 << 4) | (b10 << 3) | (b9 << 2) | (b8 << 1) | b7
        rev = (b7 << 5) | (b8 << 4) | (b9 << 3) | (b10 << 2) | (b11 << 1) | b12
        print(f"  CSS-SW spec={spec:3d} | old={old:3d}(off={old<<2:4d}) | new={new:3d} | rev={rev:3d}(off={rev<<1:4d})")
    
    # For CSS format - C.SDSP (q=2, f3=7)
    if q == 2 and f3 == 7:
        spec = (b12 << 5) | (b11 << 4) | (b10 << 3) | (b9 << 2) | (b8 << 1) | b7
        old = (b9 << 5) | (b8 << 4) | (b7 << 3) | (b12 << 2) | (b11 << 1) | b10
        new = (b12 << 5) | (b11 << 4) | (b10 << 3) | (b9 << 2) | (b8 << 1) | b7
        rev = (b7 << 5) | (b8 << 4) | (b9 << 3) | (b10 << 2) | (b11 << 1) | b12
        print(f"  CSS-SD spec={spec:3d} | old={old:3d}(off={old<<3:4d}) | new={new:3d} | rev={rev:3d}(off={rev<<3:4d})")
    
    # For CIW - C.LDSP (q=2, f3=3)
    if q == 2 and f3 == 3:
        uimm_new = (b12 << 5) | (b6 << 4) | (b5 << 3) | (b4 << 2) | (b3 << 1) | b2
        uimm_old = (b4 << 5) | (b3 << 4) | (b2 << 3) | (b12 << 2) | (b6 << 1) | b5
        print(f"  CIW-LD old=({b4}{b3}{b2}{b12}{b6}{b5}={uimm_old:3d},off={uimm_old<<3:4d})  new=({b12}{b6}{b5}{b4}{b3}{b2}={uimm_new:3d},off={uimm_new:4d})")

print()

# Now check the correct formula for CL format
print("\n--- CL LOAD format verification ---")
for mnemonic, ops, enc in instructions:
    if enc & 3 != 0:  # not q=0 (CL format)
        continue
    q, f3, b12, b11, b10, b9, b8, b7, b6, b5, b4, b3, b2 = decode_inst(enc)
    offset = extract_offset(ops)
    
    # CL load formula: uimm = {b5,b12,b11,b10,b6}, off = uimm << 2
    load_uimm = (b5<<4)|(b12<<3)|(b11<<2)|(b10<<1)|b6
    load_off = load_uimm << 2
    
    # CL store formula: uimm = {b6,b5,b12,b11,b10}, off = uimm << 3
    store_uimm = (b6<<4)|(b5<<3)|(b12<<2)|(b11<<1)|b10
    store_off = store_uimm << 3
    
    is_f3_load = f3 == 2 or f3 == 3
    is_f3_store = f3 == 6 or f3 == 7
    
    if is_f3_load:
        correct = "OK" if load_off == offset else f"X({load_off})"
        print(f"  LOAD: {mnemonic} {ops:24s} off={offset:4d}  uimm={load_uimm:3d}({b12}{b11}{b10}{b6}{b5})  load_off={load_off:4d} {correct}")
    if is_f3_store:
        correct = "OK" if store_off == offset else f"X({store_off})"
        print(f"  STORE:{mnemonic} {ops:24s} off={offset:4d}  uimm={store_uimm:3d}({b6}{b5}{b12}{b11}{b10})  store_off={store_off:4d} {correct}")

print("\n--- CSS format verification ---")
for mnemonic, ops, enc in instructions:
    if enc & 3 != 2:  # not q=2
        continue
    q, f3, b12, b11, b10, b9, b8, b7, b6, b5, b4, b3, b2 = decode_inst(enc)
    offset = extract_offset(ops)
    
    # For SWSP and SDSP
    if f3 == 6:  # C.SWSP
        new_off = (b12<<5)|(b11<<4)|(b10<<3)|(b9<<2)|(b8<<1)|b7
        old_off = ((b9<<5)|(b8<<4)|(b7<<3)|(b12<<2)|(b11<<1)|b10) << 2
        rev = ((b7<<5)|(b8<<4)|(b9<<3)|(b10<<2)|(b11<<1)|b12) << 1
        print(f"  SWSP {mnemonic} {ops:24s} off={offset:4d}  new={new_off:4d} old={old_off:4d} rev={rev:4d}")
    
    if f3 == 7:  # C.SDSP
        new_off = (b12<<5)|(b11<<4)|(b10<<3)|(b9<<2)|(b8<<1)|b7
        old_off = ((b9<<5)|(b8<<4)|(b7<<3)|(b12<<2)|(b11<<1)|b10) << 3
        rev = ((b7<<5)|(b8<<4)|(b9<<3)|(b10<<2)|(b11<<1)|b12) << 3
        print(f"  SDSP {mnemonic} {ops:24s} off={offset:4d}  new={new_off:4d} old={old_off:4d} rev={rev:4d}")
