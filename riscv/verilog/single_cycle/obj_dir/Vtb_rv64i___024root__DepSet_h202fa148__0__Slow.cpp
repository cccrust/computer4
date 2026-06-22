// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vtb_rv64i.h for the primary calling header

#include "Vtb_rv64i__pch.h"
#include "Vtb_rv64i___024root.h"

VL_ATTR_COLD void Vtb_rv64i___024root___eval_static(Vtb_rv64i___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root___eval_static\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
}

VL_ATTR_COLD void Vtb_rv64i___024root___eval_initial__TOP(Vtb_rv64i___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root___eval_initial__TOP\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    VlWide<3>/*95:0*/ __Vtemp_1;
    // Body
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[1U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[2U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[3U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[4U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[5U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[6U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[7U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[8U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[9U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0xaU] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0xbU] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0xcU] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0xdU] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0xeU] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0xfU] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0x10U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0x11U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0x12U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0x13U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0x14U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0x15U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0x16U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0x17U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0x18U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0x19U] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0x1aU] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0x1bU] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0x1cU] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0x1dU] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0x1eU] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0x1fU] = 0ULL;
    vlSelfRef.tb_rv64i__DOT__i = 0x20U;
    vlSelfRef.tb_rv64i__DOT__trace_cnt = 0U;
    __Vtemp_1[0U] = 0x2e686578U;
    __Vtemp_1[1U] = 0x6772616dU;
    __Vtemp_1[2U] = 0x70726fU;
    VL_READMEM_N(true, 32, 16384, 0, VL_CVT_PACK_STR_NW(3, __Vtemp_1)
                 ,  &(vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem)
                 , 0, ~0ULL);
}

VL_ATTR_COLD void Vtb_rv64i___024root___eval_final(Vtb_rv64i___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root___eval_final\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
}

#ifdef VL_DEBUG
VL_ATTR_COLD void Vtb_rv64i___024root___dump_triggers__stl(Vtb_rv64i___024root* vlSelf);
#endif  // VL_DEBUG
VL_ATTR_COLD bool Vtb_rv64i___024root___eval_phase__stl(Vtb_rv64i___024root* vlSelf);

VL_ATTR_COLD void Vtb_rv64i___024root___eval_settle(Vtb_rv64i___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root___eval_settle\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    IData/*31:0*/ __VstlIterCount;
    CData/*0:0*/ __VstlContinue;
    // Body
    __VstlIterCount = 0U;
    vlSelfRef.__VstlFirstIteration = 1U;
    __VstlContinue = 1U;
    while (__VstlContinue) {
        if (VL_UNLIKELY((0x64U < __VstlIterCount))) {
#ifdef VL_DEBUG
            Vtb_rv64i___024root___dump_triggers__stl(vlSelf);
#endif
            VL_FATAL_MT("tb_rv64i.v", 5, "", "Settle region did not converge.");
        }
        __VstlIterCount = ((IData)(1U) + __VstlIterCount);
        __VstlContinue = 0U;
        if (Vtb_rv64i___024root___eval_phase__stl(vlSelf)) {
            __VstlContinue = 1U;
        }
        vlSelfRef.__VstlFirstIteration = 0U;
    }
}

#ifdef VL_DEBUG
VL_ATTR_COLD void Vtb_rv64i___024root___dump_triggers__stl(Vtb_rv64i___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root___dump_triggers__stl\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if ((1U & (~ vlSelfRef.__VstlTriggered.any()))) {
        VL_DBG_MSGF("         No triggers active\n");
    }
    if ((1ULL & vlSelfRef.__VstlTriggered.word(0U))) {
        VL_DBG_MSGF("         'stl' region trigger index 0 is active: Internal 'stl' trigger - first iteration\n");
    }
}
#endif  // VL_DEBUG

VL_ATTR_COLD void Vtb_rv64i___024root___stl_sequent__TOP__0(Vtb_rv64i___024root* vlSelf);

VL_ATTR_COLD void Vtb_rv64i___024root___eval_stl(Vtb_rv64i___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root___eval_stl\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if ((1ULL & vlSelfRef.__VstlTriggered.word(0U))) {
        Vtb_rv64i___024root___stl_sequent__TOP__0(vlSelf);
        vlSelfRef.__Vm_traceActivity[6U] = 1U;
        vlSelfRef.__Vm_traceActivity[5U] = 1U;
        vlSelfRef.__Vm_traceActivity[4U] = 1U;
        vlSelfRef.__Vm_traceActivity[3U] = 1U;
        vlSelfRef.__Vm_traceActivity[2U] = 1U;
        vlSelfRef.__Vm_traceActivity[1U] = 1U;
        vlSelfRef.__Vm_traceActivity[0U] = 1U;
    }
}

