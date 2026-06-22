// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vtb_rv64i.h for the primary calling header

#include "Vtb_rv64i__pch.h"
#include "Vtb_rv64i__Syms.h"
#include "Vtb_rv64i___024root.h"

void Vtb_rv64i___024root___ctor_var_reset(Vtb_rv64i___024root* vlSelf);

Vtb_rv64i___024root::Vtb_rv64i___024root(Vtb_rv64i__Syms* symsp, const char* v__name)
    : VerilatedModule{v__name}
    , __VdlySched{*symsp->_vm_contextp__}
    , vlSymsp{symsp}
 {
    // Reset structure values
    Vtb_rv64i___024root___ctor_var_reset(this);
}

void Vtb_rv64i___024root::__Vconfigure(bool first) {
    (void)first;  // Prevent unused variable warning
}

Vtb_rv64i___024root::~Vtb_rv64i___024root() {
}
