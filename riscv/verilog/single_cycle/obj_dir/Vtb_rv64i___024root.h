// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design internal header
// See Vtb_rv64i.h for the primary calling header

#ifndef VERILATED_VTB_RV64I___024ROOT_H_
#define VERILATED_VTB_RV64I___024ROOT_H_  // guard

#include "verilated.h"
#include "verilated_timing.h"


class Vtb_rv64i__Syms;

class alignas(VL_CACHE_LINE_BYTES) Vtb_rv64i___024root final : public VerilatedModule {
  public:

    // DESIGN SPECIFIC STATE
    // Anonymous structures to workaround compiler member-count bugs
    struct {
        CData/*0:0*/ tb_rv64i__DOT__clk;
        CData/*0:0*/ tb_rv64i__DOT__rst_n;
        CData/*7:0*/ tb_rv64i__DOT__byte_val;
        CData/*0:0*/ tb_rv64i__DOT__term;
        CData/*0:0*/ tb_rv64i__DOT__uart_active;
        CData/*0:0*/ tb_rv64i__DOT__trace_enabled;
        CData/*4:0*/ tb_rv64i__DOT__cpu__DOT__rvc_rd5;
        CData/*4:0*/ tb_rv64i__DOT__cpu__DOT__rvc_rs15;
        CData/*4:0*/ tb_rv64i__DOT__cpu__DOT__rvc_rs25;
        CData/*5:0*/ tb_rv64i__DOT__cpu__DOT__rvc_imm_i6;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT__is_sd;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT__is_uart_mmio;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT__is_clint_mmio;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT__is_mmio;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT__timer_irq;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT__csr_write;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT__is_ecall;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT__is_mret;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT__reg_write;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT__mem_write;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT__branch;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT__jump;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT__jalr;
        CData/*1:0*/ tb_rv64i__DOT__cpu__DOT__reg_src;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT__alu_src_a;
        CData/*1:0*/ tb_rv64i__DOT__cpu__DOT__alu_src_b;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT__csr_op;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT__csr_imm;
        CData/*3:0*/ tb_rv64i__DOT__cpu__DOT__alu_ctrl;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_1;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_38;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_55;
        CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_59;
        CData/*0:0*/ __VstlFirstIteration;
        CData/*0:0*/ __Vtrigprevexpr___TOP__tb_rv64i__DOT__clk__0;
        CData/*0:0*/ __Vtrigprevexpr___TOP__tb_rv64i__DOT__rst_n__0;
        CData/*0:0*/ __VactContinue;
        SData/*15:0*/ tb_rv64i__DOT__cpu__DOT__upper_half;
        SData/*15:0*/ tb_rv64i__DOT__cpu__DOT__fetch_half;
        IData/*31:0*/ tb_rv64i__DOT__i;
        IData/*31:0*/ tb_rv64i__DOT__j;
        IData/*31:0*/ tb_rv64i__DOT__word_val;
        IData/*31:0*/ tb_rv64i__DOT__trace_cnt;
        IData/*31:0*/ tb_rv64i__DOT__cpu__DOT__instr;
        IData/*31:0*/ tb_rv64i__DOT__cpu__DOT__init_i;
        IData/*31:0*/ tb_rv64i__DOT__cpu__DOT__data_word;
        VlWide<4>/*127:0*/ tb_rv64i__DOT__cpu__DOT__mul_full_s;
        IData/*31:0*/ tb_rv64i__DOT__cpu__DOT__mem_wdata;
        IData/*31:0*/ __VactIterCount;
        QData/*63:0*/ tb_rv64i__DOT__sa0;
        QData/*63:0*/ tb_rv64i__DOT__sa7;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__pc;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__pc_next;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__rvc_imm;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__rf_rdata1;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__rf_rdata2;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__imm_i;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__imm_u;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__mcycle;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__minstret;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__mstatus;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__mie;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__mtvec;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__mscratch;
    };
    struct {
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__mepc;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__mcause;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__clint_mtime;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__clint_mtimecmp;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__csr_rdata;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__csr_wdata;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__alu_b;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__alu_a;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__rvc_alu_imm;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__csr_rs1_val;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__alu_result;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__mmio_rdata;
        QData/*63:0*/ tb_rv64i__DOT__cpu__DOT__mem_rdata_word;
        VlUnpacked<IData/*31:0*/, 16384> tb_rv64i__DOT__cpu__DOT__imem;
        VlUnpacked<QData/*63:0*/, 32> tb_rv64i__DOT__cpu__DOT__rf;
        VlUnpacked<CData/*0:0*/, 7> __Vm_traceActivity;
    };
    VlDelayScheduler __VdlySched;
    VlTriggerVec<1> __VstlTriggered;
    VlTriggerVec<4> __VactTriggered;
    VlTriggerVec<4> __VnbaTriggered;

    // INTERNAL VARIABLES
    Vtb_rv64i__Syms* const vlSymsp;

    // CONSTRUCTORS
    Vtb_rv64i___024root(Vtb_rv64i__Syms* symsp, const char* v__name);
    ~Vtb_rv64i___024root();
    VL_UNCOPYABLE(Vtb_rv64i___024root);

    // INTERNAL METHODS
    void __Vconfigure(bool first);
};


#endif  // guard
