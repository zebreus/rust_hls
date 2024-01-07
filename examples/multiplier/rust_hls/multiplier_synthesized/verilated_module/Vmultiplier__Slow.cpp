// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vmultiplier.h for the primary calling header

#include "Vmultiplier.h"
#include "Vmultiplier__Syms.h"

//==========

Vmultiplier::Vmultiplier(const char* __VCname)
    : VerilatedModule(__VCname)
 {
    Vmultiplier__Syms* __restrict vlSymsp = __VlSymsp = new Vmultiplier__Syms(this, name());
    Vmultiplier* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Reset internal values
    
    // Reset structure values
    _ctor_var_reset();
}

void Vmultiplier::__Vconfigure(Vmultiplier__Syms* vlSymsp, bool first) {
    if (false && first) {}  // Prevent unused
    this->__VlSymsp = vlSymsp;
    if (false && this->__VlSymsp) {}  // Prevent unused
    Verilated::timeunit(-9);
    Verilated::timeprecision(-12);
}

Vmultiplier::~Vmultiplier() {
    VL_DO_CLEAR(delete __VlSymsp, __VlSymsp = nullptr);
}

void Vmultiplier::_initial__TOP__1(Vmultiplier__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vmultiplier::_initial__TOP__1\n"); );
    Vmultiplier* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->multiplier__DOT___multiplier_i0__DOT__Controller_i__DOT___present_state = 1U;
}

void Vmultiplier::_settle__TOP__3(Vmultiplier__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vmultiplier::_settle__TOP__3\n"); );
    Vmultiplier* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->return_port = (vlTOPp->Pd6 * vlTOPp->Pd5);
    vlTOPp->multiplier__DOT___multiplier_i0__DOT__Controller_i__DOT___next_state = 1U;
    vlTOPp->done_port = 0U;
    if (vlTOPp->multiplier__DOT___multiplier_i0__DOT__Controller_i__DOT___present_state) {
        if (vlTOPp->start_port) {
            vlTOPp->done_port = 1U;
        }
    }
}

void Vmultiplier::_eval_initial(Vmultiplier__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vmultiplier::_eval_initial\n"); );
    Vmultiplier* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->_initial__TOP__1(vlSymsp);
    vlTOPp->__Vclklast__TOP__clk = vlTOPp->clk;
}

void Vmultiplier::final() {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vmultiplier::final\n"); );
    // Variables
    Vmultiplier__Syms* __restrict vlSymsp = this->__VlSymsp;
    Vmultiplier* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
}

void Vmultiplier::_eval_settle(Vmultiplier__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vmultiplier::_eval_settle\n"); );
    Vmultiplier* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->_settle__TOP__3(vlSymsp);
}

void Vmultiplier::_ctor_var_reset() {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vmultiplier::_ctor_var_reset\n"); );
    // Body
    clk = VL_RAND_RESET_I(1);
    reset = VL_RAND_RESET_I(1);
    start_port = VL_RAND_RESET_I(1);
    Pd5 = VL_RAND_RESET_I(32);
    Pd6 = VL_RAND_RESET_I(32);
    done_port = VL_RAND_RESET_I(1);
    return_port = VL_RAND_RESET_I(32);
    multiplier__DOT___multiplier_i0__DOT__Controller_i__DOT___present_state = VL_RAND_RESET_I(1);
    multiplier__DOT___multiplier_i0__DOT__Controller_i__DOT___next_state = VL_RAND_RESET_I(1);
    for (int __Vi0=0; __Vi0<1; ++__Vi0) {
        __Vm_traceActivity[__Vi0] = VL_RAND_RESET_I(1);
    }
}
