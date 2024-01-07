// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design internal header
// See Vsubtractor.h for the primary calling header

#ifndef VERILATED_VSUBTRACTOR___024ROOT_H_
#define VERILATED_VSUBTRACTOR___024ROOT_H_  // guard

#include "verilated.h"


class Vsubtractor__Syms;

class alignas(VL_CACHE_LINE_BYTES) Vsubtractor___024root final : public VerilatedModule {
  public:

    // DESIGN SPECIFIC STATE
    VL_IN8(clk,0,0);
    VL_IN8(reset,0,0);
    VL_IN8(start_port,0,0);
    VL_OUT8(done_port,0,0);
    CData/*0:0*/ subtractor__DOT___subtractor_i0__DOT__Controller_i__DOT___present_state;
    CData/*0:0*/ subtractor__DOT___subtractor_i0__DOT__Controller_i__DOT___next_state;
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
    Vsubtractor__Syms* const vlSymsp;

    // CONSTRUCTORS
    Vsubtractor___024root(Vsubtractor__Syms* symsp, const char* v__name);
    ~Vsubtractor___024root();
    VL_UNCOPYABLE(Vsubtractor___024root);

    // INTERNAL METHODS
    void __Vconfigure(bool first);
};


#endif  // guard
