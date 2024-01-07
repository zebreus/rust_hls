// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design internal header
// See Vmultiplier.h for the primary calling header

#ifndef VERILATED_VMULTIPLIER___024ROOT_H_
#define VERILATED_VMULTIPLIER___024ROOT_H_  // guard

#include "verilated.h"


class Vmultiplier__Syms;

class alignas(VL_CACHE_LINE_BYTES) Vmultiplier___024root final : public VerilatedModule {
  public:

    // DESIGN SPECIFIC STATE
    VL_IN8(clk,0,0);
    VL_IN8(reset,0,0);
    VL_IN8(start_port,0,0);
    VL_OUT8(done_port,0,0);
    CData/*0:0*/ multiplier__DOT___multiplier_i0__DOT__Controller_i__DOT___present_state;
    CData/*0:0*/ multiplier__DOT___multiplier_i0__DOT__Controller_i__DOT___next_state;
    CData/*0:0*/ __VstlFirstIteration;
    CData/*0:0*/ __VicoFirstIteration;
    CData/*0:0*/ __Vtrigprevexpr___TOP__clk__0;
    CData/*0:0*/ __VactContinue;
    VL_IN(Pd5,31,0);
    VL_IN(Pd6,31,0);
    VL_OUT(return_port,31,0);
    IData/*31:0*/ __VactIterCount;
    VlTriggerVec<1> __VstlTriggered;
    VlTriggerVec<1> __VicoTriggered;
    VlTriggerVec<1> __VactTriggered;
    VlTriggerVec<1> __VnbaTriggered;

    // INTERNAL VARIABLES
    Vmultiplier__Syms* const vlSymsp;

    // CONSTRUCTORS
    Vmultiplier___024root(Vmultiplier__Syms* symsp, const char* v__name);
    ~Vmultiplier___024root();
    VL_UNCOPYABLE(Vmultiplier___024root);

    // INTERNAL METHODS
    void __Vconfigure(bool first);
};


#endif  // guard
