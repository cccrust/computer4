// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Tracing implementation internals
#include "verilated_vcd_c.h"
#include "Vtb_rv64i__Syms.h"


VL_ATTR_COLD void Vtb_rv64i___024root__trace_init_sub__TOP__0(Vtb_rv64i___024root* vlSelf, VerilatedVcd* tracep) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root__trace_init_sub__TOP__0\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    const int c = vlSymsp->__Vm_baseCode;
    // Body
    tracep->pushPrefix("tb_rv64i", VerilatedTracePrefixType::SCOPE_MODULE);
    tracep->declBit(c+226,0,"clk",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+227,0,"rst_n",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+98,0,"dbg_ecall",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBus(c+228,0,"i",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::INTEGER, false,-1, 31,0);
    tracep->declBus(c+87,0,"j",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::INTEGER, false,-1, 31,0);
    tracep->declBus(c+88,0,"word_val",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::INTEGER, false,-1, 31,0);
    tracep->declBus(c+89,0,"byte_val",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 7,0);
    tracep->declBit(c+229,0,"term",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+230,0,"uart_active",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+231,0,"trace_enabled",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declQuad(c+90,0,"sa0",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+92,0,"sa7",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declBus(c+1,0,"trace_cnt",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::INTEGER, false,-1, 31,0);
    tracep->pushPrefix("cpu", VerilatedTracePrefixType::SCOPE_MODULE);
    tracep->declQuad(c+241,0,"BASE_ADDR",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::PARAMETER, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declBit(c+226,0,"clk",-1, VerilatedTraceSigDirection::INPUT, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+227,0,"rst_n",-1, VerilatedTraceSigDirection::INPUT, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+98,0,"dbg_ecall",-1, VerilatedTraceSigDirection::OUTPUT, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declQuad(c+94,0,"pc",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+99,0,"pc_inc",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+232,0,"pc_next",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declBus(c+96,0,"word_addr",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 13,0);
    tracep->declBit(c+97,0,"half_sel",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBus(c+234,0,"lower_half",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 15,0);
    tracep->declBus(c+101,0,"upper_half",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 15,0);
    tracep->declBus(c+102,0,"fetch_half",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 15,0);
    tracep->declBit(c+103,0,"is_compressed",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBus(c+104,0,"instr",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 31,0);
    tracep->declBus(c+243,0,"init_i",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::INTEGER, false,-1, 31,0);
    tracep->declBus(c+105,0,"opcode",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 6,0);
    tracep->declBus(c+106,0,"rd",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 4,0);
    tracep->declBus(c+107,0,"funct3",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 2,0);
    tracep->declBus(c+108,0,"rs1",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 4,0);
    tracep->declBus(c+109,0,"rs2",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 4,0);
    tracep->declBus(c+110,0,"funct7",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 6,0);
    tracep->declBus(c+111,0,"funct5",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 4,0);
    tracep->declBit(c+112,0,"is_lr",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+113,0,"is_sc",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBus(c+114,0,"rvc_q",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 1,0);
    tracep->declBus(c+115,0,"rvc_funct3",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 2,0);
    tracep->declBit(c+116,0,"rvc_bit12",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBus(c+117,0,"rvc_rd",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 4,0);
    tracep->declBus(c+118,0,"rvc_rs2",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 4,0);
    tracep->declBus(c+119,0,"rvc_rdq",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 2,0);
    tracep->declBus(c+120,0,"rvc_rs1q",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 2,0);
    tracep->declBus(c+119,0,"rvc_rs2q",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 2,0);
    tracep->declBus(c+121,0,"rvc_rd5",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 4,0);
    tracep->declBus(c+122,0,"rvc_rs15",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 4,0);
    tracep->declBus(c+123,0,"rvc_rs25",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 4,0);
    tracep->declBus(c+124,0,"rvc_imm_i6",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 5,0);
    tracep->declQuad(c+125,0,"rvc_imm",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declBus(c+127,0,"rvc_imm_j11",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 11,0);
    tracep->declQuad(c+128,0,"rvc_imm_j",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declBus(c+130,0,"rvc_imm_b9",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 8,0);
    tracep->declQuad(c+131,0,"rvc_imm_b",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declBus(c+133,0,"rvc_nzuimm10",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 9,0);
    tracep->declQuad(c+134,0,"rvc_imm_addi4spn",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declBus(c+136,0,"rvc_uimm_ld",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 7,0);
    tracep->declQuad(c+137,0,"rvc_imm_ld",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+137,0,"rvc_imm_sd",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declBus(c+139,0,"rvc_uimm_lw",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 6,0);
    tracep->declQuad(c+140,0,"rvc_imm_lw",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+140,0,"rvc_imm_sw",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declBus(c+142,0,"rvc_imm_addi16sp_val",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 9,0);
    tracep->declQuad(c+143,0,"rvc_imm_addi16sp",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+145,0,"rvc_imm_lui",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declBus(c+124,0,"rvc_uimm_sp_ld",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 5,0);
    tracep->declQuad(c+147,0,"rvc_imm_sp_ld",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declBus(c+149,0,"rvc_uimm_sp_st",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 5,0);
    tracep->declQuad(c+150,0,"rvc_imm_sp_st",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->pushPrefix("rf", VerilatedTracePrefixType::ARRAY_UNPACKED);
    for (int i = 0; i < 32; ++i) {
        tracep->declQuad(c+2+i*2,0,"",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, true,(i+0), 63,0);
    }
    tracep->popPrefix();
    tracep->declQuad(c+152,0,"rf_rdata1",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+154,0,"rf_rdata2",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declBus(c+156,0,"data_word_addr",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 13,0);
    tracep->declBus(c+157,0,"byte_lane",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 1,0);
    tracep->declBus(c+158,0,"data_word",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 31,0);
    tracep->declBit(c+159,0,"is_sd",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+160,0,"is_ram_addr",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+161,0,"is_uart_mmio",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+162,0,"is_clint_mmio",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+163,0,"is_mmio",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declQuad(c+164,0,"imm_i",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+166,0,"imm_s",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+168,0,"imm_b",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+170,0,"imm_u",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+172,0,"imm_j",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+174,0,"imm_z",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+66,0,"mcycle",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+68,0,"minstret",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+70,0,"mstatus",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+72,0,"mie",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+74,0,"mtvec",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+76,0,"mscratch",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+78,0,"mepc",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+80,0,"mcause",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+82,0,"clint_mtime",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+84,0,"clint_mtimecmp",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declBit(c+86,0,"timer_irq",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declQuad(c+176,0,"csr_rdata",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declBit(c+178,0,"csr_write",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declQuad(c+179,0,"csr_wdata",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declBus(c+181,0,"funct12",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 11,0);
    tracep->declBit(c+98,0,"is_ecall",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+182,0,"is_mret",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+183,0,"is_wfi",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+184,0,"reg_write",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+185,0,"mem_write",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+186,0,"branch",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+187,0,"jump",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+188,0,"jalr",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBus(c+189,0,"reg_src",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 1,0);
    tracep->declBit(c+190,0,"alu_src_a",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBus(c+191,0,"alu_src_b",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 1,0);
    tracep->declBit(c+192,0,"csr_op",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+193,0,"csr_imm",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declQuad(c+194,0,"alu_b",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declBus(c+196,0,"alu_ctrl",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 3,0);
    tracep->declQuad(c+197,0,"alu_a",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+199,0,"rvc_alu_imm",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+201,0,"csr_rs1_val",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+203,0,"alu_result",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declArray(c+205,0,"mul_full_s",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 127,0);
    tracep->declArray(c+209,0,"mul_full_u",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 127,0);
    tracep->declArray(c+213,0,"mul_full_su",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 127,0);
    tracep->declBit(c+217,0,"branch_cond",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declBit(c+218,0,"branch_taken",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1);
    tracep->declQuad(c+235,0,"jal_target",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+219,0,"jalr_target",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+237,0,"pc_regular",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::WIRE, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+221,0,"mmio_rdata",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+223,0,"mem_rdata_word",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declQuad(c+239,0,"reg_wdata",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 63,0);
    tracep->declBus(c+225,0,"mem_wdata",-1, VerilatedTraceSigDirection::NONE, VerilatedTraceSigKind::VAR, VerilatedTraceSigType::LOGIC, false,-1, 31,0);
    tracep->popPrefix();
    tracep->popPrefix();
}

VL_ATTR_COLD void Vtb_rv64i___024root__trace_init_top(Vtb_rv64i___024root* vlSelf, VerilatedVcd* tracep) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root__trace_init_top\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    Vtb_rv64i___024root__trace_init_sub__TOP__0(vlSelf, tracep);
}

VL_ATTR_COLD void Vtb_rv64i___024root__trace_const_0(void* voidSelf, VerilatedVcd::Buffer* bufp);
VL_ATTR_COLD void Vtb_rv64i___024root__trace_full_0(void* voidSelf, VerilatedVcd::Buffer* bufp);
void Vtb_rv64i___024root__trace_chg_0(void* voidSelf, VerilatedVcd::Buffer* bufp);
void Vtb_rv64i___024root__trace_cleanup(void* voidSelf, VerilatedVcd* /*unused*/);

VL_ATTR_COLD void Vtb_rv64i___024root__trace_register(Vtb_rv64i___024root* vlSelf, VerilatedVcd* tracep) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root__trace_register\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    tracep->addConstCb(&Vtb_rv64i___024root__trace_const_0, 0U, vlSelf);
    tracep->addFullCb(&Vtb_rv64i___024root__trace_full_0, 0U, vlSelf);
    tracep->addChgCb(&Vtb_rv64i___024root__trace_chg_0, 0U, vlSelf);
    tracep->addCleanupCb(&Vtb_rv64i___024root__trace_cleanup, vlSelf);
}

VL_ATTR_COLD void Vtb_rv64i___024root__trace_const_0_sub_0(Vtb_rv64i___024root* vlSelf, VerilatedVcd::Buffer* bufp);

VL_ATTR_COLD void Vtb_rv64i___024root__trace_const_0(void* voidSelf, VerilatedVcd::Buffer* bufp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root__trace_const_0\n"); );
    // Init
    Vtb_rv64i___024root* const __restrict vlSelf VL_ATTR_UNUSED = static_cast<Vtb_rv64i___024root*>(voidSelf);
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    // Body
    Vtb_rv64i___024root__trace_const_0_sub_0((&vlSymsp->TOP), bufp);
}

VL_ATTR_COLD void Vtb_rv64i___024root__trace_const_0_sub_0(Vtb_rv64i___024root* vlSelf, VerilatedVcd::Buffer* bufp) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root__trace_const_0_sub_0\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    uint32_t* const oldp VL_ATTR_UNUSED = bufp->oldp(vlSymsp->__Vm_baseCode);
    // Body
    bufp->fullQData(oldp+241,(0x80000000ULL),64);
    bufp->fullIData(oldp+243,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__init_i),32);
}

VL_ATTR_COLD void Vtb_rv64i___024root__trace_full_0_sub_0(Vtb_rv64i___024root* vlSelf, VerilatedVcd::Buffer* bufp);

VL_ATTR_COLD void Vtb_rv64i___024root__trace_full_0(void* voidSelf, VerilatedVcd::Buffer* bufp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root__trace_full_0\n"); );
    // Init
    Vtb_rv64i___024root* const __restrict vlSelf VL_ATTR_UNUSED = static_cast<Vtb_rv64i___024root*>(voidSelf);
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    // Body
    Vtb_rv64i___024root__trace_full_0_sub_0((&vlSymsp->TOP), bufp);
}

VL_ATTR_COLD void Vtb_rv64i___024root__trace_full_0_sub_0(Vtb_rv64i___024root* vlSelf, VerilatedVcd::Buffer* bufp) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root__trace_full_0_sub_0\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    uint32_t* const oldp VL_ATTR_UNUSED = bufp->oldp(vlSymsp->__Vm_baseCode);
    VlWide<4>/*127:0*/ __Vtemp_1;
    VlWide<4>/*127:0*/ __Vtemp_2;
    VlWide<4>/*127:0*/ __Vtemp_3;
    VlWide<4>/*127:0*/ __Vtemp_4;
    VlWide<3>/*95:0*/ __Vtemp_5;
    VlWide<4>/*127:0*/ __Vtemp_6;
    VlWide<4>/*127:0*/ __Vtemp_7;
    // Body
    bufp->fullIData(oldp+1,(vlSelfRef.tb_rv64i__DOT__trace_cnt),32);
    bufp->fullQData(oldp+2,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0]),64);
    bufp->fullQData(oldp+4,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[1]),64);
    bufp->fullQData(oldp+6,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[2]),64);
    bufp->fullQData(oldp+8,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[3]),64);
    bufp->fullQData(oldp+10,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[4]),64);
    bufp->fullQData(oldp+12,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[5]),64);
    bufp->fullQData(oldp+14,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[6]),64);
    bufp->fullQData(oldp+16,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[7]),64);
    bufp->fullQData(oldp+18,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[8]),64);
    bufp->fullQData(oldp+20,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[9]),64);
    bufp->fullQData(oldp+22,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[10]),64);
    bufp->fullQData(oldp+24,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[11]),64);
    bufp->fullQData(oldp+26,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[12]),64);
    bufp->fullQData(oldp+28,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[13]),64);
    bufp->fullQData(oldp+30,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[14]),64);
    bufp->fullQData(oldp+32,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[15]),64);
    bufp->fullQData(oldp+34,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[16]),64);
    bufp->fullQData(oldp+36,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[17]),64);
    bufp->fullQData(oldp+38,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[18]),64);
    bufp->fullQData(oldp+40,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[19]),64);
    bufp->fullQData(oldp+42,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[20]),64);
    bufp->fullQData(oldp+44,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[21]),64);
    bufp->fullQData(oldp+46,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[22]),64);
    bufp->fullQData(oldp+48,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[23]),64);
    bufp->fullQData(oldp+50,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[24]),64);
    bufp->fullQData(oldp+52,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[25]),64);
    bufp->fullQData(oldp+54,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[26]),64);
    bufp->fullQData(oldp+56,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[27]),64);
    bufp->fullQData(oldp+58,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[28]),64);
    bufp->fullQData(oldp+60,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[29]),64);
    bufp->fullQData(oldp+62,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[30]),64);
    bufp->fullQData(oldp+64,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[31]),64);
    bufp->fullQData(oldp+66,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mcycle),64);
    bufp->fullQData(oldp+68,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__minstret),64);
    bufp->fullQData(oldp+70,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mstatus),64);
    bufp->fullQData(oldp+72,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mie),64);
    bufp->fullQData(oldp+74,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mtvec),64);
    bufp->fullQData(oldp+76,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mscratch),64);
    bufp->fullQData(oldp+78,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mepc),64);
    bufp->fullQData(oldp+80,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mcause),64);
    bufp->fullQData(oldp+82,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__clint_mtime),64);
    bufp->fullQData(oldp+84,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__clint_mtimecmp),64);
    bufp->fullBit(oldp+86,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__timer_irq));
    bufp->fullIData(oldp+87,(vlSelfRef.tb_rv64i__DOT__j),32);
    bufp->fullIData(oldp+88,(vlSelfRef.tb_rv64i__DOT__word_val),32);
    bufp->fullCData(oldp+89,(vlSelfRef.tb_rv64i__DOT__byte_val),8);
    bufp->fullQData(oldp+90,(vlSelfRef.tb_rv64i__DOT__sa0),64);
    bufp->fullQData(oldp+92,(vlSelfRef.tb_rv64i__DOT__sa7),64);
    bufp->fullQData(oldp+94,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc),64);
    bufp->fullSData(oldp+96,((0x3fffU & (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                                 >> 2U)))),14);
    bufp->fullBit(oldp+97,((1U & (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                          >> 1U)))));
    bufp->fullBit(oldp+98,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_ecall));
    bufp->fullQData(oldp+99,(((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                               ? 2ULL : 4ULL)),64);
    bufp->fullSData(oldp+101,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__upper_half),16);
    bufp->fullSData(oldp+102,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half),16);
    bufp->fullBit(oldp+103,((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))));
    bufp->fullIData(oldp+104,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr),32);
    bufp->fullCData(oldp+105,((0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)),7);
    bufp->fullCData(oldp+106,((0x1fU & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                        >> 7U))),5);
    bufp->fullCData(oldp+107,((7U & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                     >> 0xcU))),3);
    bufp->fullCData(oldp+108,((0x1fU & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                        >> 0xfU))),5);
    bufp->fullCData(oldp+109,((0x1fU & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                        >> 0x14U))),5);
    bufp->fullCData(oldp+110,((vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                               >> 0x19U)),7);
    bufp->fullCData(oldp+111,((vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                               >> 0x1bU)),5);
    bufp->fullBit(oldp+112,(((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_1) 
                             & (IData)((0x10002000U 
                                        == (0xf8007000U 
                                            & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))))));
    bufp->fullBit(oldp+113,(((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_1) 
                             & (IData)((0x18002000U 
                                        == (0xf8007000U 
                                            & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))))));
    bufp->fullCData(oldp+114,((3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))),2);
    bufp->fullCData(oldp+115,((7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 0xdU))),3);
    bufp->fullBit(oldp+116,((1U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                   >> 0xcU))));
    bufp->fullCData(oldp+117,((0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                        >> 7U))),5);
    bufp->fullCData(oldp+118,((0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                        >> 2U))),5);
    bufp->fullCData(oldp+119,((7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 2U))),3);
    bufp->fullCData(oldp+120,((7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 7U))),3);
    bufp->fullCData(oldp+121,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rd5),5);
    bufp->fullCData(oldp+122,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rs15),5);
    bufp->fullCData(oldp+123,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rs25),5);
    bufp->fullCData(oldp+124,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm_i6),6);
    bufp->fullQData(oldp+125,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm),64);
    bufp->fullSData(oldp+127,(((0x800U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                          >> 1U)) | 
                               ((0x400U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                           << 2U)) 
                                | ((0x300U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                              >> 1U)) 
                                   | ((0x80U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                << 1U)) 
                                      | ((0x40U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                   >> 1U)) 
                                         | ((0x20U 
                                             & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                << 3U)) 
                                            | ((0x10U 
                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                   >> 7U)) 
                                               | (0xeU 
                                                  & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                     >> 2U)))))))))),12);
    bufp->fullQData(oldp+128,((((- (QData)((IData)(
                                                   (1U 
                                                    & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                       >> 0xcU))))) 
                                << 0xcU) | (QData)((IData)(
                                                           ((0x800U 
                                                             & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                >> 1U)) 
                                                            | ((0x400U 
                                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                   << 2U)) 
                                                               | ((0x300U 
                                                                   & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                      >> 1U)) 
                                                                  | ((0x80U 
                                                                      & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                         << 1U)) 
                                                                     | ((0x40U 
                                                                         & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                            >> 1U)) 
                                                                        | ((0x20U 
                                                                            & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                               << 3U)) 
                                                                           | ((0x10U 
                                                                               & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                >> 7U)) 
                                                                              | (0xeU 
                                                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                >> 2U))))))))))))),64);
    bufp->fullSData(oldp+130,(((0x100U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                          >> 4U)) | 
                               ((0xc0U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                          << 1U)) | 
                                ((0x20U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                           << 3U)) 
                                 | ((0x18U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                              >> 7U)) 
                                    | (6U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                             >> 2U))))))),9);
    bufp->fullQData(oldp+131,((((- (QData)((IData)(
                                                   (1U 
                                                    & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                       >> 0xcU))))) 
                                << 9U) | (QData)((IData)(
                                                         ((0x100U 
                                                           & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                              >> 4U)) 
                                                          | ((0xc0U 
                                                              & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                 << 1U)) 
                                                             | ((0x20U 
                                                                 & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                    << 3U)) 
                                                                | ((0x18U 
                                                                    & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                       >> 7U)) 
                                                                   | (6U 
                                                                      & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                         >> 2U)))))))))),64);
    bufp->fullSData(oldp+133,(((0x3c0U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                          >> 1U)) | 
                               ((0x30U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                          >> 7U)) | 
                                ((8U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                        >> 2U)) | (4U 
                                                   & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                      >> 4U)))))),10);
    bufp->fullQData(oldp+134,((QData)((IData)(((0x3c0U 
                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                   >> 1U)) 
                                               | ((0x30U 
                                                   & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                      >> 7U)) 
                                                  | ((8U 
                                                      & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                         >> 2U)) 
                                                     | (4U 
                                                        & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                           >> 4U)))))))),64);
    bufp->fullCData(oldp+136,(((0xc0U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                         >> 4U)) | 
                               ((0x30U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                          >> 2U)) | 
                                (8U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                       >> 9U))))),8);
    bufp->fullQData(oldp+137,((QData)((IData)(((0xc0U 
                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                   >> 4U)) 
                                               | ((0x30U 
                                                   & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                      >> 2U)) 
                                                  | (8U 
                                                     & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                        >> 9U))))))),64);
    bufp->fullCData(oldp+139,(((0x40U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                         << 1U)) | 
                               ((0x38U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                          >> 7U)) | 
                                (4U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                       >> 4U))))),7);
    bufp->fullQData(oldp+140,((QData)((IData)(((0x40U 
                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                   << 1U)) 
                                               | ((0x38U 
                                                   & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                      >> 7U)) 
                                                  | (4U 
                                                     & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                        >> 4U))))))),64);
    bufp->fullSData(oldp+142,(((0x200U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                          >> 3U)) | 
                               ((0x180U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                           << 4U)) 
                                | ((0x40U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                             << 1U)) 
                                   | ((0x20U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                << 3U)) 
                                      | (0x10U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                  >> 2U))))))),10);
    bufp->fullQData(oldp+143,((((- (QData)((IData)(
                                                   (1U 
                                                    & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                       >> 0xcU))))) 
                                << 0xaU) | (QData)((IData)(
                                                           ((0x200U 
                                                             & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                >> 3U)) 
                                                            | ((0x180U 
                                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                   << 4U)) 
                                                               | ((0x40U 
                                                                   & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                      << 1U)) 
                                                                  | ((0x20U 
                                                                      & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                         << 3U)) 
                                                                     | (0x10U 
                                                                        & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                           >> 2U)))))))))),64);
    bufp->fullQData(oldp+145,(((0x1ffffffffffc0000ULL 
                                & ((- (QData)((IData)(
                                                      (1U 
                                                       & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                          >> 0xcU))))) 
                                   << 0x12U)) | (QData)((IData)(
                                                                ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm_i6) 
                                                                 << 0xcU))))),64);
    bufp->fullQData(oldp+147,((QData)((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm_i6))),64);
    bufp->fullCData(oldp+149,((0x3fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                        >> 7U))),6);
    bufp->fullQData(oldp+150,((QData)((IData)((0x3fU 
                                               & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                  >> 7U))))),64);
    bufp->fullQData(oldp+152,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1),64);
    bufp->fullQData(oldp+154,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2),64);
    bufp->fullSData(oldp+156,((0x3fffU & (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                  >> 2U)))),14);
    bufp->fullCData(oldp+157,((3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result))),2);
    bufp->fullIData(oldp+158,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__data_word),32);
    bufp->fullBit(oldp+159,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_sd));
    bufp->fullBit(oldp+160,(((0x80000000ULL <= vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result) 
                             & (0x80040000ULL > vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result))));
    bufp->fullBit(oldp+161,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_uart_mmio));
    bufp->fullBit(oldp+162,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_clint_mmio));
    bufp->fullBit(oldp+163,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_mmio));
    bufp->fullQData(oldp+164,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__imm_i),64);
    bufp->fullQData(oldp+166,((((- (QData)((IData)(
                                                   (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                    >> 0x1fU)))) 
                                << 0xcU) | (QData)((IData)(
                                                           ((0xfe0U 
                                                             & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                >> 0x14U)) 
                                                            | (0x1fU 
                                                               & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                  >> 7U))))))),64);
    bufp->fullQData(oldp+168,((((- (QData)((IData)(
                                                   (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                    >> 0x1fU)))) 
                                << 0xdU) | (QData)((IData)(
                                                           ((0x1000U 
                                                             & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                >> 0x13U)) 
                                                            | ((0x800U 
                                                                & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                   << 4U)) 
                                                               | ((0x7e0U 
                                                                   & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                      >> 0x14U)) 
                                                                  | (0x1eU 
                                                                     & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                        >> 7U))))))))),64);
    bufp->fullQData(oldp+170,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__imm_u),64);
    bufp->fullQData(oldp+172,((((- (QData)((IData)(
                                                   (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                    >> 0x1fU)))) 
                                << 0x15U) | (QData)((IData)(
                                                            ((0x100000U 
                                                              & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                 >> 0xbU)) 
                                                             | ((0xff000U 
                                                                 & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr) 
                                                                | ((0x800U 
                                                                    & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                       >> 9U)) 
                                                                   | (0x7feU 
                                                                      & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                         >> 0x14U))))))))),64);
    bufp->fullQData(oldp+174,((QData)((IData)((0x1fU 
                                               & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                  >> 0xfU))))),64);
    bufp->fullQData(oldp+176,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rdata),64);
    bufp->fullBit(oldp+178,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_write));
    bufp->fullQData(oldp+179,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_wdata),64);
    bufp->fullSData(oldp+181,((vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                               >> 0x14U)),12);
    bufp->fullBit(oldp+182,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_mret));
    bufp->fullBit(oldp+183,(((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_55) 
                             & (0x105U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                           >> 0x14U)))));
    bufp->fullBit(oldp+184,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write));
    bufp->fullBit(oldp+185,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_write));
    bufp->fullBit(oldp+186,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__branch));
    bufp->fullBit(oldp+187,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__jump));
    bufp->fullBit(oldp+188,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__jalr));
    bufp->fullCData(oldp+189,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src),2);
    bufp->fullBit(oldp+190,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a));
    bufp->fullCData(oldp+191,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b),2);
    bufp->fullBit(oldp+192,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_op));
    bufp->fullBit(oldp+193,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_imm));
    bufp->fullQData(oldp+194,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b),64);
    bufp->fullCData(oldp+196,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl),4);
    bufp->fullQData(oldp+197,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a),64);
    bufp->fullQData(oldp+199,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_alu_imm),64);
    bufp->fullQData(oldp+201,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rs1_val),64);
    bufp->fullQData(oldp+203,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result),64);
    bufp->fullWData(oldp+205,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mul_full_s),128);
    __Vtemp_1[0U] = (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a);
    __Vtemp_1[1U] = (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a 
                             >> 0x20U));
    __Vtemp_1[2U] = 0U;
    __Vtemp_1[3U] = 0U;
    __Vtemp_2[0U] = (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b);
    __Vtemp_2[1U] = (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b 
                             >> 0x20U));
    __Vtemp_2[2U] = 0U;
    __Vtemp_2[3U] = 0U;
    VL_MUL_W(4, __Vtemp_3, __Vtemp_1, __Vtemp_2);
    bufp->fullWData(oldp+209,(__Vtemp_3),128);
    VL_EXTENDS_WQ(128,64, __Vtemp_4, vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a);
    __Vtemp_5[0U] = (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b);
    __Vtemp_5[1U] = (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b 
                             >> 0x20U));
    __Vtemp_5[2U] = 0U;
    VL_EXTENDS_WW(128,65, __Vtemp_6, __Vtemp_5);
    VL_MULS_WWW(128, __Vtemp_7, __Vtemp_4, __Vtemp_6);
    bufp->fullWData(oldp+213,(__Vtemp_7),128);
    bufp->fullBit(oldp+217,(((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                              ? ((6U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                               >> 0xdU)))
                                  ? (0ULL == vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1)
                                  : (IData)(((0xe000U 
                                              == (0xe000U 
                                                  & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) 
                                             & (0ULL 
                                                != vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1))))
                              : ((0U == (7U & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                               >> 0xcU)))
                                  ? (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                     == vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                  : ((1U == (7U & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                   >> 0xcU)))
                                      ? (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                         != vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                      : ((4U == (7U 
                                                 & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                    >> 0xcU)))
                                          ? VL_LTS_IQQ(64, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                          : ((5U == 
                                              (7U & 
                                               (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                >> 0xcU)))
                                              ? VL_GTES_IQQ(64, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                              : ((6U 
                                                  == 
                                                  (7U 
                                                   & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                      >> 0xcU)))
                                                  ? 
                                                 (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                                  < vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                                  : (IData)(
                                                            ((0x7000U 
                                                              == 
                                                              (0x7000U 
                                                               & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)) 
                                                             & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                                                >= vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)))))))))));
    bufp->fullBit(oldp+218,(((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__branch) 
                             & ((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                 ? ((6U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                  >> 0xdU)))
                                     ? (0ULL == vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1)
                                     : (IData)(((0xe000U 
                                                 == 
                                                 (0xe000U 
                                                  & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) 
                                                & (0ULL 
                                                   != vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1))))
                                 : ((0U == (7U & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                  >> 0xcU)))
                                     ? (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                        == vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                     : ((1U == (7U 
                                                & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                   >> 0xcU)))
                                         ? (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                            != vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                         : ((4U == 
                                             (7U & 
                                              (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                               >> 0xcU)))
                                             ? VL_LTS_IQQ(64, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                             : ((5U 
                                                 == 
                                                 (7U 
                                                  & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                     >> 0xcU)))
                                                 ? 
                                                VL_GTES_IQQ(64, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                                 : 
                                                ((6U 
                                                  == 
                                                  (7U 
                                                   & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                      >> 0xcU)))
                                                  ? 
                                                 (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                                  < vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                                  : (IData)(
                                                            ((0x7000U 
                                                              == 
                                                              (0x7000U 
                                                               & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)) 
                                                             & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                                                >= vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2))))))))))));
    bufp->fullQData(oldp+219,((0xfffffffffffffffeULL 
                               & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                  + ((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                      ? 0ULL : vlSelfRef.tb_rv64i__DOT__cpu__DOT__imm_i)))),64);
    bufp->fullQData(oldp+221,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mmio_rdata),64);
    bufp->fullQData(oldp+223,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_rdata_word),64);
    bufp->fullIData(oldp+225,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_wdata),32);
    bufp->fullBit(oldp+226,(vlSelfRef.tb_rv64i__DOT__clk));
    bufp->fullBit(oldp+227,(vlSelfRef.tb_rv64i__DOT__rst_n));
    bufp->fullIData(oldp+228,(vlSelfRef.tb_rv64i__DOT__i),32);
    bufp->fullBit(oldp+229,(vlSelfRef.tb_rv64i__DOT__term));
    bufp->fullBit(oldp+230,(vlSelfRef.tb_rv64i__DOT__uart_active));
    bufp->fullBit(oldp+231,(vlSelfRef.tb_rv64i__DOT__trace_enabled));
    bufp->fullQData(oldp+232,(((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_mret)
                                ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__mepc
                                : (((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__timer_irq) 
                                    & ((~ (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_ecall)) 
                                       & (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__mstatus 
                                                  >> 3U))))
                                    ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__mtvec
                                    : ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__jump)
                                        ? ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__jalr)
                                            ? (0xfffffffffffffffeULL 
                                               & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                                  + 
                                                  ((3U 
                                                    != 
                                                    (3U 
                                                     & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                                    ? 0ULL
                                                    : vlSelfRef.tb_rv64i__DOT__cpu__DOT__imm_i)))
                                            : (vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                               + ((3U 
                                                   != 
                                                   (3U 
                                                    & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                                   ? 
                                                  (((- (QData)((IData)(
                                                                       (1U 
                                                                        & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                           >> 0xcU))))) 
                                                    << 0xcU) 
                                                   | (QData)((IData)(
                                                                     ((0x800U 
                                                                       & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                          >> 1U)) 
                                                                      | ((0x400U 
                                                                          & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                             << 2U)) 
                                                                         | ((0x300U 
                                                                             & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                >> 1U)) 
                                                                            | ((0x80U 
                                                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                << 1U)) 
                                                                               | ((0x40U 
                                                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                >> 1U)) 
                                                                                | ((0x20U 
                                                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                << 3U)) 
                                                                                | ((0x10U 
                                                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                >> 7U)) 
                                                                                | (0xeU 
                                                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                >> 2U))))))))))))
                                                   : 
                                                  (((- (QData)((IData)(
                                                                       (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                        >> 0x1fU)))) 
                                                    << 0x15U) 
                                                   | (QData)((IData)(
                                                                     ((0x100000U 
                                                                       & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                          >> 0xbU)) 
                                                                      | ((0xff000U 
                                                                          & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr) 
                                                                         | ((0x800U 
                                                                             & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                                >> 9U)) 
                                                                            | (0x7feU 
                                                                               & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                                >> 0x14U)))))))))))
                                        : (((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__branch) 
                                            & ((3U 
                                                != 
                                                (3U 
                                                 & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                                ? (
                                                   (6U 
                                                    == 
                                                    (7U 
                                                     & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                        >> 0xdU)))
                                                    ? 
                                                   (0ULL 
                                                    == vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1)
                                                    : (IData)(
                                                              ((0xe000U 
                                                                == 
                                                                (0xe000U 
                                                                 & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) 
                                                               & (0ULL 
                                                                  != vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1))))
                                                : (
                                                   (0U 
                                                    == 
                                                    (7U 
                                                     & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                        >> 0xcU)))
                                                    ? 
                                                   (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                                    == vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                                    : 
                                                   ((1U 
                                                     == 
                                                     (7U 
                                                      & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                         >> 0xcU)))
                                                     ? 
                                                    (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                                     != vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                                     : 
                                                    ((4U 
                                                      == 
                                                      (7U 
                                                       & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                          >> 0xcU)))
                                                      ? 
                                                     VL_LTS_IQQ(64, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                                      : 
                                                     ((5U 
                                                       == 
                                                       (7U 
                                                        & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                           >> 0xcU)))
                                                       ? 
                                                      VL_GTES_IQQ(64, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                                       : 
                                                      ((6U 
                                                        == 
                                                        (7U 
                                                         & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                            >> 0xcU)))
                                                        ? 
                                                       (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                                        < vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                                        : (IData)(
                                                                  ((0x7000U 
                                                                    == 
                                                                    (0x7000U 
                                                                     & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)) 
                                                                   & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                                                      >= vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2))))))))))
                                            ? (vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                               + ((3U 
                                                   != 
                                                   (3U 
                                                    & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                                   ? 
                                                  (((- (QData)((IData)(
                                                                       (1U 
                                                                        & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                           >> 0xcU))))) 
                                                    << 9U) 
                                                   | (QData)((IData)(
                                                                     ((0x100U 
                                                                       & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                          >> 4U)) 
                                                                      | ((0xc0U 
                                                                          & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                             << 1U)) 
                                                                         | ((0x20U 
                                                                             & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                << 3U)) 
                                                                            | ((0x18U 
                                                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                >> 7U)) 
                                                                               | (6U 
                                                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                >> 2U)))))))))
                                                   : 
                                                  (((- (QData)((IData)(
                                                                       (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                        >> 0x1fU)))) 
                                                    << 0xdU) 
                                                   | (QData)((IData)(
                                                                     ((0x1000U 
                                                                       & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                          >> 0x13U)) 
                                                                      | ((0x800U 
                                                                          & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                             << 4U)) 
                                                                         | ((0x7e0U 
                                                                             & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                                >> 0x14U)) 
                                                                            | (0x1eU 
                                                                               & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                                >> 7U))))))))))
                                            : (vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                               + ((3U 
                                                   != 
                                                   (3U 
                                                    & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                                   ? 2ULL
                                                   : 4ULL))))))),64);
    bufp->fullSData(oldp+234,((0xffffU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                               [(0x3fffU & (IData)(
                                                   (vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                                    >> 2U)))])),16);
    bufp->fullQData(oldp+235,((vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                               + ((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                   ? (((- (QData)((IData)(
                                                          (1U 
                                                           & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                              >> 0xcU))))) 
                                       << 0xcU) | (QData)((IData)(
                                                                  ((0x800U 
                                                                    & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                       >> 1U)) 
                                                                   | ((0x400U 
                                                                       & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                          << 2U)) 
                                                                      | ((0x300U 
                                                                          & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                             >> 1U)) 
                                                                         | ((0x80U 
                                                                             & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                << 1U)) 
                                                                            | ((0x40U 
                                                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                >> 1U)) 
                                                                               | ((0x20U 
                                                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                << 3U)) 
                                                                                | ((0x10U 
                                                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                >> 7U)) 
                                                                                | (0xeU 
                                                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                >> 2U))))))))))))
                                   : (((- (QData)((IData)(
                                                          (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                           >> 0x1fU)))) 
                                       << 0x15U) | (QData)((IData)(
                                                                   ((0x100000U 
                                                                     & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                        >> 0xbU)) 
                                                                    | ((0xff000U 
                                                                        & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr) 
                                                                       | ((0x800U 
                                                                           & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                              >> 9U)) 
                                                                          | (0x7feU 
                                                                             & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                                >> 0x14U))))))))))),64);
    bufp->fullQData(oldp+237,(((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__jump)
                                ? ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__jalr)
                                    ? (0xfffffffffffffffeULL 
                                       & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                          + ((3U != 
                                              (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                              ? 0ULL
                                              : vlSelfRef.tb_rv64i__DOT__cpu__DOT__imm_i)))
                                    : (vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                       + ((3U != (3U 
                                                  & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                           ? (((- (QData)((IData)(
                                                                  (1U 
                                                                   & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                      >> 0xcU))))) 
                                               << 0xcU) 
                                              | (QData)((IData)(
                                                                ((0x800U 
                                                                  & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                     >> 1U)) 
                                                                 | ((0x400U 
                                                                     & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                        << 2U)) 
                                                                    | ((0x300U 
                                                                        & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                           >> 1U)) 
                                                                       | ((0x80U 
                                                                           & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                              << 1U)) 
                                                                          | ((0x40U 
                                                                              & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                >> 1U)) 
                                                                             | ((0x20U 
                                                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                << 3U)) 
                                                                                | ((0x10U 
                                                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                >> 7U)) 
                                                                                | (0xeU 
                                                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                >> 2U))))))))))))
                                           : (((- (QData)((IData)(
                                                                  (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                   >> 0x1fU)))) 
                                               << 0x15U) 
                                              | (QData)((IData)(
                                                                ((0x100000U 
                                                                  & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                     >> 0xbU)) 
                                                                 | ((0xff000U 
                                                                     & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr) 
                                                                    | ((0x800U 
                                                                        & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                           >> 9U)) 
                                                                       | (0x7feU 
                                                                          & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                             >> 0x14U)))))))))))
                                : (((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__branch) 
                                    & ((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                        ? ((6U == (7U 
                                                   & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                      >> 0xdU)))
                                            ? (0ULL 
                                               == vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1)
                                            : (IData)(
                                                      ((0xe000U 
                                                        == 
                                                        (0xe000U 
                                                         & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) 
                                                       & (0ULL 
                                                          != vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1))))
                                        : ((0U == (7U 
                                                   & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                      >> 0xcU)))
                                            ? (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                               == vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                            : ((1U 
                                                == 
                                                (7U 
                                                 & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                    >> 0xcU)))
                                                ? (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                                   != vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                                : (
                                                   (4U 
                                                    == 
                                                    (7U 
                                                     & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                        >> 0xcU)))
                                                    ? 
                                                   VL_LTS_IQQ(64, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                                    : 
                                                   ((5U 
                                                     == 
                                                     (7U 
                                                      & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                         >> 0xcU)))
                                                     ? 
                                                    VL_GTES_IQQ(64, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                                     : 
                                                    ((6U 
                                                      == 
                                                      (7U 
                                                       & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                          >> 0xcU)))
                                                      ? 
                                                     (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                                      < vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                                      : (IData)(
                                                                ((0x7000U 
                                                                  == 
                                                                  (0x7000U 
                                                                   & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)) 
                                                                 & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                                                    >= vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2))))))))))
                                    ? (vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                       + ((3U != (3U 
                                                  & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                           ? (((- (QData)((IData)(
                                                                  (1U 
                                                                   & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                      >> 0xcU))))) 
                                               << 9U) 
                                              | (QData)((IData)(
                                                                ((0x100U 
                                                                  & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                     >> 4U)) 
                                                                 | ((0xc0U 
                                                                     & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                        << 1U)) 
                                                                    | ((0x20U 
                                                                        & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                           << 3U)) 
                                                                       | ((0x18U 
                                                                           & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                              >> 7U)) 
                                                                          | (6U 
                                                                             & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                >> 2U)))))))))
                                           : (((- (QData)((IData)(
                                                                  (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                   >> 0x1fU)))) 
                                               << 0xdU) 
                                              | (QData)((IData)(
                                                                ((0x1000U 
                                                                  & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                     >> 0x13U)) 
                                                                 | ((0x800U 
                                                                     & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                        << 4U)) 
                                                                    | ((0x7e0U 
                                                                        & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                           >> 0x14U)) 
                                                                       | (0x1eU 
                                                                          & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                             >> 7U))))))))))
                                    : (vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                       + ((3U != (3U 
                                                  & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                           ? 2ULL : 4ULL))))),64);
    bufp->fullQData(oldp+239,(((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_op)
                                ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rdata
                                : ((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                    ? ((0U == (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src))
                                        ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result
                                        : ((1U == (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src))
                                            ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_rdata_word
                                            : ((2U 
                                                == (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src))
                                                ? (vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                                   + 
                                                   ((3U 
                                                     != 
                                                     (3U 
                                                      & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                                     ? 2ULL
                                                     : 4ULL))
                                                : ((IData)(
                                                           (0x4001U 
                                                            == 
                                                            (0xe003U 
                                                             & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))))
                                                    ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm
                                                    : 
                                                   (((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_38) 
                                                     & (2U 
                                                        != 
                                                        (0x1fU 
                                                         & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                            >> 7U))))
                                                     ? 
                                                    ((0x1ffffffffffc0000ULL 
                                                      & ((- (QData)((IData)(
                                                                            (1U 
                                                                             & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                                >> 0xcU))))) 
                                                         << 0x12U)) 
                                                     | (QData)((IData)(
                                                                       ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm_i6) 
                                                                        << 0xcU))))
                                                     : 
                                                    ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_59)
                                                      ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2
                                                      : vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm))))))
                                    : ((0U == (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src))
                                        ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result
                                        : ((1U == (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src))
                                            ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_rdata_word
                                            : ((2U 
                                                == (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src))
                                                ? (vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                                   + 
                                                   ((3U 
                                                     != 
                                                     (3U 
                                                      & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                                     ? 2ULL
                                                     : 4ULL))
                                                : vlSelfRef.tb_rv64i__DOT__cpu__DOT__imm_u)))))),64);
}
