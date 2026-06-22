// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Tracing implementation internals
#include "verilated_vcd_c.h"
#include "Vtb_rv64i__Syms.h"


void Vtb_rv64i___024root__trace_chg_0_sub_0(Vtb_rv64i___024root* vlSelf, VerilatedVcd::Buffer* bufp);

void Vtb_rv64i___024root__trace_chg_0(void* voidSelf, VerilatedVcd::Buffer* bufp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root__trace_chg_0\n"); );
    // Init
    Vtb_rv64i___024root* const __restrict vlSelf VL_ATTR_UNUSED = static_cast<Vtb_rv64i___024root*>(voidSelf);
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    if (VL_UNLIKELY(!vlSymsp->__Vm_activity)) return;
    // Body
    Vtb_rv64i___024root__trace_chg_0_sub_0((&vlSymsp->TOP), bufp);
}

void Vtb_rv64i___024root__trace_chg_0_sub_0(Vtb_rv64i___024root* vlSelf, VerilatedVcd::Buffer* bufp) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root__trace_chg_0_sub_0\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    uint32_t* const oldp VL_ATTR_UNUSED = bufp->oldp(vlSymsp->__Vm_baseCode + 1);
    VlWide<4>/*127:0*/ __Vtemp_1;
    VlWide<4>/*127:0*/ __Vtemp_2;
    VlWide<4>/*127:0*/ __Vtemp_3;
    VlWide<4>/*127:0*/ __Vtemp_4;
    VlWide<3>/*95:0*/ __Vtemp_5;
    VlWide<4>/*127:0*/ __Vtemp_6;
    VlWide<4>/*127:0*/ __Vtemp_7;
    // Body
    if (VL_UNLIKELY((vlSelfRef.__Vm_traceActivity[1U] 
                     | vlSelfRef.__Vm_traceActivity
                     [3U]))) {
        bufp->chgIData(oldp+0,(vlSelfRef.tb_rv64i__DOT__trace_cnt),32);
        bufp->chgQData(oldp+1,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[0]),64);
        bufp->chgQData(oldp+3,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[1]),64);
        bufp->chgQData(oldp+5,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[2]),64);
        bufp->chgQData(oldp+7,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[3]),64);
        bufp->chgQData(oldp+9,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[4]),64);
        bufp->chgQData(oldp+11,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[5]),64);
        bufp->chgQData(oldp+13,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[6]),64);
        bufp->chgQData(oldp+15,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[7]),64);
        bufp->chgQData(oldp+17,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[8]),64);
        bufp->chgQData(oldp+19,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[9]),64);
        bufp->chgQData(oldp+21,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[10]),64);
        bufp->chgQData(oldp+23,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[11]),64);
        bufp->chgQData(oldp+25,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[12]),64);
        bufp->chgQData(oldp+27,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[13]),64);
        bufp->chgQData(oldp+29,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[14]),64);
        bufp->chgQData(oldp+31,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[15]),64);
        bufp->chgQData(oldp+33,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[16]),64);
        bufp->chgQData(oldp+35,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[17]),64);
        bufp->chgQData(oldp+37,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[18]),64);
        bufp->chgQData(oldp+39,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[19]),64);
        bufp->chgQData(oldp+41,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[20]),64);
        bufp->chgQData(oldp+43,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[21]),64);
        bufp->chgQData(oldp+45,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[22]),64);
        bufp->chgQData(oldp+47,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[23]),64);
        bufp->chgQData(oldp+49,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[24]),64);
        bufp->chgQData(oldp+51,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[25]),64);
        bufp->chgQData(oldp+53,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[26]),64);
        bufp->chgQData(oldp+55,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[27]),64);
        bufp->chgQData(oldp+57,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[28]),64);
        bufp->chgQData(oldp+59,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[29]),64);
        bufp->chgQData(oldp+61,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[30]),64);
        bufp->chgQData(oldp+63,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf[31]),64);
    }
    if (VL_UNLIKELY(vlSelfRef.__Vm_traceActivity[2U])) {
        bufp->chgQData(oldp+65,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mcycle),64);
        bufp->chgQData(oldp+67,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__minstret),64);
        bufp->chgQData(oldp+69,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mstatus),64);
        bufp->chgQData(oldp+71,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mie),64);
        bufp->chgQData(oldp+73,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mtvec),64);
        bufp->chgQData(oldp+75,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mscratch),64);
        bufp->chgQData(oldp+77,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mepc),64);
        bufp->chgQData(oldp+79,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mcause),64);
        bufp->chgQData(oldp+81,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__clint_mtime),64);
        bufp->chgQData(oldp+83,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__clint_mtimecmp),64);
        bufp->chgBit(oldp+85,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__timer_irq));
    }
    if (VL_UNLIKELY(vlSelfRef.__Vm_traceActivity[4U])) {
        bufp->chgIData(oldp+86,(vlSelfRef.tb_rv64i__DOT__j),32);
        bufp->chgIData(oldp+87,(vlSelfRef.tb_rv64i__DOT__word_val),32);
        bufp->chgCData(oldp+88,(vlSelfRef.tb_rv64i__DOT__byte_val),8);
        bufp->chgQData(oldp+89,(vlSelfRef.tb_rv64i__DOT__sa0),64);
        bufp->chgQData(oldp+91,(vlSelfRef.tb_rv64i__DOT__sa7),64);
    }
    if (VL_UNLIKELY(vlSelfRef.__Vm_traceActivity[5U])) {
        bufp->chgQData(oldp+93,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc),64);
        bufp->chgSData(oldp+95,((0x3fffU & (IData)(
                                                   (vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                                    >> 2U)))),14);
        bufp->chgBit(oldp+96,((1U & (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                             >> 1U)))));
    }
    if (VL_UNLIKELY(vlSelfRef.__Vm_traceActivity[6U])) {
        bufp->chgBit(oldp+97,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_ecall));
        bufp->chgQData(oldp+98,(((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                  ? 2ULL : 4ULL)),64);
        bufp->chgSData(oldp+100,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__upper_half),16);
        bufp->chgSData(oldp+101,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half),16);
        bufp->chgBit(oldp+102,((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))));
        bufp->chgIData(oldp+103,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr),32);
        bufp->chgCData(oldp+104,((0x7fU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr)),7);
        bufp->chgCData(oldp+105,((0x1fU & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                           >> 7U))),5);
        bufp->chgCData(oldp+106,((7U & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                        >> 0xcU))),3);
        bufp->chgCData(oldp+107,((0x1fU & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                           >> 0xfU))),5);
        bufp->chgCData(oldp+108,((0x1fU & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                           >> 0x14U))),5);
        bufp->chgCData(oldp+109,((vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                  >> 0x19U)),7);
        bufp->chgCData(oldp+110,((vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                  >> 0x1bU)),5);
        bufp->chgBit(oldp+111,(((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_1) 
                                & (IData)((0x10002000U 
                                           == (0xf8007000U 
                                               & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))))));
        bufp->chgBit(oldp+112,(((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_1) 
                                & (IData)((0x18002000U 
                                           == (0xf8007000U 
                                               & vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr))))));
        bufp->chgCData(oldp+113,((3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))),2);
        bufp->chgCData(oldp+114,((7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                        >> 0xdU))),3);
        bufp->chgBit(oldp+115,((1U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                      >> 0xcU))));
        bufp->chgCData(oldp+116,((0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                           >> 7U))),5);
        bufp->chgCData(oldp+117,((0x1fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                           >> 2U))),5);
        bufp->chgCData(oldp+118,((7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                        >> 2U))),3);
        bufp->chgCData(oldp+119,((7U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                        >> 7U))),3);
        bufp->chgCData(oldp+120,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rd5),5);
        bufp->chgCData(oldp+121,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rs15),5);
        bufp->chgCData(oldp+122,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_rs25),5);
        bufp->chgCData(oldp+123,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm_i6),6);
        bufp->chgQData(oldp+124,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm),64);
        bufp->chgSData(oldp+126,(((0x800U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                             >> 1U)) 
                                  | ((0x400U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                << 2U)) 
                                     | ((0x300U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                   >> 1U)) 
                                        | ((0x80U & 
                                            ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
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
                                                          >> 2U)))))))))),12);
        bufp->chgQData(oldp+127,((((- (QData)((IData)(
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
        bufp->chgSData(oldp+129,(((0x100U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                             >> 4U)) 
                                  | ((0xc0U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                               << 1U)) 
                                     | ((0x20U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                  << 3U)) 
                                        | ((0x18U & 
                                            ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                             >> 7U)) 
                                           | (6U & 
                                              ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                               >> 2U))))))),9);
        bufp->chgQData(oldp+130,((((- (QData)((IData)(
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
        bufp->chgSData(oldp+132,(((0x3c0U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                             >> 1U)) 
                                  | ((0x30U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                               >> 7U)) 
                                     | ((8U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                               >> 2U)) 
                                        | (4U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                 >> 4U)))))),10);
        bufp->chgQData(oldp+133,((QData)((IData)(((0x3c0U 
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
        bufp->chgCData(oldp+135,(((0xc0U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                            >> 4U)) 
                                  | ((0x30U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                               >> 2U)) 
                                     | (8U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                              >> 9U))))),8);
        bufp->chgQData(oldp+136,((QData)((IData)(((0xc0U 
                                                   & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                      >> 4U)) 
                                                  | ((0x30U 
                                                      & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                         >> 2U)) 
                                                     | (8U 
                                                        & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                           >> 9U))))))),64);
        bufp->chgCData(oldp+138,(((0x40U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                            << 1U)) 
                                  | ((0x38U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                               >> 7U)) 
                                     | (4U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                              >> 4U))))),7);
        bufp->chgQData(oldp+139,((QData)((IData)(((0x40U 
                                                   & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                      << 1U)) 
                                                  | ((0x38U 
                                                      & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                         >> 7U)) 
                                                     | (4U 
                                                        & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                           >> 4U))))))),64);
        bufp->chgSData(oldp+141,(((0x200U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                             >> 3U)) 
                                  | ((0x180U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                << 4U)) 
                                     | ((0x40U & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                  << 1U)) 
                                        | ((0x20U & 
                                            ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                             << 3U)) 
                                           | (0x10U 
                                              & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                 >> 2U))))))),10);
        bufp->chgQData(oldp+142,((((- (QData)((IData)(
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
        bufp->chgQData(oldp+144,(((0x1ffffffffffc0000ULL 
                                   & ((- (QData)((IData)(
                                                         (1U 
                                                          & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                             >> 0xcU))))) 
                                      << 0x12U)) | (QData)((IData)(
                                                                   ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm_i6) 
                                                                    << 0xcU))))),64);
        bufp->chgQData(oldp+146,((QData)((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_imm_i6))),64);
        bufp->chgCData(oldp+148,((0x3fU & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                           >> 7U))),6);
        bufp->chgQData(oldp+149,((QData)((IData)((0x3fU 
                                                  & ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                     >> 7U))))),64);
        bufp->chgQData(oldp+151,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1),64);
        bufp->chgQData(oldp+153,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2),64);
        bufp->chgSData(oldp+155,((0x3fffU & (IData)(
                                                    (vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result 
                                                     >> 2U)))),14);
        bufp->chgCData(oldp+156,((3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result))),2);
        bufp->chgIData(oldp+157,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__data_word),32);
        bufp->chgBit(oldp+158,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_sd));
        bufp->chgBit(oldp+159,(((0x80000000ULL <= vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result) 
                                & (0x80040000ULL > vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result))));
        bufp->chgBit(oldp+160,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_uart_mmio));
        bufp->chgBit(oldp+161,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_clint_mmio));
        bufp->chgBit(oldp+162,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_mmio));
        bufp->chgQData(oldp+163,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__imm_i),64);
        bufp->chgQData(oldp+165,((((- (QData)((IData)(
                                                      (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                       >> 0x1fU)))) 
                                   << 0xcU) | (QData)((IData)(
                                                              ((0xfe0U 
                                                                & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                   >> 0x14U)) 
                                                               | (0x1fU 
                                                                  & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                                     >> 7U))))))),64);
        bufp->chgQData(oldp+167,((((- (QData)((IData)(
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
        bufp->chgQData(oldp+169,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__imm_u),64);
        bufp->chgQData(oldp+171,((((- (QData)((IData)(
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
        bufp->chgQData(oldp+173,((QData)((IData)((0x1fU 
                                                  & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                     >> 0xfU))))),64);
        bufp->chgQData(oldp+175,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rdata),64);
        bufp->chgBit(oldp+177,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_write));
        bufp->chgQData(oldp+178,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_wdata),64);
        bufp->chgSData(oldp+180,((vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                  >> 0x14U)),12);
        bufp->chgBit(oldp+181,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_mret));
        bufp->chgBit(oldp+182,(((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT____VdfgRegularize_hca2283cc_0_55) 
                                & (0x105U == (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                              >> 0x14U)))));
        bufp->chgBit(oldp+183,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_write));
        bufp->chgBit(oldp+184,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_write));
        bufp->chgBit(oldp+185,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__branch));
        bufp->chgBit(oldp+186,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__jump));
        bufp->chgBit(oldp+187,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__jalr));
        bufp->chgCData(oldp+188,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src),2);
        bufp->chgBit(oldp+189,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_a));
        bufp->chgCData(oldp+190,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_src_b),2);
        bufp->chgBit(oldp+191,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_op));
        bufp->chgBit(oldp+192,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_imm));
        bufp->chgQData(oldp+193,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b),64);
        bufp->chgCData(oldp+195,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_ctrl),4);
        bufp->chgQData(oldp+196,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a),64);
        bufp->chgQData(oldp+198,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__rvc_alu_imm),64);
        bufp->chgQData(oldp+200,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rs1_val),64);
        bufp->chgQData(oldp+202,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result),64);
        bufp->chgWData(oldp+204,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mul_full_s),128);
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
        bufp->chgWData(oldp+208,(__Vtemp_3),128);
        VL_EXTENDS_WQ(128,64, __Vtemp_4, vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_a);
        __Vtemp_5[0U] = (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b);
        __Vtemp_5[1U] = (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_b 
                                 >> 0x20U));
        __Vtemp_5[2U] = 0U;
        VL_EXTENDS_WW(128,65, __Vtemp_6, __Vtemp_5);
        VL_MULS_WWW(128, __Vtemp_7, __Vtemp_4, __Vtemp_6);
        bufp->chgWData(oldp+212,(__Vtemp_7),128);
        bufp->chgBit(oldp+216,(((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
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
                                                                >= vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)))))))))));
        bufp->chgBit(oldp+217,(((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__branch) 
                                & ((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                    ? ((6U == (7U & 
                                               ((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half) 
                                                >> 0xdU)))
                                        ? (0ULL == vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1)
                                        : (IData)((
                                                   (0xe000U 
                                                    == 
                                                    (0xe000U 
                                                     & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half))) 
                                                   & (0ULL 
                                                      != vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1))))
                                    : ((0U == (7U & 
                                               (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                >> 0xcU)))
                                        ? (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                           == vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                        : ((1U == (7U 
                                                   & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                      >> 0xcU)))
                                            ? (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                               != vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                            : ((4U 
                                                == 
                                                (7U 
                                                 & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                    >> 0xcU)))
                                                ? VL_LTS_IQQ(64, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1, vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                                : (
                                                   (5U 
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
        bufp->chgQData(oldp+218,((0xfffffffffffffffeULL 
                                  & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                     + ((3U != (3U 
                                                & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                         ? 0ULL : vlSelfRef.tb_rv64i__DOT__cpu__DOT__imm_i)))),64);
        bufp->chgQData(oldp+220,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mmio_rdata),64);
        bufp->chgQData(oldp+222,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_rdata_word),64);
        bufp->chgIData(oldp+224,(vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_wdata),32);
    }
    bufp->chgBit(oldp+225,(vlSelfRef.tb_rv64i__DOT__clk));
    bufp->chgBit(oldp+226,(vlSelfRef.tb_rv64i__DOT__rst_n));
    bufp->chgIData(oldp+227,(vlSelfRef.tb_rv64i__DOT__i),32);
    bufp->chgBit(oldp+228,(vlSelfRef.tb_rv64i__DOT__term));
    bufp->chgBit(oldp+229,(vlSelfRef.tb_rv64i__DOT__uart_active));
    bufp->chgBit(oldp+230,(vlSelfRef.tb_rv64i__DOT__trace_enabled));
    bufp->chgQData(oldp+231,(((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__is_mret)
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
                                           & ((3U != 
                                               (3U 
                                                & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                               ? ((6U 
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
                                               : ((0U 
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
    bufp->chgSData(oldp+233,((0xffffU & vlSelfRef.tb_rv64i__DOT__cpu__DOT__imem
                              [(0x3fffU & (IData)((vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
                                                   >> 2U)))])),16);
    bufp->chgQData(oldp+234,((vlSelfRef.tb_rv64i__DOT__cpu__DOT__pc 
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
    bufp->chgQData(oldp+236,(((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__jump)
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
                                           : ((1U == 
                                               (7U 
                                                & (vlSelfRef.tb_rv64i__DOT__cpu__DOT__instr 
                                                   >> 0xcU)))
                                               ? (vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata1 
                                                  != vlSelfRef.tb_rv64i__DOT__cpu__DOT__rf_rdata2)
                                               : ((4U 
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
    bufp->chgQData(oldp+238,(((IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_op)
                               ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__csr_rdata
                               : ((3U != (3U & (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__fetch_half)))
                                   ? ((0U == (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src))
                                       ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__alu_result
                                       : ((1U == (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src))
                                           ? vlSelfRef.tb_rv64i__DOT__cpu__DOT__mem_rdata_word
                                           : ((2U == (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src))
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
                                           : ((2U == (IData)(vlSelfRef.tb_rv64i__DOT__cpu__DOT__reg_src))
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

void Vtb_rv64i___024root__trace_cleanup(void* voidSelf, VerilatedVcd* /*unused*/) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtb_rv64i___024root__trace_cleanup\n"); );
    // Init
    Vtb_rv64i___024root* const __restrict vlSelf VL_ATTR_UNUSED = static_cast<Vtb_rv64i___024root*>(voidSelf);
    Vtb_rv64i__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    // Body
    vlSymsp->__Vm_activity = false;
    vlSymsp->TOP.__Vm_traceActivity[0U] = 0U;
    vlSymsp->TOP.__Vm_traceActivity[1U] = 0U;
    vlSymsp->TOP.__Vm_traceActivity[2U] = 0U;
    vlSymsp->TOP.__Vm_traceActivity[3U] = 0U;
    vlSymsp->TOP.__Vm_traceActivity[4U] = 0U;
    vlSymsp->TOP.__Vm_traceActivity[5U] = 0U;
    vlSymsp->TOP.__Vm_traceActivity[6U] = 0U;
}
