// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vadder.h for the primary calling header

#include "Vadder.h"
#include "Vadder__Syms.h"

//==========

Vadder::Vadder(const char* __VCname)
    : VerilatedModule(__VCname)
 {
    Vadder__Syms* __restrict vlSymsp = __VlSymsp = new Vadder__Syms(this, name());
    Vadder* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Reset internal values
    
    // Reset structure values
    _ctor_var_reset();
}

void Vadder::__Vconfigure(Vadder__Syms* vlSymsp, bool first) {
    if (false && first) {}  // Prevent unused
    this->__VlSymsp = vlSymsp;
    if (false && this->__VlSymsp) {}  // Prevent unused
    Verilated::timeunit(-9);
    Verilated::timeprecision(-12);
}

Vadder::~Vadder() {
    VL_DO_CLEAR(delete __VlSymsp, __VlSymsp = nullptr);
}

void Vadder::_initial__TOP__1(Vadder__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vadder::_initial__TOP__1\n"); );
    Vadder* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->adder__DOT___adder_i0__DOT__Controller_i__DOT___present_state = 1U;
}

void Vadder::_settle__TOP__3(Vadder__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vadder::_settle__TOP__3\n"); );
    Vadder* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->return_port = (vlTOPp->Pd6 + vlTOPp->Pd5);
    vlTOPp->adder__DOT___adder_i0__DOT__Controller_i__DOT___next_state = 1U;
    vlTOPp->done_port = 0U;
    if (vlTOPp->adder__DOT___adder_i0__DOT__Controller_i__DOT___present_state) {
        if (vlTOPp->start_port) {
            vlTOPp->done_port = 1U;
        }
    }
}

void Vadder::_eval_initial(Vadder__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vadder::_eval_initial\n"); );
    Vadder* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->_initial__TOP__1(vlSymsp);
    vlTOPp->__Vclklast__TOP__clk = vlTOPp->clk;
}

void Vadder::final() {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vadder::final\n"); );
    // Variables
    Vadder__Syms* __restrict vlSymsp = this->__VlSymsp;
    Vadder* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
}

void Vadder::_eval_settle(Vadder__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vadder::_eval_settle\n"); );
    Vadder* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->_settle__TOP__3(vlSymsp);
}

void Vadder::_ctor_var_reset() {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vadder::_ctor_var_reset\n"); );
    // Body
    clk = VL_RAND_RESET_I(1);
    reset = VL_RAND_RESET_I(1);
    start_port = VL_RAND_RESET_I(1);
    Pd5 = VL_RAND_RESET_I(32);
    Pd6 = VL_RAND_RESET_I(32);
    done_port = VL_RAND_RESET_I(1);
    return_port = VL_RAND_RESET_I(32);
    adder__DOT___adder_i0__DOT__Controller_i__DOT___present_state = VL_RAND_RESET_I(1);
    adder__DOT___adder_i0__DOT__Controller_i__DOT___next_state = VL_RAND_RESET_I(1);
    for (int __Vi0=0; __Vi0<1; ++__Vi0) {
        __Vm_traceActivity[__Vi0] = VL_RAND_RESET_I(1);
    }
}
