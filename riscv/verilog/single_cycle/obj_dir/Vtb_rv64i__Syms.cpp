// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Symbol table implementation internals

#include "Vtb_rv64i__pch.h"
#include "Vtb_rv64i.h"
#include "Vtb_rv64i___024root.h"

// FUNCTIONS
Vtb_rv64i__Syms::~Vtb_rv64i__Syms()
{
}

Vtb_rv64i__Syms::Vtb_rv64i__Syms(VerilatedContext* contextp, const char* namep, Vtb_rv64i* modelp)
    : VerilatedSyms{contextp}
    // Setup internal state of the Syms class
    , __Vm_modelp{modelp}
    // Setup module instances
    , TOP{this, namep}
{
        // Check resources
        Verilated::stackCheck(1034);
    // Configure time unit / time precision
    _vm_contextp__->timeunit(-9);
    _vm_contextp__->timeprecision(-12);
    // Setup each module's pointers to their submodules
    // Setup each module's pointer back to symbol table (for public functions)
    TOP.__Vconfigure(true);
}
