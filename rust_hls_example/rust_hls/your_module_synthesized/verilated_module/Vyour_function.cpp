// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vyour_function.h for the primary calling header

#include "Vyour_function.h"
#include "Vyour_function__Syms.h"

//==========

void Vyour_function::eval_step() {
    VL_DEBUG_IF(VL_DBG_MSGF("+++++TOP Evaluate Vyour_function::eval\n"); );
    Vyour_function__Syms* __restrict vlSymsp = this->__VlSymsp;  // Setup global symbol table
    Vyour_function* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
#ifdef VL_DEBUG
    // Debug assertions
    _eval_debug_assertions();
#endif  // VL_DEBUG
    // Initialize
    if (VL_UNLIKELY(!vlSymsp->__Vm_didInit)) _eval_initial_loop(vlSymsp);
    // Evaluate till stable
    int __VclockLoop = 0;
    QData __Vchange = 1;
    do {
        VL_DEBUG_IF(VL_DBG_MSGF("+ Clock loop\n"););
        vlSymsp->__Vm_activity = true;
        _eval(vlSymsp);
        if (VL_UNLIKELY(++__VclockLoop > 100)) {
            // About to fail, so enable debug to see what's not settling.
            // Note you must run make with OPT=-DVL_DEBUG for debug prints.
            int __Vsaved_debug = Verilated::debug();
            Verilated::debug(1);
            __Vchange = _change_request(vlSymsp);
            Verilated::debug(__Vsaved_debug);
            VL_FATAL_MT("/tmp/nix-shell.Lyz5Bd/.tmpkPxJ1n/test.v", 360, "",
                "Verilated model didn't converge\n"
                "- See DIDNOTCONVERGE in the Verilator manual");
        } else {
            __Vchange = _change_request(vlSymsp);
        }
    } while (VL_UNLIKELY(__Vchange));
}

void Vyour_function::_eval_initial_loop(Vyour_function__Syms* __restrict vlSymsp) {
    vlSymsp->__Vm_didInit = true;
    _eval_initial(vlSymsp);
    vlSymsp->__Vm_activity = true;
    // Evaluate till stable
    int __VclockLoop = 0;
    QData __Vchange = 1;
    do {
        _eval_settle(vlSymsp);
        _eval(vlSymsp);
        if (VL_UNLIKELY(++__VclockLoop > 100)) {
            // About to fail, so enable debug to see what's not settling.
            // Note you must run make with OPT=-DVL_DEBUG for debug prints.
            int __Vsaved_debug = Verilated::debug();
            Verilated::debug(1);
            __Vchange = _change_request(vlSymsp);
            Verilated::debug(__Vsaved_debug);
            VL_FATAL_MT("/tmp/nix-shell.Lyz5Bd/.tmpkPxJ1n/test.v", 360, "",
                "Verilated model didn't DC converge\n"
                "- See DIDNOTCONVERGE in the Verilator manual");
        } else {
            __Vchange = _change_request(vlSymsp);
        }
    } while (VL_UNLIKELY(__Vchange));
}

VL_INLINE_OPT void Vyour_function::_sequent__TOP__2(Vyour_function__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vyour_function::_sequent__TOP__2\n"); );
    Vyour_function* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->your_function__DOT___your_function_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
        = ((IData)(1U) + vlTOPp->Pd6);
    vlTOPp->your_function__DOT___your_function_i0__DOT__done_delayed_REG__DOT__reg_out1 
        = ((IData)(vlTOPp->reset) & (IData)(vlTOPp->your_function__DOT___your_function_i0__DOT__done_delayed_REG_signal_in));
    vlTOPp->your_function__DOT___your_function_i0__DOT__Controller_i__DOT___present_state 
        = ((IData)(vlTOPp->reset) ? (IData)(vlTOPp->your_function__DOT___your_function_i0__DOT__Controller_i__DOT___next_state)
            : 1U);
    vlTOPp->done_port = vlTOPp->your_function__DOT___your_function_i0__DOT__done_delayed_REG__DOT__reg_out1;
}

VL_INLINE_OPT void Vyour_function::_combo__TOP__4(Vyour_function__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vyour_function::_combo__TOP__4\n"); );
    Vyour_function* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->return_port = (vlTOPp->your_function__DOT___your_function_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                           * vlTOPp->Pd5);
    vlTOPp->your_function__DOT___your_function_i0__DOT__done_delayed_REG_signal_in = 0U;
    if ((1U == (IData)(vlTOPp->your_function__DOT___your_function_i0__DOT__Controller_i__DOT___present_state))) {
        if (vlTOPp->start_port) {
            vlTOPp->your_function__DOT___your_function_i0__DOT__done_delayed_REG_signal_in = 1U;
        }
    }
    vlTOPp->your_function__DOT___your_function_i0__DOT__Controller_i__DOT___next_state 
        = ((1U == (IData)(vlTOPp->your_function__DOT___your_function_i0__DOT__Controller_i__DOT___present_state))
            ? ((IData)(vlTOPp->start_port) ? 2U : 1U)
            : 1U);
}

void Vyour_function::_eval(Vyour_function__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vyour_function::_eval\n"); );
    Vyour_function* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    if (((IData)(vlTOPp->clk) & (~ (IData)(vlTOPp->__Vclklast__TOP__clk)))) {
        vlTOPp->_sequent__TOP__2(vlSymsp);
    }
    vlTOPp->_combo__TOP__4(vlSymsp);
    // Final
    vlTOPp->__Vclklast__TOP__clk = vlTOPp->clk;
}

VL_INLINE_OPT QData Vyour_function::_change_request(Vyour_function__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vyour_function::_change_request\n"); );
    Vyour_function* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    return (vlTOPp->_change_request_1(vlSymsp));
}

VL_INLINE_OPT QData Vyour_function::_change_request_1(Vyour_function__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vyour_function::_change_request_1\n"); );
    Vyour_function* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    // Change detection
    QData __req = false;  // Logically a bool
    return __req;
}

#ifdef VL_DEBUG
void Vyour_function::_eval_debug_assertions() {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vyour_function::_eval_debug_assertions\n"); );
    // Body
    if (VL_UNLIKELY((clk & 0xfeU))) {
        Verilated::overWidthError("clk");}
    if (VL_UNLIKELY((reset & 0xfeU))) {
        Verilated::overWidthError("reset");}
    if (VL_UNLIKELY((start_port & 0xfeU))) {
        Verilated::overWidthError("start_port");}
}
#endif  // VL_DEBUG