VL_ATTR_COLD void Vtb_rv64i___024root___stl_sequent__TOP__0(Vtb_rv64i___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root___stl_sequent__TOP__0\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h4bd778cc__0;
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h4bd778cc__0 = 0;
    CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h00388b63__0;
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h00388b63__0 = 0;
    CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hb2e46a47__0;
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hb2e46a47__0 = 0;
    CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hb2e705f3__0;
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hb2e705f3__0 = 0;
    CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h77e9143c__0;
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h77e9143c__0 = 0;
    CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h24f9eeb0__0;
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h24f9eeb0__0 = 0;
    CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hf6a39fa8__0;
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hf6a39fa8__0 = 0;
    CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgExtracted_ha830e83f__0;
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_ha830e83f__0 = 0;
    CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h0d0b5e95__0;
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h0d0b5e95__0 = 0;
    CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h276e89c6__0;
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h276e89c6__0 = 0;
    CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h276853d3__0;
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h276853d3__0 = 0;
    QData/*63:0*/ tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hf9ddbd55__0;
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hf9ddbd55__0 = 0;
    IData/*31:0*/ tb_rv64i__DOT__cpu__DOT____VdfgExtracted_he1520fff__0;
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_he1520fff__0 = 0;
    CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_7;
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_7 = 0;
    CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_12;
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_12 = 0;
    CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_39;
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_39 = 0;
    CData/*7:0*/ tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_41;
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_41 = 0;
    CData/*0:0*/ tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_43;
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_43 = 0;
    CData/*7:0*/ tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_44;
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_44 = 0;
    CData/*7:0*/ tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_46;
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_46 = 0;
    CData/*7:0*/ tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_47;
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_47 = 0;
    SData/*15:0*/ tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_48;
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_48 = 0;
    SData/*15:0*/ tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_49;
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_49 = 0;
    VlWide<3>/*95:0*/ __Vtemp_12;
    VlWide<3>/*95:0*/ __Vtemp_26;
    VlWide<3>/*95:0*/ __Vtemp_45;
    VlWide<3>/*95:0*/ __Vtemp_50;
    VlWide<3>/*95:0*/ __Vtemp_52;
    VlWide<4>/*127:0*/ __Vtemp_53;
    VlWide<4>/*127:0*/ __Vtemp_54;
    VlWide<4>/*127:0*/ __Vtemp_55;
    VlWide<4>/*127:0*/ __Vtemp_56;
    VlWide<4>/*127:0*/ __Vtemp_57;
    VlWide<4>/*127:0*/ __Vtemp_58;
    VlWide<3>/*95:0*/ __Vtemp_59;
    VlWide<4>/*127:0*/ __Vtemp_60;
    VlWide<4>/*127:0*/ __Vtemp_61;
    // Body
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__timer_irq = 
        ((IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__mie 
                  >> 7U)) & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__clint_mtime 
                             >= vlSelfRef.tb_rv64i__DOT__cpu__DOT__clint_mtimecmp));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__upper_half = 
        (vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem[(0x3fffU 
                                                  & (IData)(
                                                            (vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                                             >> 2U)))] 
         >> 0x10U);
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half = 
        (0xffffU & ((1U & (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                   >> 1U))) ? (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__upper_half)
                     : vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                    [(0x3fffU & (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                         >> 2U)))]));
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hf6a39fa8__0 
        = (IData)(((0U == (0x107cU & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) 
                   & (0U != (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                      >> 7U)))));
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_ha830e83f__0 
        = (IData)(((0x1000U == (0x107cU & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) 
                   & (0U != (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                      >> 7U)))));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_38 
        = (IData)((0x6001U == (0xe003U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))));
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_12 
        = ((6U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                         >> 0xdU))) | (7U == (7U & 
                                              ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                               >> 0xdU))));
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h276853d3__0 
        = (IData)((0U == (0xe003U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm_i6 = 
        ((0x20U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                   >> 7U)) | (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                       >> 2U)));
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h00388b63__0 
        = (IData)((0x1000U == (0x1c00U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))));
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h24f9eeb0__0 
        = ((0U != (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                            >> 2U))) & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                        >> 0xcU));
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h4bd778cc__0 
        = (IData)((0U == (0x1c00U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))));
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hb2e46a47__0 
        = (IData)((0xc00U == (0xc60U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))));
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hb2e705f3__0 
        = (IData)((0xc40U == (0xc60U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))));
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h276e89c6__0 
        = (IData)((0x8001U == (0xe003U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))));
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h77e9143c__0 
        = ((~ ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
               >> 0xcU)) & (0U != (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                            >> 2U))));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm = ((
                                                   (- (QData)((IData)(
                                                                      (1U 
                                                                       & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                          >> 0xcU))))) 
                                                   << 6U) 
                                                  | (QData)((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm_i6)));
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_7 
        = ((IData)(tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h276e89c6__0) 
           & (0xc00U == (0xc00U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_59 
        = ((IData)((0x8002U == (0xe003U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) 
           & (IData)(tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h77e9143c__0));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_write = 0U;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr = ((3U 
                                                 != 
                                                 (3U 
                                                  & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                                 ? (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)
                                                 : 
                                                ((1U 
                                                  & (IData)(
                                                            (vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                                             >> 1U)))
                                                  ? 
                                                 ((vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                                   [
                                                   (0x3fffU 
                                                    & ((IData)(1U) 
                                                       + (IData)(
                                                                 (vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                                                  >> 2U))))] 
                                                   << 0x10U) 
                                                  | (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__upper_half))
                                                  : 
                                                 vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                                 [(0x3fffU 
                                                   & (IData)(
                                                             (vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                                              >> 2U)))]));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_1 
        = ((3U == (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) 
           & (0x2fU == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 0U;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rdata = 
        (((((((((0x300U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                            >> 0x14U)) | (0x304U == 
                                          (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                           >> 0x14U))) 
               | (0x305U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                             >> 0x14U))) | (0x340U 
                                            == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                >> 0x14U))) 
             | (0x341U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                           >> 0x14U))) | (0x342U == 
                                          (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                           >> 0x14U))) 
           | (0xb00U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                         >> 0x14U))) | (0xb02U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                   >> 0x14U)))
          ? ((0x300U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                         >> 0x14U)) ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__mstatus
              : ((0x304U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                             >> 0x14U)) ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__mie
                  : ((0x305U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                 >> 0x14U)) ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__mtvec
                      : ((0x340U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                     >> 0x14U)) ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__mscratch
                          : ((0x341U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                         >> 0x14U))
                              ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__mepc
                              : ((0x342U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                             >> 0x14U))
                                  ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__mcause
                                  : ((0xb00U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                 >> 0x14U))
                                      ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__mcycle
                                      : vlSelfRef.tb_rv64i__DOT__cpu__DOT__minstret)))))))
          : 0ULL);
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__branch = 0U;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__jalr = 0U;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__jump = 0U;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_55 
        = ((3U == (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) 
           & (IData)((0x73U == (0x707fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__imm_u = (((QData)((IData)(
                                                                 (- (IData)(
                                                                            (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                             >> 0x1fU))))) 
                                                 << 0x20U) 
                                                | (QData)((IData)(
                                                                  (0xfffff000U 
                                                                   & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 0U;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__imm_i = (((- (QData)((IData)(
                                                                    (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                     >> 0x1fU)))) 
                                                 << 0xcU) 
                                                | (QData)((IData)(
                                                                  (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                   >> 0x14U))));
    __Vtemp_12[0U] = ((2U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                    >> 0xdU))) ? ((0x40U 
                                                   & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                      << 1U)) 
                                                  | ((0x38U 
                                                      & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                         >> 7U)) 
                                                     | (4U 
                                                        & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                           >> 4U))))
                       : ((3U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                        >> 0xdU))) ? 
                          ((0xc0U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 4U)) | ((0x30U 
                                                 & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                    >> 2U)) 
                                                | (8U 
                                                   & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                      >> 9U))))
                           : ((6U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                            >> 0xdU)))
                               ? ((0x40U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                            << 1U)) 
                                  | ((0x38U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                               >> 7U)) 
                                     | (4U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                              >> 4U))))
                               : (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm))));
    if ((0x6100U == (0xef80U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
        __Vtemp_26[0U] = (IData)((((- (QData)((IData)(
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
                                                                              >> 2U))))))))));
        __Vtemp_26[1U] = (IData)(((((- (QData)((IData)(
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
                                                                               >> 2U))))))))) 
                                  >> 0x20U));
    } else if ((0x8800U == (0xec00U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
        __Vtemp_26[0U] = (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm);
        __Vtemp_26[1U] = (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm 
                                  >> 0x20U));
    } else if ((0x8000U == (0xec00U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
        __Vtemp_26[0U] = vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm_i6;
        __Vtemp_26[1U] = 0U;
    } else {
        __Vtemp_26[0U] = (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm);
        __Vtemp_26[1U] = (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm 
                                  >> 0x20U));
    }
    __Vtemp_45[0U] = ((0U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                    >> 0xdU))) ? (0x1fU 
                                                  & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                     >> 7U))
                       : ((1U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                        >> 0xdU))) ? (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm_i6)
                           : ((2U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                            >> 0xdU)))
                               ? (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm_i6)
                               : ((3U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                >> 0xdU)))
                                   ? (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm_i6)
                                   : ((6U == (7U & 
                                              ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                               >> 0xdU)))
                                       ? (0x3fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                   >> 7U))
                                       : ((7U == (7U 
                                                  & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                     >> 0xdU)))
                                           ? (0x3fU 
                                              & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                 >> 7U))
                                           : (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm)))))));
    if ((1U == (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
        __Vtemp_50[1U] = __Vtemp_26[1U];
        __Vtemp_50[2U] = ((IData)((0x6100U == (0xef80U 
                                               & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))))
                           ? 0U : ((IData)((0x8800U 
                                            == (0xec00U 
                                                & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))))
                                    ? 0U : 0U));
    } else if ((2U == (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
        if ((0U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                          >> 0xdU)))) {
            __Vtemp_50[1U] = 0U;
            __Vtemp_50[2U] = 0U;
        } else if ((1U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                 >> 0xdU)))) {
            __Vtemp_50[1U] = 0U;
            __Vtemp_50[2U] = 0U;
        } else if ((2U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                 >> 0xdU)))) {
            __Vtemp_50[1U] = 0U;
            __Vtemp_50[2U] = 0U;
        } else if ((3U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                 >> 0xdU)))) {
            __Vtemp_50[1U] = 0U;
            __Vtemp_50[2U] = 0U;
        } else if ((6U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                 >> 0xdU)))) {
            __Vtemp_50[1U] = 0U;
            __Vtemp_50[2U] = 0U;
        } else {
            __Vtemp_50[1U] = ((7U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                            >> 0xdU)))
                               ? 0U : (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm 
                                               >> 0x20U)));
            __Vtemp_50[2U] = 0U;
        }
    } else {
        __Vtemp_50[1U] = 0U;
        __Vtemp_50[2U] = 0U;
    }
    __Vtemp_52[0U] = ((0U == (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                       ? ((0U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                        >> 0xdU))) ? 
                          ((0x3c0U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                      >> 1U)) | ((0x30U 
                                                  & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                     >> 7U)) 
                                                 | ((8U 
                                                     & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                        >> 2U)) 
                                                    | (4U 
                                                       & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                          >> 4U)))))
                           : ((1U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                            >> 0xdU)))
                               ? ((0xc0U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                            >> 4U)) 
                                  | ((0x30U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                               >> 2U)) 
                                     | (8U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                              >> 9U))))
                               : __Vtemp_12[0U])) : 
                      ((1U == (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                        ? __Vtemp_26[0U] : ((2U == 
                                             (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                             ? __Vtemp_45[0U]
                                             : 0U)));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_alu_imm 
        = (((QData)((IData)(((0U == (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                              ? ((0U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                               >> 0xdU)))
                                  ? 0U : ((1U == (7U 
                                                  & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                     >> 0xdU)))
                                           ? 0U : (
                                                   (2U 
                                                    == 
                                                    (7U 
                                                     & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                        >> 0xdU)))
                                                    ? 0U
                                                    : 
                                                   ((3U 
                                                     == 
                                                     (7U 
                                                      & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                         >> 0xdU)))
                                                     ? 0U
                                                     : 
                                                    ((6U 
                                                      == 
                                                      (7U 
                                                       & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                          >> 0xdU)))
                                                      ? 0U
                                                      : (IData)(
                                                                (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm 
                                                                 >> 0x20U)))))))
                              : __Vtemp_50[1U]))) << 0x20U) 
           | (QData)((IData)(__Vtemp_52[0U])));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_ecall = ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_55) 
                                                   & (0U 
                                                      == 
                                                      (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                       >> 0x14U)));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_mret = ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_55) 
                                                  & (0x302U 
                                                     == 
                                                     (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                      >> 0x14U)));
    if ((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
        if ((1U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
            if ((2U == (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
                if ((0U != (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                  >> 0xdU)))) {
                    if ((1U != (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                      >> 0xdU)))) {
                        if ((2U != (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                          >> 0xdU)))) {
                            if ((3U != (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                              >> 0xdU)))) {
                                if ((4U != (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                  >> 0xdU)))) {
                                    if ((6U == (7U 
                                                & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                   >> 0xdU)))) {
                                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_write = 1U;
                                    } else if ((7U 
                                                == 
                                                (7U 
                                                 & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                    >> 0xdU)))) {
                                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_write = 1U;
                                    }
                                }
                                if ((4U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                  >> 0xdU)))) {
                                    if ((1U & (~ (IData)(tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h77e9143c__0)))) {
                                        if ((1U & (~ (IData)(tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h24f9eeb0__0)))) {
                                            if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hf6a39fa8__0) {
                                                vlSelfRef.tb_rv64i__DOT__cpu__DOT__jalr = 1U;
                                            } else if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_ha830e83f__0) {
                                                vlSelfRef.tb_rv64i__DOT__cpu__DOT__jalr = 1U;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else if ((0U == (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
                if ((0U != (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                  >> 0xdU)))) {
                    if ((1U != (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                      >> 0xdU)))) {
                        if ((2U != (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                          >> 0xdU)))) {
                            if ((3U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                              >> 0xdU)))) {
                                vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_write = 1U;
                            } else if ((6U == (7U & 
                                               ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                >> 0xdU)))) {
                                vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_write = 1U;
                            }
                        }
                    }
                }
            }
        }
        if ((1U == (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
            if ((0x8000U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) {
                if ((1U & (~ ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                              >> 0xeU)))) {
                    if ((1U & (~ ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                  >> 0xdU)))) {
                        if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h4bd778cc__0) {
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 0U;
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
                        } else if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h00388b63__0) {
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 0U;
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
                        } else if ((2U == (3U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                 >> 0xaU)))) {
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 0U;
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
                        } else if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hb2e46a47__0) {
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 0U;
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 0U;
                        } else if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hb2e705f3__0) {
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 0U;
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 0U;
                        }
                    }
                    if ((0x2000U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) {
                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__jump = 1U;
                    }
                }
                if ((0x4000U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) {
                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__branch = 1U;
                }
            } else if ((0x4000U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) {
                if ((0x2000U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) {
                    if ((2U == (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                         >> 7U)))) {
                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 0U;
                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
                    } else if ((0U != (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                >> 7U)))) {
                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 3U;
                    }
                } else {
                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 3U;
                }
            } else {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
            }
        } else if ((2U == (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
            if ((0U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                              >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
            } else if ((1U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 1U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
            } else if ((2U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 1U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
            } else if ((3U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 1U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
            } else if ((4U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 0xdU)))) {
                if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h77e9143c__0) {
                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 3U;
                } else if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h24f9eeb0__0) {
                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 0U;
                } else if ((1U & (~ (IData)(tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hf6a39fa8__0)))) {
                    if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_ha830e83f__0) {
                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 2U;
                    }
                }
                if ((1U & (~ (IData)(tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h77e9143c__0)))) {
                    if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h24f9eeb0__0) {
                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 0U;
                    }
                }
            } else if ((6U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
            } else if ((7U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
            }
            if ((0U != (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                              >> 0xdU)))) {
                if ((1U != (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                  >> 0xdU)))) {
                    if ((2U != (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                      >> 0xdU)))) {
                        if ((3U != (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                          >> 0xdU)))) {
                            if ((4U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                              >> 0xdU)))) {
                                if ((1U & (~ (IData)(tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h77e9143c__0)))) {
                                    if ((1U & (~ (IData)(tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h24f9eeb0__0)))) {
                                        if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hf6a39fa8__0) {
                                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__jump = 1U;
                                        } else if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_ha830e83f__0) {
                                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__jump = 1U;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if ((0U == (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
            if ((0U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                              >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
            } else if ((1U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 1U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
            } else if ((2U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 1U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
            } else if ((3U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
            } else if ((6U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
            }
        }
        vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_sd = 
            ((IData)((0xe002U == (0xe003U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) 
             | (IData)((0x6000U == (0xe003U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))));
        vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl 
            = ((IData)((1U == (0xe003U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))))
                ? 0U : ((IData)((0x2001U == (0xe003U 
                                             & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))))
                         ? 0U : (((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_38) 
                                  & (0x100U == (0xf80U 
                                                & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))))
                                  ? 0U : ((IData)(tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h276e89c6__0)
                                           ? ((IData)(tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h4bd778cc__0)
                                               ? 6U
                                               : ((IData)(
                                                          (0x1000U 
                                                           == 
                                                           (0x1c00U 
                                                            & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))))
                                                   ? 7U
                                                   : 
                                                  ((2U 
                                                    == 
                                                    (3U 
                                                     & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                        >> 0xaU)))
                                                    ? 9U
                                                    : 
                                                   ((IData)(tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hb2e46a47__0)
                                                     ? 
                                                    ((0x1000U 
                                                      & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))
                                                      ? 8U
                                                      : 1U)
                                                     : 
                                                    ((IData)(tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hb2e705f3__0)
                                                      ? 
                                                     ((0x1000U 
                                                       & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))
                                                       ? 9U
                                                       : 5U)
                                                      : 0U)))))
                                           : ((IData)(tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h276853d3__0)
                                               ? 0U
                                               : ((IData)(
                                                          (2U 
                                                           == 
                                                           (0xe003U 
                                                            & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))))
                                                   ? 2U
                                                   : 0U))))));
        if ((0U == (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
            vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rd5 
                = (0x1fU & (8U | (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                        >> 2U))));
            vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rs25 
                = (0x1fU & (8U | (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                        >> 2U))));
        } else if (tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_7) {
            vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rd5 
                = (0x1fU & (8U | (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                        >> 7U))));
            vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rs25 
                = (0x1fU & (8U | (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                        >> 2U))));
        } else {
            vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rd5 
                = (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                            >> 7U));
            vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rs25 
                = (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                            >> 2U));
        }
    } else {
        if (((((((((0x33U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)) 
                   | (0x13U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                  | (3U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                 | (0x23U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                | (0x63U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
               | (0x37U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
              | (0x17U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
             | (0x6fU == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)))) {
            if ((0x33U != (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                if ((0x13U != (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                    if ((3U != (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                        if ((0x23U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_write = 1U;
                        }
                        if ((0x23U != (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                            if ((0x63U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                                vlSelfRef.tb_rv64i__DOT__cpu__DOT__branch = 1U;
                            }
                            if ((0x63U != (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                                if ((0x37U != (0x7fU 
                                               & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                                    if ((0x17U != (0x7fU 
                                                   & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__jump = 1U;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if ((0x33U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 0U;
            } else if ((0x13U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
            } else if ((3U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 1U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
            } else {
                if ((0x23U != (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                    if ((0x63U != (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src 
                            = ((0x37U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))
                                ? 3U : ((0x17U == (0x7fU 
                                                   & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))
                                         ? 0U : 2U));
                    }
                }
                if ((0x23U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
                } else if ((0x63U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 0U;
                } else if ((0x37U != (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                    if ((0x17U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 1U;
                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 2U;
                    }
                }
            }
        } else {
            if ((0x67U != (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                if ((0x2fU == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                    if ((2U != (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                >> 0x1bU))) {
                        if ((3U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                    >> 0x1bU))) {
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_write = 1U;
                        }
                    }
                }
            }
            if ((0x67U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 2U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__jump = 1U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
            } else if ((0x2fU == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                if ((2U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                            >> 0x1bU))) {
                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src = 1U;
                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
                } else if ((3U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                   >> 0x1bU))) {
                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a = 0U;
                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b = 1U;
                }
            }
        }
        vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_sd = (IData)(
                                                           (0x3023U 
                                                            == 
                                                            (0x707fU 
                                                             & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)));
        if ((1U & (~ ((((((((0x33U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)) 
                            | (0x13U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                           | (3U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                          | (0x23U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                         | (0x63U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                        | (0x37U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                       | (0x17U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                      | (0x6fU == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)))))) {
            if ((0x67U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__jalr = 1U;
            }
        }
        vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl 
            = ((0x33U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))
                ? ((vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                    >> 0x1fU) ? 0U : ((0x40000000U 
                                       & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                       ? ((0x20000000U 
                                           & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                           ? 0U : (
                                                   (0x10000000U 
                                                    & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                    ? 0U
                                                    : 
                                                   ((0x8000000U 
                                                     & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                     ? 0U
                                                     : 
                                                    ((0x4000000U 
                                                      & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                      ? 0U
                                                      : 
                                                     ((0x2000000U 
                                                       & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                       ? 0U
                                                       : 
                                                      ((0x4000U 
                                                        & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                        ? 
                                                       ((0x2000U 
                                                         & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                         ? 0U
                                                         : 
                                                        ((0x1000U 
                                                          & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                          ? 7U
                                                          : 0U))
                                                        : 
                                                       ((0x2000U 
                                                         & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                         ? 0U
                                                         : 
                                                        ((0x1000U 
                                                          & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                          ? 0U
                                                          : 1U))))))))
                                       : ((0x20000000U 
                                           & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                           ? 0U : (
                                                   (0x10000000U 
                                                    & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                    ? 0U
                                                    : 
                                                   ((0x8000000U 
                                                     & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                     ? 0U
                                                     : 
                                                    ((0x4000000U 
                                                      & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                      ? 0U
                                                      : 
                                                     ((0x2000000U 
                                                       & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                       ? 
                                                      ((0x4000U 
                                                        & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                        ? 
                                                       ((0x2000U 
                                                         & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                         ? 
                                                        ((0x1000U 
                                                          & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                          ? 0xfU
                                                          : 0xeU)
                                                         : 
                                                        ((0x1000U 
                                                          & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                          ? 0xfU
                                                          : 0xeU))
                                                        : 
                                                       ((0x2000U 
                                                         & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                         ? 
                                                        ((0x1000U 
                                                          & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                          ? 0xdU
                                                          : 0xcU)
                                                         : 
                                                        ((0x1000U 
                                                          & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                          ? 0xcU
                                                          : 0xbU)))
                                                       : 
                                                      ((0x4000U 
                                                        & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                        ? 
                                                       ((0x2000U 
                                                         & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                         ? 
                                                        ((0x1000U 
                                                          & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                          ? 9U
                                                          : 8U)
                                                         : 
                                                        ((0x1000U 
                                                          & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                          ? 6U
                                                          : 5U))
                                                        : 
                                                       ((0x2000U 
                                                         & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                         ? 
                                                        ((0x1000U 
                                                          & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                          ? 4U
                                                          : 3U)
                                                         : 
                                                        ((0x1000U 
                                                          & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                          ? 2U
                                                          : 0U))))))))))
                : ((0x13U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))
                    ? ((0x4000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                        ? ((0x2000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                            ? ((0x1000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                ? 9U : 8U) : ((0x1000U 
                                               & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                               ? ((0x40000000U 
                                                   & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                   ? 7U
                                                   : 6U)
                                               : 5U))
                        : ((0x2000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                            ? ((0x1000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                ? 4U : 3U) : ((0x1000U 
                                               & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                               ? ((0x40000000U 
                                                   & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                                   ? 0U
                                                   : 2U)
                                               : 0U)))
                    : ((0x37U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))
                        ? 0xaU : 0U)));
        vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rd5 
            = (0x1fU & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                        >> 7U));
        vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rs25 
            = (0x1fU & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                        >> 0x14U));
    }
    if ((0U != (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rs25))) {
        tb_rv64i__DOT__cpu__DOT____VdfgExtracted_he1520fff__0 
            = (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf
                      [vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rs25]);
        vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2 
            = vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf
            [vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rs25];
    } else {
        tb_rv64i__DOT__cpu__DOT____VdfgExtracted_he1520fff__0 = 0U;
        vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2 = 0ULL;
    }
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rs15 = (0x1fU 
                                                   & ((3U 
                                                       != 
                                                       (3U 
                                                        & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                                       ? 
                                                      ((IData)(tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h276853d3__0)
                                                        ? 2U
                                                        : 
                                                       ((0U 
                                                         == 
                                                         (3U 
                                                          & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                                         ? 
                                                        (8U 
                                                         | (7U 
                                                            & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                               >> 7U)))
                                                         : 
                                                        ((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_7)
                                                          ? 
                                                         (8U 
                                                          | (7U 
                                                             & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                >> 7U)))
                                                          : 
                                                         (((1U 
                                                            == 
                                                            (3U 
                                                             & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) 
                                                           & (IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_12))
                                                           ? 
                                                          (8U 
                                                           | (7U 
                                                              & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                 >> 7U)))
                                                           : 
                                                          (((2U 
                                                             == 
                                                             (3U 
                                                              & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) 
                                                            & ((2U 
                                                                == 
                                                                (7U 
                                                                 & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                    >> 0xdU))) 
                                                               | ((3U 
                                                                   == 
                                                                   (7U 
                                                                    & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                                       >> 0xdU))) 
                                                                  | (IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_12))))
                                                            ? 2U
                                                            : 
                                                           ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_59)
                                                             ? 0U
                                                             : 
                                                            ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                             >> 7U)))))))
                                                       : 
                                                      (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                       >> 0xfU)));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 = 
        ((0U == (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rs15))
          ? 0ULL : vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf
         [vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rs15]);
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h0d0b5e95__0 
        = (1U & ((~ (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_ecall)) 
                 & (~ ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_mret) 
                       | ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_55) 
                          & (0x105U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                        >> 0x14U)))))));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc_next = ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_mret)
                                                   ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__mepc
                                                   : 
                                                  (((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__timer_irq) 
                                                    & ((~ (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_ecall)) 
                                                       & (IData)(
                                                                 (vlSelfRef.tb_rv64i__DOT__cpu__DOT__mstatus 
                                                                  >> 3U))))
                                                    ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__mtvec
                                                    : 
                                                   ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__jump)
                                                     ? 
                                                    ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__jalr)
                                                      ? 
                                                     (0xfffffffffffffffeULL 
                                                      & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                                         + 
                                                         ((3U 
                                                           != 
                                                           (3U 
                                                            & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                                           ? 0ULL
                                                           : vlSelfRef.tb_rv64i__DOT__cpu__DOT__imm_i)))
                                                      : 
                                                     (vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                                      + 
                                                      ((3U 
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
                                                     : 
                                                    (((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__branch) 
                                                      & ((3U 
                                                          != 
                                                          (3U 
                                                           & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                                          ? 
                                                         ((6U 
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
                                                          : 
                                                         ((0U 
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
                                                      ? 
                                                     (vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                                      + 
                                                      ((3U 
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
                                                      : 
                                                     (vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                                      + 
                                                      ((3U 
                                                        != 
                                                        (3U 
                                                         & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                                        ? 2ULL
                                                        : 4ULL))))));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a = ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a)
                                                 ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc
                                                 : vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1);
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_write = 0U;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_op = 0U;
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 0U;
    if ((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
        vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b = 
            ((0U == (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b))
              ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2
              : vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_alu_imm);
        if ((1U == (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
            if ((0x8000U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) {
                if ((1U & (~ ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                              >> 0xeU)))) {
                    if ((1U & (~ ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                  >> 0xdU)))) {
                        if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h4bd778cc__0) {
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 1U;
                        } else if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h00388b63__0) {
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 1U;
                        } else if ((2U == (3U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                 >> 0xaU)))) {
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 1U;
                        } else if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hb2e46a47__0) {
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write 
                                = (0U != (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                   >> 7U)));
                        } else if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hb2e705f3__0) {
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write 
                                = (0U != (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                   >> 7U)));
                        }
                    }
                }
            } else if ((0x4000U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) {
                if ((0x2000U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) {
                    if ((2U == (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                         >> 7U)))) {
                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 1U;
                    } else if ((0U != (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                >> 7U)))) {
                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 1U;
                    }
                } else {
                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write 
                        = (0U != (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                           >> 7U)));
                }
            } else {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write 
                    = ((0x2000U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))
                        ? (0U != (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                           >> 7U)))
                        : ((0U != (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                            >> 7U))) 
                           & (0U != (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm_i6))));
            }
        } else if ((2U == (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
            if ((0U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                              >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write 
                    = (0U != (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                       >> 7U)));
            } else if ((1U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write 
                    = (0U != (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                       >> 7U)));
            } else if ((2U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write 
                    = (0U != (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                       >> 7U)));
            } else if ((3U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write 
                    = (0U != (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                       >> 7U)));
            } else if ((4U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 0xdU)))) {
                if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h77e9143c__0) {
                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write 
                        = (0U != (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                           >> 7U)));
                } else if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h24f9eeb0__0) {
                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write 
                        = (0U != (0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                           >> 7U)));
                } else if ((1U & (~ (IData)(tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hf6a39fa8__0)))) {
                    if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_ha830e83f__0) {
                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 1U;
                    }
                }
            }
        } else if ((0U == (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
            if ((0U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                              >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 1U;
            } else if ((1U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 1U;
            } else if ((2U == (7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                     >> 0xdU)))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 1U;
            }
        }
    } else {
        vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b = 
            ((0x2fU == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))
              ? 0ULL : ((2U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b))
                         ? ((1U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b))
                             ? 4ULL : vlSelfRef.tb_rv64i__DOT__cpu__DOT__imm_u)
                         : ((1U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b))
                             ? ((0x23U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))
                                 ? (((- (QData)((IData)(
                                                        (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                         >> 0x1fU)))) 
                                     << 0xcU) | (QData)((IData)(
                                                                ((0xfe0U 
                                                                  & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                     >> 0x14U)) 
                                                                 | (0x1fU 
                                                                    & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                       >> 7U))))))
                                 : vlSelfRef.tb_rv64i__DOT__cpu__DOT__imm_i)
                             : vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)));
        if (((((((((0x33U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)) 
                   | (0x13U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                  | (3U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                 | (0x23U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                | (0x63U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
               | (0x37U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
              | (0x17U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
             | (0x6fU == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)))) {
            if ((0x33U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 1U;
            } else if ((0x13U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 1U;
            } else if ((3U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 1U;
            } else if ((0x23U != (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                if ((0x63U != (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 1U;
                }
            }
        } else if ((0x67U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
            vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 1U;
        } else if ((0x2fU == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
            if ((2U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                        >> 0x1bU))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 1U;
            } else if ((3U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                               >> 0x1bU))) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 1U;
            }
        } else if ((0x73U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
            if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h0d0b5e95__0) {
                vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write = 1U;
            }
        }
    }
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_imm = 0U;
    if ((3U == (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))) {
        if ((1U & (~ ((((((((0x33U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)) 
                            | (0x13U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                           | (3U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                          | (0x23U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                         | (0x63U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                        | (0x37U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                       | (0x17U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) 
                      | (0x6fU == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)))))) {
            if ((0x67U != (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                if ((0x2fU != (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                    if ((0x73U == (0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))) {
                        if (tb_rv64i__DOT__cpu__DOT____VdfgExtracted_h0d0b5e95__0) {
                            if (((1U == (7U & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                               >> 0xcU))) 
                                 | (5U == (7U & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                 >> 0xcU))))) {
                                vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_write = 1U;
                            } else if (((2U == (7U 
                                                & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                   >> 0xcU))) 
                                        | (6U == (7U 
                                                  & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                     >> 0xcU))))) {
                                if ((0U != (0x1fU & 
                                            (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                             >> 0xfU)))) {
                                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_write = 1U;
                                }
                            } else if (((3U == (7U 
                                                & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                   >> 0xcU))) 
                                        | (7U == (7U 
                                                  & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                     >> 0xcU))))) {
                                if ((0U != (0x1fU & 
                                            (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                             >> 0xfU)))) {
                                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_write = 1U;
                                }
                            }
                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_op = 1U;
                            if ((0x4000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)) {
                                vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_imm = 1U;
                            }
                        }
                    }
                }
            }
        }
    }
    VL_EXTENDS_WQ(128,64, __Vtemp_53, vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a);
    VL_EXTENDS_WQ(128,64, __Vtemp_54, vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b);
    VL_MULS_WWW(128, vlSelfRef.tb_rv64i__DOT__cpu__DOT__mul_full_s, __Vtemp_53, __Vtemp_54);
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rs1_val 
        = ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_imm)
            ? (QData)((IData)((0x1fU & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                        >> 0xfU))))
            : vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1);
    __Vtemp_55[0U] = (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a);
    __Vtemp_55[1U] = (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a 
                              >> 0x20U));
    __Vtemp_55[2U] = 0U;
    __Vtemp_55[3U] = 0U;
    __Vtemp_56[0U] = (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b);
    __Vtemp_56[1U] = (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b 
                              >> 0x20U));
    __Vtemp_56[2U] = 0U;
    __Vtemp_56[3U] = 0U;
    VL_MUL_W(4, __Vtemp_57, __Vtemp_55, __Vtemp_56);
    VL_EXTENDS_WQ(128,64, __Vtemp_58, vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a);
    __Vtemp_59[0U] = (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b);
    __Vtemp_59[1U] = (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b 
                              >> 0x20U));
    __Vtemp_59[2U] = 0U;
    VL_EXTENDS_WW(128,65, __Vtemp_60, __Vtemp_59);
    VL_MULS_WWW(128, __Vtemp_61, __Vtemp_58, __Vtemp_60);
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result = 
        ((8U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl))
          ? ((4U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl))
              ? ((2U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl))
                  ? ((1U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl))
                      ? ((0ULL == vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b)
                          ? 0xffffffffffffffffULL : 
                         ((7U == (7U & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                        >> 0xcU))) ? 
                          VL_MODDIV_QQQ(64, vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a, vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b)
                           : VL_DIV_QQQ(64, vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a, vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b)))
                      : ((0ULL == vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b)
                          ? 0xffffffffffffffffULL : 
                         ((6U == (7U & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                        >> 0xcU))) ? 
                          VL_MODDIVS_QQQ(64, vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a, vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b)
                           : (((IData)(((0x4000U == 
                                         (0x7000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)) 
                                        & (0x8000000000000000ULL 
                                           == vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a))) 
                               & (0xffffffffffffffffULL 
                                  == vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b))
                               ? 0x8000000000000000ULL
                               : ((4U == (7U & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                >> 0xcU)))
                                   ? VL_DIVS_QQQ(64, vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a, vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b)
                                   : vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a)))))
                  : ((1U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl))
                      ? (((QData)((IData)(__Vtemp_57[3U])) 
                          << 0x20U) | (QData)((IData)(
                                                      __Vtemp_57[2U])))
                      : ((2U == (7U & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                       >> 0xcU))) ? 
                         (((QData)((IData)(__Vtemp_61[3U])) 
                           << 0x20U) | (QData)((IData)(
                                                       __Vtemp_61[2U])))
                          : (((QData)((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mul_full_s[3U])) 
                              << 0x20U) | (QData)((IData)(
                                                          vlSelfRef.tb_rv64i__DOT__cpu__DOT__mul_full_s[2U]))))))
              : ((2U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl))
                  ? ((1U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl))
                      ? (((QData)((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mul_full_s[1U])) 
                          << 0x20U) | (QData)((IData)(
                                                      vlSelfRef.tb_rv64i__DOT__cpu__DOT__mul_full_s[0U])))
                      : vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b)
                  : ((1U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl))
                      ? (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a 
                         & vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b)
                      : (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a 
                         | vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b))))
          : ((4U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl))
              ? ((2U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl))
                  ? ((1U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl))
                      ? VL_SHIFTRS_QQI(64,64,6, vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a, 
                                       (0x3fU & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b)))
                      : (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a 
                         >> (0x3fU & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b))))
                  : ((1U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl))
                      ? (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a 
                         ^ vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b)
                      : ((vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a 
                          < vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b)
                          ? 1ULL : 0ULL))) : ((2U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl))
                                               ? ((1U 
                                                   & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl))
                                                   ? 
                                                  (VL_LTS_IQQ(64, vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a, vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b)
                                                    ? 1ULL
                                                    : 0ULL)
                                                   : 
                                                  (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a 
                                                   << 
                                                   (0x3fU 
                                                    & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b))))
                                               : ((1U 
                                                   & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl))
                                                   ? 
                                                  (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a 
                                                   - vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b)
                                                   : 
                                                  (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a 
                                                   + vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b)))));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_wdata = 
        ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_op)
          ? ((0x4000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
              ? ((0x2000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                  ? ((0x1000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                      ? (vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rdata 
                         & (~ vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rs1_val))
                      : (vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rdata 
                         | vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rs1_val))
                  : vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rs1_val)
              : ((0x2000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                  ? ((0x1000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                      ? (vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rdata 
                         & (~ vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rs1_val))
                      : (vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rdata 
                         | vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rs1_val))
                  : vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rs1_val))
          : 0ULL);
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_wdata = 
        ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_sd)
          ? tb_rv64i__DOT__cpu__DOT____VdfgExtracted_he1520fff__0
          : ((0U == (7U & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                           >> 0xcU))) ? ((0U == (3U 
                                                 & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result)))
                                          ? ((0xffffff00U 
                                              & vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                              [(0x3fffU 
                                                & (IData)(
                                                          (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                           >> 2U)))]) 
                                             | (0xffU 
                                                & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)))
                                          : ((1U == 
                                              (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result)))
                                              ? ((0xffff0000U 
                                                  & vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                                  [
                                                  (0x3fffU 
                                                   & (IData)(
                                                             (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                              >> 2U)))]) 
                                                 | ((0xff00U 
                                                     & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2) 
                                                        << 8U)) 
                                                    | (0xffU 
                                                       & vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                                       [
                                                       (0x3fffU 
                                                        & (IData)(
                                                                  (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                                   >> 2U)))])))
                                              : ((2U 
                                                  == 
                                                  (3U 
                                                   & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result)))
                                                  ? 
                                                 ((0xff000000U 
                                                   & vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                                   [
                                                   (0x3fffU 
                                                    & (IData)(
                                                              (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                               >> 2U)))]) 
                                                  | ((0xff0000U 
                                                      & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2) 
                                                         << 0x10U)) 
                                                     | (0xffffU 
                                                        & vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                                        [
                                                        (0x3fffU 
                                                         & (IData)(
                                                                   (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                                    >> 2U)))])))
                                                  : 
                                                 (((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2) 
                                                   << 0x18U) 
                                                  | (0xffffffU 
                                                     & vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                                     [
                                                     (0x3fffU 
                                                      & (IData)(
                                                                (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                                 >> 2U)))])))))
              : ((1U == (7U & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                               >> 0xcU))) ? ((1U & (IData)(
                                                           (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                            >> 1U)))
                                              ? (((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2) 
                                                  << 0x10U) 
                                                 | (0xffffU 
                                                    & vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                                    [
                                                    (0x3fffU 
                                                     & (IData)(
                                                               (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                                >> 2U)))]))
                                              : ((0xffff0000U 
                                                  & vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                                  [
                                                  (0x3fffU 
                                                   & (IData)(
                                                             (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                              >> 2U)))]) 
                                                 | (0xffffU 
                                                    & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2))))
                  : tb_rv64i__DOT__cpu__DOT____VdfgExtracted_he1520fff__0)));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__data_word = 
        vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem[(0x3fffU 
                                                 & (IData)(
                                                           (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                            >> 2U)))];
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_41 
        = (0xffU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
           [(0x3fffU & (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                >> 2U)))]);
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_43 
        = (1U & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                 [(0x3fffU & (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                      >> 2U)))] >> 0xfU));
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_44 
        = (0xffU & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                    [(0x3fffU & (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                         >> 2U)))] 
                    >> 8U));
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_46 
        = (0xffU & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                    [(0x3fffU & (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                         >> 2U)))] 
                    >> 0x10U));
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_47 
        = (vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem[
           (0x3fffU & (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                               >> 2U)))] >> 0x18U);
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_48 
        = (vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem[
           (0x3fffU & (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                               >> 2U)))] >> 0x10U);
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_49 
        = (0xffffU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
           [(0x3fffU & (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                >> 2U)))]);
    tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_39 
        = (vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem[
           (0x3fffU & (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                               >> 2U)))] >> 0x1fU);
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_uart_mmio 
        = ((0x10000000ULL <= vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result) 
           & (0x10000010ULL > vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_clint_mmio 
        = ((0x2000000ULL <= vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result) 
           & (0x2010000ULL > vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result));
    tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hf9ddbd55__0 
        = (((QData)((IData)((- (IData)((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_39))))) 
            << 0x20U) | (QData)((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                        [(0x3fffU & (IData)(
                                                            (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                             >> 2U)))])));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_mmio = ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_uart_mmio) 
                                                  | (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_clint_mmio));
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__mmio_rdata = 0ULL;
    if (vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_uart_mmio) {
        if ((0U == (0xfU & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result)))) {
            vlSelfRef.tb_rv64i__DOT__cpu__DOT__mmio_rdata = 0ULL;
        } else if ((5U == (0xfU & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result)))) {
            vlSelfRef.tb_rv64i__DOT__cpu__DOT__mmio_rdata = 0x60ULL;
        }
    } else if (vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_clint_mmio) {
        if ((0xbff8U == (0xffffU & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result)))) {
            vlSelfRef.tb_rv64i__DOT__cpu__DOT__mmio_rdata 
                = vlSelfRef.tb_rv64i__DOT__cpu__DOT__clint_mtime;
        } else if ((0xbffcU == (0xffffU & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result)))) {
            vlSelfRef.tb_rv64i__DOT__cpu__DOT__mmio_rdata 
                = (QData)((IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__clint_mtime 
                                   >> 0x20U)));
        } else if ((0x4000U == (0xffffU & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result)))) {
            vlSelfRef.tb_rv64i__DOT__cpu__DOT__mmio_rdata 
                = vlSelfRef.tb_rv64i__DOT__cpu__DOT__clint_mtimecmp;
        }
    }
    vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_rdata_word 
        = ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_mmio)
            ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__mmio_rdata
            : (((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) 
                & (IData)((0x4002U == (0xe003U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))))
                ? tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hf9ddbd55__0
                : (((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) 
                    & (IData)((0x6002U == (0xe003U 
                                           & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))))
                    ? (((QData)((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                        [(0x3fffU & 
                                          ((IData)(1U) 
                                           + (IData)(
                                                     (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                      >> 2U))))])) 
                        << 0x20U) | (QData)((IData)(
                                                    vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                                    [
                                                    (0x3fffU 
                                                     & (IData)(
                                                               (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                                >> 2U)))])))
                    : (((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) 
                        & (IData)((0x2000U == (0xe003U 
                                               & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))))
                        ? (((QData)((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                            [(0x3fffU 
                                              & ((IData)(1U) 
                                                 + (IData)(
                                                           (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                            >> 2U))))])) 
                            << 0x20U) | (QData)((IData)(
                                                        vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                                        [
                                                        (0x3fffU 
                                                         & (IData)(
                                                                   (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                                    >> 2U)))])))
                        : ((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                            ? tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hf9ddbd55__0
                            : ((0x4000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                ? ((0x2000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                    ? ((0x1000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                        ? (((QData)((IData)(
                                                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                                            [
                                                            (0x3fffU 
                                                             & ((IData)(1U) 
                                                                + (IData)(
                                                                          (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                                           >> 2U))))])) 
                                            << 0x20U) 
                                           | (QData)((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__data_word)))
                                        : (QData)((IData)(
                                                          vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                                          [
                                                          (0x3fffU 
                                                           & (IData)(
                                                                     (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                                      >> 2U)))])))
                                    : ((0x1000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                        ? ((1U & (IData)(
                                                         (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                          >> 1U)))
                                            ? (QData)((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_48))
                                            : (QData)((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_49)))
                                        : ((0U == (3U 
                                                   & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result)))
                                            ? (QData)((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_41))
                                            : ((1U 
                                                == 
                                                (3U 
                                                 & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result)))
                                                ? (QData)((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_44))
                                                : (
                                                   (2U 
                                                    == 
                                                    (3U 
                                                     & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result)))
                                                    ? (QData)((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_46))
                                                    : (QData)((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_47)))))))
                                : ((0x2000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                    ? ((0x1000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                        ? (((QData)((IData)(
                                                            vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                                            [
                                                            (0x3fffU 
                                                             & ((IData)(1U) 
                                                                + (IData)(
                                                                          (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                                           >> 2U))))])) 
                                            << 0x20U) 
                                           | (QData)((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__data_word)))
                                        : tb_rv64i__DOT__cpu__DOT____VdfgExtracted_hf9ddbd55__0)
                                    : ((0x1000U & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)
                                        ? ((1U & (IData)(
                                                         (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                          >> 1U)))
                                            ? (((- (QData)((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_39))) 
                                                << 0x10U) 
                                               | (QData)((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_48)))
                                            : (((- (QData)((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_43))) 
                                                << 0x10U) 
                                               | (QData)((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_49))))
                                        : ((0U == (3U 
                                                   & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result)))
                                            ? (((- (QData)((IData)(
                                                                   (1U 
                                                                    & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                                                       [
                                                                       (0x3fffU 
                                                                        & (IData)(
                                                                                (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                                                >> 2U)))] 
                                                                       >> 7U))))) 
                                                << 8U) 
                                               | (QData)((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_41)))
                                            : ((1U 
                                                == 
                                                (3U 
                                                 & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result)))
                                                ? (
                                                   ((- (QData)((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_43))) 
                                                    << 8U) 
                                                   | (QData)((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_44)))
                                                : (
                                                   (2U 
                                                    == 
                                                    (3U 
                                                     & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result)))
                                                    ? 
                                                   (((- (QData)((IData)(
                                                                        (1U 
                                                                         & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                                                                            [
                                                                            (0x3fffU 
                                                                             & (IData)(
                                                                                (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                                                >> 2U)))] 
                                                                            >> 0x17U))))) 
                                                     << 8U) 
                                                    | (QData)((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_46)))
                                                    : 
                                                   (((- (QData)((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_39))) 
                                                     << 8U) 
                                                    | (QData)((IData)(tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_47))))))))))))));
}

VL_ATTR_COLD void Vtb_rv64i___024root___eval_triggers__stl(Vtb_rv64i___024root* vlSelf);

VL_ATTR_COLD bool Vtb_rv64i___024root___eval_phase__stl(Vtb_rv64i___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root___eval_phase__stl\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    CData/*0:0*/ __VstlExecute;
    // Body
    Vtb_rv64i___024root___eval_triggers__stl(vlSelf);
    __VstlExecute = vlSelfRef.__VstlTriggered.any();
    if (__VstlExecute) {
        Vtb_rv64i___024root___eval_stl(vlSelf);
    }
    return (__VstlExecute);
}

#ifdef VL_DEBUG
VL_ATTR_COLD void Vtb_rv64i___024root___dump_triggers__act(Vtb_rv64i___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root___dump_triggers__act\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if ((1U & (~ vlSelfRef.__VactTriggered.any()))) {
        VL_DBG_MSGF("         No triggers active\n");
    }
    if ((1ULL & vlSelfRef.__VactTriggered.word(0U))) {
        VL_DBG_MSGF("         'act' region trigger index 0 is active: @(posedge tb_rv64i.clk)\n");
    }
    if ((2ULL & vlSelfRef.__VactTriggered.word(0U))) {
        VL_DBG_MSGF("         'act' region trigger index 1 is active: @(negedge tb_rv64i.clk)\n");
    }
    if ((4ULL & vlSelfRef.__VactTriggered.word(0U))) {
        VL_DBG_MSGF("         'act' region trigger index 2 is active: @(posedge tb_rv64i.clk or negedge tb_rv64i.rst_n)\n");
    }
    if ((8ULL & vlSelfRef.__VactTriggered.word(0U))) {
        VL_DBG_MSGF("         'act' region trigger index 3 is active: @([true] __VdlySched.awaitingCurrentTime())\n");
    }
}
#endif  // VL_DEBUG

#ifdef VL_DEBUG
VL_ATTR_COLD void Vtb_rv64i___024root___dump_triggers__nba(Vtb_rv64i___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root___dump_triggers__nba\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if ((1U & (~ vlSelfRef.__VnbaTriggered.any()))) {
        VL_DBG_MSGF("         No triggers active\n");
    }
    if ((1ULL & vlSelfRef.__VnbaTriggered.word(0U))) {
        VL_DBG_MSGF("         'nba' region trigger index 0 is active: @(posedge tb_rv64i.clk)\n");
    }
    if ((2ULL & vlSelfRef.__VnbaTriggered.word(0U))) {
        VL_DBG_MSGF("         'nba' region trigger index 1 is active: @(negedge tb_rv64i.clk)\n");
    }
    if ((4ULL & vlSelfRef.__VnbaTriggered.word(0U))) {
        VL_DBG_MSGF("         'nba' region trigger index 2 is active: @(posedge tb_rv64i.clk or negedge tb_rv64i.rst_n)\n");
    }
    if ((8ULL & vlSelfRef.__VnbaTriggered.word(0U))) {
        VL_DBG_MSGF("         'nba' region trigger index 3 is active: @([true] __VdlySched.awaitingCurrentTime())\n");
    }
}
#endif  // VL_DEBUG

VL_ATTR_COLD void Vtb_rv64i___024root___ctor_var_reset(Vtb_rv64i___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root___ctor_var_reset\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelf->tb_rv64i__DOT__clk = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__rst_n = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__i = VL_RAND_RESET_I(32);
    vlSelf->tb_rv64i__DOT__j = VL_RAND_RESET_I(32);
    vlSelf->tb_rv64i__DOT__word_val = VL_RAND_RESET_I(32);
    vlSelf->tb_rv64i__DOT__byte_val = VL_RAND_RESET_I(8);
    vlSelf->tb_rv64i__DOT__term = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__uart_active = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__trace_enabled = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__sa0 = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__sa7 = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__trace_cnt = VL_RAND_RESET_I(32);
    vlSelf->tb_rv64i__DOT__cpu__DOT__pc = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__pc_next = VL_RAND_RESET_Q(64);
    for (int __Vi0 = 0; __Vi0 < 16384; ++__Vi0) {
        vlSelf->tb_rv64i__DOT__cpu__DOT__imem[__Vi0] = VL_RAND_RESET_I(32);
    }
    vlSelf->tb_rv64i__DOT__cpu__DOT__upper_half = VL_RAND_RESET_I(16);
    vlSelf->tb_rv64i__DOT__cpu__DOT__fetch_half = VL_RAND_RESET_I(16);
    vlSelf->tb_rv64i__DOT__cpu__DOT__instr = VL_RAND_RESET_I(32);
    vlSelf->tb_rv64i__DOT__cpu__DOT__init_i = VL_RAND_RESET_I(32);
    vlSelf->tb_rv64i__DOT__cpu__DOT__rvc_rd5 = VL_RAND_RESET_I(5);
    vlSelf->tb_rv64i__DOT__cpu__DOT__rvc_rs15 = VL_RAND_RESET_I(5);
    vlSelf->tb_rv64i__DOT__cpu__DOT__rvc_rs25 = VL_RAND_RESET_I(5);
    vlSelf->tb_rv64i__DOT__cpu__DOT__rvc_imm_i6 = VL_RAND_RESET_I(6);
    vlSelf->tb_rv64i__DOT__cpu__DOT__rvc_imm = VL_RAND_RESET_Q(64);
    for (int __Vi0 = 0; __Vi0 < 32; ++__Vi0) {
        vlSelf->tb_rv64i__DOT__cpu__DOT__rf[__Vi0] = VL_RAND_RESET_Q(64);
    }
    vlSelf->tb_rv64i__DOT__cpu__DOT__rf_rdata1 = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__rf_rdata2 = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__data_word = VL_RAND_RESET_I(32);
    vlSelf->tb_rv64i__DOT__cpu__DOT__is_sd = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT__is_uart_mmio = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT__is_clint_mmio = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT__is_mmio = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT__imm_i = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__imm_u = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__mcycle = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__minstret = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__mstatus = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__mie = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__mtvec = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__mscratch = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__mepc = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__mcause = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__clint_mtime = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__clint_mtimecmp = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__timer_irq = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT__csr_rdata = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__csr_write = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT__csr_wdata = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__is_ecall = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT__is_mret = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT__reg_write = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT__mem_write = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT__branch = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT__jump = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT__jalr = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT__reg_src = VL_RAND_RESET_I(2);
    vlSelf->tb_rv64i__DOT__cpu__DOT__alu_src_a = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT__alu_src_b = VL_RAND_RESET_I(2);
    vlSelf->tb_rv64i__DOT__cpu__DOT__csr_op = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT__csr_imm = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT__alu_b = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__alu_ctrl = VL_RAND_RESET_I(4);
    vlSelf->tb_rv64i__DOT__cpu__DOT__alu_a = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__rvc_alu_imm = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__csr_rs1_val = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__alu_result = VL_RAND_RESET_Q(64);
    VL_RAND_RESET_W(128, vlSelf->tb_rv64i__DOT__cpu__DOT__mul_full_s);
    vlSelf->tb_rv64i__DOT__cpu__DOT__mmio_rdata = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__mem_rdata_word = VL_RAND_RESET_Q(64);
    vlSelf->tb_rv64i__DOT__cpu__DOT__mem_wdata = VL_RAND_RESET_I(32);
    vlSelf->tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_1 = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_38 = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_55 = VL_RAND_RESET_I(1);
    vlSelf->tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_59 = VL_RAND_RESET_I(1);
    vlSelf->__Vtrigprevexpr___TOP__tb_rv64i__DOT__clk__0 = VL_RAND_RESET_I(1);
    vlSelf->__Vtrigprevexpr___TOP__tb_rv64i__DOT__rst_n__0 = VL_RAND_RESET_I(1);
    for (int __Vi0 = 0; __Vi0 < 7; ++__Vi0) {
        vlSelf->__Vm_traceActivity[__Vi0] = 0;
    }
}
