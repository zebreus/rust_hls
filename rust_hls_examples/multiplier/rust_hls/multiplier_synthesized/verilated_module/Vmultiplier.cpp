// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vmultiplier.h for the primary calling header

#include "Vmultiplier.h"
#include "Vmultiplier__Syms.h"

//==========

void Vmultiplier::eval_step() {
    VL_DEBUG_IF(VL_DBG_MSGF("+++++TOP Evaluate Vmultiplier::eval\n"); );
    Vmultiplier__Syms* __restrict vlSymsp = this->__VlSymsp;  // Setup global symbol table
    Vmultiplier* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
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
            VL_FATAL_MT("/tmp/nix-shell.TXfosx/.tmpbBt7ot/test.v", 235, "",
                "Verilated model didn't converge\n"
                "- See DIDNOTCONVERGE in the Verilator manual");
        } else {
            __Vchange = _change_request(vlSymsp);
        }
    } while (VL_UNLIKELY(__Vchange));
}

void Vmultiplier::_eval_initial_loop(Vmultiplier__Syms* __restrict vlSymsp) {
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
            VL_FATAL_MT("/tmp/nix-shell.TXfosx/.tmpbBt7ot/test.v", 235, "",
                "Verilated model didn't DC converge\n"
                "- See DIDNOTCONVERGE in the Verilator manual");
        } else {
            __Vchange = _change_request(vlSymsp);
        }
    } while (VL_UNLIKELY(__Vchange));
}

VL_INLINE_OPT void Vmultiplier::_combo__TOP__2(Vmultiplier__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vmultiplier::_combo__TOP__2\n"); );
    Vmultiplier* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->return_port = (vlTOPp->Pd6 * vlTOPp->Pd5);
}

VL_INLINE_OPT void Vmultiplier::_sequent__TOP__4(Vmultiplier__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vmultiplier::_sequent__TOP__4\n"); );
    Vmultiplier* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->multiplier__DOT___multiplier_i0__DOT__Controller_i__DOT___present_state 
        = (1U & ((~ (IData)(vlTOPp->reset)) | (IData)(vlTOPp->multiplier__DOT___multiplier_i0__DOT__Controller_i__DOT___next_state)));
}

VL_INLINE_OPT void Vmultiplier::_combo__TOP__5(Vmultiplier__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vmultiplier::_combo__TOP__5\n"); );
    Vmultiplier* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->done_port = 0U;
    if (vlTOPp->multiplier__DOT___multiplier_i0__DOT__Controller_i__DOT___present_state) {
        if (vlTOPp->start_port) {
            vlTOPp->done_port = 1U;
        }
    }
}

void Vmultiplier::_eval(Vmultiplier__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vmultiplier::_eval\n"); );
    Vmultiplier* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->_combo__TOP__2(vlSymsp);
    if (((IData)(vlTOPp->clk) & (~ (IData)(vlTOPp->__Vclklast__TOP__clk)))) {
        vlTOPp->_sequent__TOP__4(vlSymsp);
    }
    vlTOPp->_combo__TOP__5(vlSymsp);
    // Final
    vlTOPp->__Vclklast__TOP__clk = vlTOPp->clk;
}

VL_INLINE_OPT QData Vmultiplier::_change_request(Vmultiplier__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vmultiplier::_change_request\n"); );
    Vmultiplier* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    return (vlTOPp->_change_request_1(vlSymsp));
}

VL_INLINE_OPT QData Vmultiplier::_change_request_1(Vmultiplier__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vmultiplier::_change_request_1\n"); );
    Vmultiplier* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    // Change detection
    QData __req = false;  // Logically a bool
    return __req;
}

#ifdef VL_DEBUG
void Vmultiplier::_eval_debug_assertions() {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vmultiplier::_eval_debug_assertions\n"); );
    // Body
    if (VL_UNLIKELY((clk & 0xfeU))) {
        Verilated::overWidthError("clk");}
    if (VL_UNLIKELY((reset & 0xfeU))) {
        Verilated::overWidthError("reset");}
    if (VL_UNLIKELY((start_port & 0xfeU))) {
        Verilated::overWidthError("start_port");}
}
#endif  // VL_DEBUG
