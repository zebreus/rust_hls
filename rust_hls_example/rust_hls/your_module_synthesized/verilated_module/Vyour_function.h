// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Primary design header
//
// This header should be included by all source files instantiating the design.
// The class here is then constructed to instantiate the design.
// See the Verilator manual for examples.

#ifndef _VYOUR_FUNCTION_H_
#define _VYOUR_FUNCTION_H_  // guard

#include "verilated_heavy.h"

//==========

class Vyour_function__Syms;
class Vyour_function_VerilatedVcd;


//----------

VL_MODULE(Vyour_function) {
  public:
    
    // PORTS
    // The application code writes and reads these signals to
    // propagate new values into/out from the Verilated model.
    VL_IN8(clk,0,0);
    VL_IN8(reset,0,0);
    VL_IN8(start_port,0,0);
    VL_OUT8(done_port,0,0);
    VL_IN(Pd5,31,0);
    VL_IN(Pd6,31,0);
    VL_OUT(return_port,31,0);
    
    // LOCAL SIGNALS
    // Internals; generally not touched by application code
    CData/*0:0*/ your_function__DOT___your_function_i0__DOT__done_delayed_REG_signal_in;
    CData/*1:0*/ your_function__DOT___your_function_i0__DOT__Controller_i__DOT___present_state;
    CData/*1:0*/ your_function__DOT___your_function_i0__DOT__Controller_i__DOT___next_state;
    CData/*0:0*/ your_function__DOT___your_function_i0__DOT__done_delayed_REG__DOT__reg_out1;
    IData/*31:0*/ your_function__DOT___your_function_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1;
    
    // LOCAL VARIABLES
    // Internals; generally not touched by application code
    CData/*0:0*/ __Vclklast__TOP__clk;
    CData/*0:0*/ __Vm_traceActivity[1];
    
    // INTERNAL VARIABLES
    // Internals; generally not touched by application code
    Vyour_function__Syms* __VlSymsp;  // Symbol table
    
    // CONSTRUCTORS
  private:
    VL_UNCOPYABLE(Vyour_function);  ///< Copying not allowed
  public:
    /// Construct the model; called by application code
    /// The special name  may be used to make a wrapper with a
    /// single model invisible with respect to DPI scope names.
    Vyour_function(const char* name = "TOP");
    /// Destroy the model; called (often implicitly) by application code
    ~Vyour_function();
    /// Trace signals in the model; called by application code
    void trace(VerilatedVcdC* tfp, int levels, int options = 0);
    
    // API METHODS
    /// Evaluate the model.  Application must call when inputs change.
    void eval() { eval_step(); }
    /// Evaluate when calling multiple units/models per time step.
    void eval_step();
    /// Evaluate at end of a timestep for tracing, when using eval_step().
    /// Application must call after all eval() and before time changes.
    void eval_end_step() {}
    /// Simulation complete, run final blocks.  Application must call on completion.
    void final();
    
    // INTERNAL METHODS
    static void _eval_initial_loop(Vyour_function__Syms* __restrict vlSymsp);
    void __Vconfigure(Vyour_function__Syms* symsp, bool first);
  private:
    static QData _change_request(Vyour_function__Syms* __restrict vlSymsp);
    static QData _change_request_1(Vyour_function__Syms* __restrict vlSymsp);
  public:
    static void _combo__TOP__4(Vyour_function__Syms* __restrict vlSymsp);
  private:
    void _ctor_var_reset() VL_ATTR_COLD;
  public:
    static void _eval(Vyour_function__Syms* __restrict vlSymsp);
  private:
#ifdef VL_DEBUG
    void _eval_debug_assertions();
#endif  // VL_DEBUG
  public:
    static void _eval_initial(Vyour_function__Syms* __restrict vlSymsp) VL_ATTR_COLD;
    static void _eval_settle(Vyour_function__Syms* __restrict vlSymsp) VL_ATTR_COLD;
    static void _initial__TOP__1(Vyour_function__Syms* __restrict vlSymsp) VL_ATTR_COLD;
    static void _sequent__TOP__2(Vyour_function__Syms* __restrict vlSymsp);
    static void _settle__TOP__3(Vyour_function__Syms* __restrict vlSymsp) VL_ATTR_COLD;
  private:
    static void traceChgSub0(void* userp, VerilatedVcd* tracep);
    static void traceChgTop0(void* userp, VerilatedVcd* tracep);
    static void traceCleanup(void* userp, VerilatedVcd* /*unused*/);
    static void traceFullSub0(void* userp, VerilatedVcd* tracep) VL_ATTR_COLD;
    static void traceFullTop0(void* userp, VerilatedVcd* tracep) VL_ATTR_COLD;
    static void traceInitSub0(void* userp, VerilatedVcd* tracep) VL_ATTR_COLD;
    static void traceInitTop(void* userp, VerilatedVcd* tracep) VL_ATTR_COLD;
    void traceRegister(VerilatedVcd* tracep) VL_ATTR_COLD;
    static void traceInit(void* userp, VerilatedVcd* tracep, uint32_t code) VL_ATTR_COLD;
} VL_ATTR_ALIGNED(VL_CACHE_LINE_BYTES);

//----------


#endif  // guard
