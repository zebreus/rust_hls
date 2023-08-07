// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vother_function.h for the primary calling header

#include "Vother_function.h"
#include "Vother_function__Syms.h"

//==========

Vother_function::Vother_function(const char* __VCname)
    : VerilatedModule(__VCname)
 {
    Vother_function__Syms* __restrict vlSymsp = __VlSymsp = new Vother_function__Syms(this, name());
    Vother_function* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Reset internal values
    
    // Reset structure values
    _ctor_var_reset();
}

void Vother_function::__Vconfigure(Vother_function__Syms* vlSymsp, bool first) {
    if (false && first) {}  // Prevent unused
    this->__VlSymsp = vlSymsp;
    if (false && this->__VlSymsp) {}  // Prevent unused
    Verilated::timeunit(-9);
    Verilated::timeprecision(-12);
}

Vother_function::~Vother_function() {
    VL_DO_CLEAR(delete __VlSymsp, __VlSymsp = nullptr);
}

void Vother_function::_initial__TOP__1(Vother_function__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vother_function::_initial__TOP__1\n"); );
    Vother_function* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->other_function__DOT___other_function_i0__DOT__done_delayed_REG__DOT__reg_out1 = 0U;
    vlTOPp->other_function__DOT___other_function_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 = 0U;
    vlTOPp->other_function__DOT___other_function_i0__DOT__Controller_i__DOT___present_state = 1U;
}

void Vother_function::_settle__TOP__3(Vother_function__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vother_function::_settle__TOP__3\n"); );
    Vother_function* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->done_port = vlTOPp->other_function__DOT___other_function_i0__DOT__done_delayed_REG__DOT__reg_out1;
    vlTOPp->return_port = (vlTOPp->other_function__DOT___other_function_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                           * vlTOPp->Pd5);
    vlTOPp->other_function__DOT___other_function_i0__DOT__done_delayed_REG_signal_in = 0U;
    if ((1U == (IData)(vlTOPp->other_function__DOT___other_function_i0__DOT__Controller_i__DOT___present_state))) {
        if (vlTOPp->start_port) {
            vlTOPp->other_function__DOT___other_function_i0__DOT__done_delayed_REG_signal_in = 1U;
        }
    }
    vlTOPp->other_function__DOT___other_function_i0__DOT__Controller_i__DOT___next_state 
        = ((1U == (IData)(vlTOPp->other_function__DOT___other_function_i0__DOT__Controller_i__DOT___present_state))
            ? ((IData)(vlTOPp->start_port) ? 2U : 1U)
            : 1U);
}

void Vother_function::_eval_initial(Vother_function__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vother_function::_eval_initial\n"); );
    Vother_function* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->_initial__TOP__1(vlSymsp);
    vlTOPp->__Vclklast__TOP__clk = vlTOPp->clk;
}

void Vother_function::final() {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vother_function::final\n"); );
    // Variables
    Vother_function__Syms* __restrict vlSymsp = this->__VlSymsp;
    Vother_function* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
}

void Vother_function::_eval_settle(Vother_function__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vother_function::_eval_settle\n"); );
    Vother_function* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->_settle__TOP__3(vlSymsp);
}

void Vother_function::_ctor_var_reset() {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vother_function::_ctor_var_reset\n"); );
    // Body
    clk = VL_RAND_RESET_I(1);
    reset = VL_RAND_RESET_I(1);
    start_port = VL_RAND_RESET_I(1);
    Pd5 = VL_RAND_RESET_I(32);
    Pd6 = VL_RAND_RESET_I(32);
    done_port = VL_RAND_RESET_I(1);
    return_port = VL_RAND_RESET_I(32);
    other_function__DOT___other_function_i0__DOT__done_delayed_REG_signal_in = VL_RAND_RESET_I(1);
    other_function__DOT___other_function_i0__DOT__Controller_i__DOT___present_state = VL_RAND_RESET_I(2);
    other_function__DOT___other_function_i0__DOT__Controller_i__DOT___next_state = VL_RAND_RESET_I(2);
    other_function__DOT___other_function_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 = VL_RAND_RESET_I(32);
    other_function__DOT___other_function_i0__DOT__done_delayed_REG__DOT__reg_out1 = VL_RAND_RESET_I(1);
    for (int __Vi0=0; __Vi0<1; ++__Vi0) {
        __Vm_traceActivity[__Vi0] = VL_RAND_RESET_I(1);
    }
}
