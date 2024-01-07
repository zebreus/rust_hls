// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design internal header
// See Vdivider.h for the primary calling header

#ifndef VERILATED_VDIVIDER___024ROOT_H_
#define VERILATED_VDIVIDER___024ROOT_H_  // guard

#include "verilated.h"


class Vdivider__Syms;

class alignas(VL_CACHE_LINE_BYTES) Vdivider___024root final : public VerilatedModule {
  public:

    // DESIGN SPECIFIC STATE
    VL_IN8(clk,0,0);
    VL_IN8(reset,0,0);
    VL_IN8(start_port,0,0);
    VL_OUT8(done_port,0,0);
    CData/*0:0*/ divider__DOT___divider_i0__DOT__done_delayed_REG_signal_in;
    CData/*0:0*/ divider__DOT___divider_i0__DOT__wrenable_reg_0;
    CData/*0:0*/ divider__DOT___divider_i0__DOT__wrenable_reg_1;
    CData/*3:0*/ divider__DOT___divider_i0__DOT__Controller_i__DOT___present_state;
    CData/*3:0*/ divider__DOT___divider_i0__DOT__Controller_i__DOT___next_state;
    CData/*0:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__done_delayed_REG_signal_in;
    CData/*0:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_10;
    CData/*0:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_11;
    CData/*0:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_15;
    CData/*0:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_2;
    CData/*0:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_3;
    CData/*0:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_5;
    CData/*0:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_6;
    CData/*0:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_7;
    CData/*4:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_bit_ior_expr_FU_0_8_8_78_i0_fu___05F_udivsi3_5825_7952;
    CData/*7:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1;
    CData/*4:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_1__DOT__reg_out1;
    CData/*0:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__done_delayed_REG__DOT__reg_out1;
    CData/*0:0*/ divider__DOT___divider_i0__DOT__done_delayed_REG__DOT__reg_out1;
    CData/*0:0*/ __VstlFirstIteration;
    CData/*0:0*/ __VicoFirstIteration;
    CData/*0:0*/ __Vtrigprevexpr___TOP__clk__0;
    CData/*0:0*/ __Vtrigprevexpr___TOP__reset__0;
    CData/*0:0*/ __VactContinue;
    SData/*10:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state;
    SData/*10:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___next_state;
    SData/*10:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_4__DOT__reg_out1;
    VL_IN(Pd5,31,0);
    VL_IN(P0,31,0);
    VL_OUT(return_port,31,0);
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_UUdata_converter_FU_63_i0_fu___05F_udivsi3_5825_7965;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_UUdata_converter_FU_66_i0_fu___05F_udivsi3_5825_7972;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_minus_expr_FU_32_32_32_93_i0_fu___05F_udivsi3_5825_7984;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_10__DOT__reg_out1;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_11__DOT__reg_out1;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_12__DOT__reg_out1;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_13__DOT__reg_out1;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_14__DOT__reg_out1;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_15__DOT__reg_out1;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_16__DOT__reg_out1;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_2__DOT__reg_out1;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_3__DOT__reg_out1;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_5__DOT__reg_out1;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_6__DOT__reg_out1;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_7__DOT__reg_out1;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_8__DOT__reg_out1;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_9__DOT__reg_out1;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1;
    IData/*31:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_1__DOT__reg_out1;
    IData/*31:0*/ __VactIterCount;
    VlUnpacked<CData/*7:0*/, 256> divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__array_7052_0__DOT__memory;
    VlTriggerVec<1> __VstlTriggered;
    VlTriggerVec<1> __VicoTriggered;
    VlTriggerVec<2> __VactTriggered;
    VlTriggerVec<2> __VnbaTriggered;

    // INTERNAL VARIABLES
    Vdivider__Syms* const vlSymsp;

    // CONSTRUCTORS
    Vdivider___024root(Vdivider__Syms* symsp, const char* v__name);
    ~Vdivider___024root();
    VL_UNCOPYABLE(Vdivider___024root);

    // INTERNAL METHODS
    void __Vconfigure(bool first);
};


#endif  // guard
