// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Tracing implementation internals
#include "verilated_vcd_c.h"
#include "Vmultiplier__Syms.h"


void Vmultiplier___024root__trace_chg_0_sub_0(Vmultiplier___024root* vlSelf, VerilatedVcd::Buffer* bufp);

void Vmultiplier___024root__trace_chg_0(void* voidSelf, VerilatedVcd::Buffer* bufp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vmultiplier___024root__trace_chg_0\n"); );
    // Init
    Vmultiplier___024root* const __restrict vlSelf VL_ATTR_UNUSED = static_cast<Vmultiplier___024root*>(voidSelf);
    Vmultiplier__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    if (VL_UNLIKELY(!vlSymsp->__Vm_activity)) return;
    // Body
    Vmultiplier___024root__trace_chg_0_sub_0((&vlSymsp->TOP), bufp);
}

void Vmultiplier___024root__trace_chg_0_sub_0(Vmultiplier___024root* vlSelf, VerilatedVcd::Buffer* bufp) {
    if (false && vlSelf) {}  // Prevent unused
    Vmultiplier__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vmultiplier___024root__trace_chg_0_sub_0\n"); );
    // Init
    uint32_t* const oldp VL_ATTR_UNUSED = bufp->oldp(vlSymsp->__Vm_baseCode + 1);
    // Body
    bufp->chgBit(oldp+0,(vlSelf->clk));
    bufp->chgBit(oldp+1,(vlSelf->reset));
    bufp->chgBit(oldp+2,(vlSelf->start_port));
    bufp->chgIData(oldp+3,(vlSelf->Pd5),32);
    bufp->chgIData(oldp+4,(vlSelf->Pd6),32);
    bufp->chgBit(oldp+5,(vlSelf->done_port));
    bufp->chgIData(oldp+6,(vlSelf->return_port),32);
}

void Vmultiplier___024root__trace_cleanup(void* voidSelf, VerilatedVcd* /*unused*/) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vmultiplier___024root__trace_cleanup\n"); );
    // Init
    Vmultiplier___024root* const __restrict vlSelf VL_ATTR_UNUSED = static_cast<Vmultiplier___024root*>(voidSelf);
    Vmultiplier__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VlUnpacked<CData/*0:0*/, 1> __Vm_traceActivity;
    for (int __Vi0 = 0; __Vi0 < 1; ++__Vi0) {
        __Vm_traceActivity[__Vi0] = 0;
    }
    // Body
    vlSymsp->__Vm_activity = false;
    __Vm_traceActivity[0U] = 0U;
}
