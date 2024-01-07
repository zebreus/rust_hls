// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vdivider.h for the primary calling header

#include "Vdivider__pch.h"
#include "Vdivider___024root.h"

extern const VlUnpacked<CData/*0:0*/, 64> Vdivider__ConstPool__TABLE_h0b723b94_0;
extern const VlUnpacked<CData/*0:0*/, 64> Vdivider__ConstPool__TABLE_hcd3067ba_0;
extern const VlUnpacked<CData/*0:0*/, 64> Vdivider__ConstPool__TABLE_h0a208bc8_0;
extern const VlUnpacked<CData/*3:0*/, 64> Vdivider__ConstPool__TABLE_hef3a3573_0;

VL_INLINE_OPT void Vdivider___024root___ico_sequent__TOP__0(Vdivider___024root* vlSelf) {
    if (false && vlSelf) {}  // Prevent unused
    Vdivider__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vdivider___024root___ico_sequent__TOP__0\n"); );
    // Init
    CData/*0:0*/ divider__DOT___divider_i0__DOT__selector_IN_UNBOUNDED_divider_428394_428415;
    divider__DOT___divider_i0__DOT__selector_IN_UNBOUNDED_divider_428394_428415 = 0;
    CData/*5:0*/ __Vtableidx1;
    __Vtableidx1 = 0;
    // Body
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_minus_expr_FU_32_32_32_93_i0_fu___05F_udivsi3_5825_7984 
        = (vlSelf->Pd5 - vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_16__DOT__reg_out1);
    __Vtableidx1 = (((IData)(vlSelf->start_port) << 5U) 
                    | (((IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__done_delayed_REG__DOT__reg_out1) 
                        << 4U) | (IData)(vlSelf->divider__DOT___divider_i0__DOT__Controller_i__DOT___present_state)));
    vlSelf->divider__DOT___divider_i0__DOT__done_delayed_REG_signal_in 
        = Vdivider__ConstPool__TABLE_h0b723b94_0[__Vtableidx1];
    divider__DOT___divider_i0__DOT__selector_IN_UNBOUNDED_divider_428394_428415 
        = Vdivider__ConstPool__TABLE_hcd3067ba_0[__Vtableidx1];
    vlSelf->divider__DOT___divider_i0__DOT__wrenable_reg_0 
        = Vdivider__ConstPool__TABLE_h0a208bc8_0[__Vtableidx1];
    vlSelf->divider__DOT___divider_i0__DOT__wrenable_reg_1 
        = Vdivider__ConstPool__TABLE_h0b723b94_0[__Vtableidx1];
    vlSelf->divider__DOT___divider_i0__DOT__Controller_i__DOT___next_state 
        = Vdivider__ConstPool__TABLE_hef3a3573_0[__Vtableidx1];
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_5 = 0U;
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_3 = 0U;
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_2 = 0U;
    if (((((((((1U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state)) 
               | (2U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
              | (4U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
             | (8U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
            | (0x10U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
           | (0x20U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
          | (0x40U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
         | (0x80U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state)))) {
        if ((1U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) {
            if (divider__DOT___divider_i0__DOT__selector_IN_UNBOUNDED_divider_428394_428415) {
                vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_5 = 1U;
                vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_3 = 1U;
                vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___next_state = 2U;
                vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_2 = 1U;
            } else {
                vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___next_state = 1U;
            }
        } else {
            vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___next_state 
                = ((2U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))
                    ? 4U : ((4U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))
                             ? 8U : ((8U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))
                                      ? 0x10U : ((0x10U 
                                                  == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))
                                                  ? 0x20U
                                                  : 
                                                 ((0x20U 
                                                   == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))
                                                   ? 0x40U
                                                   : 
                                                  ((0x40U 
                                                    == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))
                                                    ? 0x80U
                                                    : 0x100U))))));
        }
    } else {
        vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___next_state 
            = ((0x100U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))
                ? 0x200U : ((0x200U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))
                             ? 0x400U : 1U));
    }
}

void Vdivider___024root___eval_ico(Vdivider___024root* vlSelf) {
    if (false && vlSelf) {}  // Prevent unused
    Vdivider__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vdivider___024root___eval_ico\n"); );
    // Body
    if ((1ULL & vlSelf->__VicoTriggered.word(0U))) {
        Vdivider___024root___ico_sequent__TOP__0(vlSelf);
    }
}

void Vdivider___024root___eval_triggers__ico(Vdivider___024root* vlSelf);

bool Vdivider___024root___eval_phase__ico(Vdivider___024root* vlSelf) {
    if (false && vlSelf) {}  // Prevent unused
    Vdivider__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vdivider___024root___eval_phase__ico\n"); );
    // Init
    CData/*0:0*/ __VicoExecute;
    // Body
    Vdivider___024root___eval_triggers__ico(vlSelf);
    __VicoExecute = vlSelf->__VicoTriggered.any();
    if (__VicoExecute) {
        Vdivider___024root___eval_ico(vlSelf);
    }
    return (__VicoExecute);
}

void Vdivider___024root___eval_act(Vdivider___024root* vlSelf) {
    if (false && vlSelf) {}  // Prevent unused
    Vdivider__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vdivider___024root___eval_act\n"); );
}

VL_INLINE_OPT void Vdivider___024root___nba_sequent__TOP__0(Vdivider___024root* vlSelf) {
    if (false && vlSelf) {}  // Prevent unused
    Vdivider__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vdivider___024root___nba_sequent__TOP__0\n"); );
    // Init
    CData/*2:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_bit_and_expr_FU_8_8_8_74_i0_fu___05F_udivsi3_5825_7944;
    divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_bit_and_expr_FU_8_8_8_74_i0_fu___05F_udivsi3_5825_7944 = 0;
    CData/*6:0*/ divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_cond_expr_FU_8_8_8_8_81_i2_fu___05F_udivsi3_5825_428788;
    divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_cond_expr_FU_8_8_8_8_81_i2_fu___05F_udivsi3_5825_428788 = 0;
    CData/*4:0*/ __VdfgTmp_h5bb2e28f__0;
    __VdfgTmp_h5bb2e28f__0 = 0;
    CData/*4:0*/ __VdfgTmp_h46a667f2__0;
    __VdfgTmp_h46a667f2__0 = 0;
    CData/*1:0*/ __VdfgTmp_h8229692c__0;
    __VdfgTmp_h8229692c__0 = 0;
    CData/*5:0*/ __VdfgTmp_h37c94a1d__0;
    __VdfgTmp_h37c94a1d__0 = 0;
    CData/*2:0*/ __VdfgTmp_h1d6eb907__0;
    __VdfgTmp_h1d6eb907__0 = 0;
    CData/*2:0*/ __VdfgTmp_h4a3140c9__0;
    __VdfgTmp_h4a3140c9__0 = 0;
    // Body
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_4__DOT__reg_out1 = 0x400U;
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_1__DOT__reg_out1 
        = (0x1fU & (~ (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_bit_ior_expr_FU_0_8_8_78_i0_fu___05F_udivsi3_5825_7952)));
    if (vlSelf->reset) {
        vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state 
            = vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___next_state;
        vlSelf->divider__DOT___divider_i0__DOT__Controller_i__DOT___present_state 
            = vlSelf->divider__DOT___divider_i0__DOT__Controller_i__DOT___next_state;
    } else {
        vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state = 1U;
        vlSelf->divider__DOT___divider_i0__DOT__Controller_i__DOT___present_state = 1U;
    }
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
        = (0xffU & ((vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                     << (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_bit_ior_expr_FU_0_8_8_78_i0_fu___05F_udivsi3_5825_7952)) 
                    >> 0x17U));
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_9__DOT__reg_out1 
        = (IData)((((QData)((IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_8__DOT__reg_out1)) 
                    * (QData)((IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_7__DOT__reg_out1))) 
                   >> 0x20U));
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_16__DOT__reg_out1 
        = (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_15__DOT__reg_out1 
           * vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1);
    if (vlSelf->divider__DOT___divider_i0__DOT__wrenable_reg_1) {
        vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_1__DOT__reg_out1 
            = (((vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_minus_expr_FU_32_32_32_93_i0_fu___05F_udivsi3_5825_7984 
                 < vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1)
                 ? 0U : (((vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_minus_expr_FU_32_32_32_93_i0_fu___05F_udivsi3_5825_7984 
                           - vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1) 
                          < vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1)
                          ? 1U : 2U)) + vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_15__DOT__reg_out1);
    }
    if (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_7) {
        vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_7__DOT__reg_out1 
            = vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_UUdata_converter_FU_63_i0_fu___05F_udivsi3_5825_7965;
    }
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_8__DOT__reg_out1 
        = (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_2__DOT__reg_out1 
           * vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_6__DOT__reg_out1);
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_minus_expr_FU_32_32_32_93_i0_fu___05F_udivsi3_5825_7984 
        = (vlSelf->Pd5 - vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_16__DOT__reg_out1);
    vlSelf->return_port = vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_1__DOT__reg_out1;
    if (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_15) {
        vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_15__DOT__reg_out1 
            = (IData)((((QData)((IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_14__DOT__reg_out1)) 
                        * (QData)((IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_3__DOT__reg_out1))) 
                       >> 0x20U));
    }
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_7 = 0U;
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_15 = 0U;
    if ((1U & (~ ((((((((1U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state)) 
                        | (2U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
                       | (4U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
                      | (8U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
                     | (0x10U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
                    | (0x20U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
                   | (0x40U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
                  | (0x80U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state)))))) {
        if ((0x100U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) {
            vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_15 = 1U;
        }
    }
    if (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_6) {
        vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_6__DOT__reg_out1 
            = vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_UUdata_converter_FU_63_i0_fu___05F_udivsi3_5825_7965;
    }
    if (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_2) {
        vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_2__DOT__reg_out1 
            = (- vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1);
    }
    if (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_3) {
        vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_3__DOT__reg_out1 
            = vlSelf->Pd5;
    }
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_14__DOT__reg_out1 
        = (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_10__DOT__reg_out1 
           + vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_13__DOT__reg_out1);
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_6 = 0U;
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_UUdata_converter_FU_63_i0_fu___05F_udivsi3_5825_7965 
        = ((0x80000000U | (0x7fffffffU & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__array_7052_0__DOT__memory
                                          [(0xffU & 
                                            ((IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_4__DOT__reg_out1) 
                                             + (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1)))] 
                                          << 0x17U))) 
           >> (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_1__DOT__reg_out1));
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_13__DOT__reg_out1 
        = (IData)((((QData)((IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_12__DOT__reg_out1)) 
                    * (QData)((IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_11__DOT__reg_out1))) 
                   >> 0x20U));
    if (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_11) {
        vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_11__DOT__reg_out1 
            = vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_UUdata_converter_FU_66_i0_fu___05F_udivsi3_5825_7972;
    }
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_12__DOT__reg_out1 
        = (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_10__DOT__reg_out1 
           * vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_5__DOT__reg_out1);
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_11 = 0U;
    if (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_10) {
        vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_10__DOT__reg_out1 
            = vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_UUdata_converter_FU_66_i0_fu___05F_udivsi3_5825_7972;
    }
    if (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_5) {
        vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_5__DOT__reg_out1 
            = (- vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1);
    }
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_10 = 0U;
    if (((((((((1U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state)) 
               | (2U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
              | (4U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
             | (8U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
            | (0x10U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
           | (0x20U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
          | (0x40U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
         | (0x80U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state)))) {
        if ((1U != (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) {
            if ((2U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) {
                vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_7 = 1U;
                vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_6 = 1U;
            }
            if ((2U != (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) {
                if ((4U != (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) {
                    if ((8U != (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) {
                        if ((0x10U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) {
                            vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_11 = 1U;
                            vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_10 = 1U;
                        }
                    }
                }
            }
        }
    }
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_UUdata_converter_FU_66_i0_fu___05F_udivsi3_5825_7972 
        = (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_6__DOT__reg_out1 
           + vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__reg_9__DOT__reg_out1);
    if (vlSelf->divider__DOT___divider_i0__DOT__wrenable_reg_0) {
        vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
            = ((0U == vlSelf->P0) ? 2U : vlSelf->P0);
    }
    __VdfgTmp_h1d6eb907__0 = ((4U & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                     >> 0x18U)) | (
                                                   (2U 
                                                    & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                       >> 9U)) 
                                                   | (0x10000U 
                                                      > vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1)));
    __VdfgTmp_h4a3140c9__0 = ((4U & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                     >> 0x1cU)) | (
                                                   (2U 
                                                    & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                       >> 0xdU)) 
                                                   | (0x10000U 
                                                      > vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1)));
    __VdfgTmp_h37c94a1d__0 = ((0x20U & ((IData)((0x3fffffffffffffULL 
                                                 & (0x22052700000000ULL 
                                                    >> 
                                                    ((0x20U 
                                                      & ((IData)(
                                                                 (0x7fffffffffffffULL 
                                                                  & (0x50035300000000ULL 
                                                                     >> 
                                                                     ((0x20U 
                                                                       & ((0x220527U 
                                                                           >> 
                                                                           ((0x10U 
                                                                             & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                                >> 0x17U)) 
                                                                            | ((8U 
                                                                                & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                                >> 8U)) 
                                                                               | (IData)(__VdfgTmp_h1d6eb907__0)))) 
                                                                          << 5U)) 
                                                                      | ((0x10U 
                                                                          & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                             >> 0x15U)) 
                                                                         | ((8U 
                                                                             & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                                >> 6U)) 
                                                                            | (((0x10000U 
                                                                                > vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1) 
                                                                                << 2U) 
                                                                               | ((2U 
                                                                                & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                                >> 0x17U)) 
                                                                                | (1U 
                                                                                & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                                >> 8U)))))))))) 
                                                         << 5U)) 
                                                     | ((0x10U 
                                                         & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                            >> 0x1bU)) 
                                                        | ((8U 
                                                            & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                               >> 0xcU)) 
                                                           | (IData)(__VdfgTmp_h4a3140c9__0))))))) 
                                        << 5U)) | (
                                                   (0x10U 
                                                    & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                       >> 0x19U)) 
                                                   | ((8U 
                                                       & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                          >> 0xaU)) 
                                                      | ((4U 
                                                          & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                             >> 0x1aU)) 
                                                         | ((2U 
                                                             & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                >> 0xbU)) 
                                                            | (0x10000U 
                                                               > vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1))))));
    __VdfgTmp_h8229692c__0 = ((2U & ((IData)((0x3fffffffffffffULL 
                                              & (0x22052700000000ULL 
                                                 >> (IData)(__VdfgTmp_h37c94a1d__0)))) 
                                     << 1U)) | (0x10000U 
                                                > vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1));
    divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_cond_expr_FU_8_8_8_8_81_i2_fu___05F_udivsi3_5825_428788 
        = (0x7fU & ((1U & (0xeU >> (IData)(__VdfgTmp_h8229692c__0)))
                     ? ((1U & (2U >> (IData)(__VdfgTmp_h8229692c__0)))
                         ? (0x7fU & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                     >> 9U)) : ((1U 
                                                 & (8U 
                                                    >> (IData)(__VdfgTmp_h8229692c__0)))
                                                 ? 
                                                (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                 >> 1U)
                                                 : 
                                                (0x7fU 
                                                 & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                    >> 0x11U))))
                     : (0xffU & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                 >> 0x19U))));
    __VdfgTmp_h5bb2e28f__0 = ((0x10U & ((IData)((0x1ffffffffffffULL 
                                                 & (0x1ffff00010000ULL 
                                                    >> 
                                                    ((0x20U 
                                                      & ((1U 
                                                          >> 
                                                          ((8U 
                                                            & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                               >> 0x14U)) 
                                                           | ((4U 
                                                               & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                  >> 0x14U)) 
                                                              | ((2U 
                                                                  & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                     >> 0x14U)) 
                                                                 | (1U 
                                                                    & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                       >> 0x14U)))))) 
                                                         << 5U)) 
                                                     | ((0x10U 
                                                         & ((8U 
                                                             >> (IData)(__VdfgTmp_h8229692c__0)) 
                                                            << 4U)) 
                                                        | ((8U 
                                                            & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                               >> 4U)) 
                                                           | ((4U 
                                                               & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                  >> 4U)) 
                                                              | ((2U 
                                                                  & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                     >> 4U)) 
                                                                 | (1U 
                                                                    & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                       >> 4U)))))))))) 
                                        << 4U)) | (
                                                   (8U 
                                                    & ((1U 
                                                        >> 
                                                        ((8U 
                                                          & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                             >> 0xcU)) 
                                                         | ((4U 
                                                             & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                >> 0xcU)) 
                                                            | ((2U 
                                                                & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                   >> 0xcU)) 
                                                               | (1U 
                                                                  & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                     >> 0xcU)))))) 
                                                       << 3U)) 
                                                   | ((4U 
                                                       & ((1U 
                                                           >> (IData)(__VdfgTmp_h8229692c__0)) 
                                                          << 2U)) 
                                                      | ((2U 
                                                          & ((2U 
                                                              >> (IData)(__VdfgTmp_h8229692c__0)) 
                                                             << 1U)) 
                                                         | (0x10000000U 
                                                            > vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1)))));
    __VdfgTmp_h46a667f2__0 = ((0x10U & ((IData)((0xff55aa00d8d8d8d8ULL 
                                                 >> 
                                                 ((0x20U 
                                                   & ((IData)(
                                                              (0x3fffffffffffffULL 
                                                               & (0x22052700000000ULL 
                                                                  >> (IData)(__VdfgTmp_h37c94a1d__0)))) 
                                                      << 5U)) 
                                                  | ((0x10U 
                                                      & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                         >> 0x13U)) 
                                                     | ((8U 
                                                         & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                            >> 4U)) 
                                                        | ((4U 
                                                            & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                               >> 0x1dU)) 
                                                           | ((2U 
                                                               & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                  >> 0xeU)) 
                                                              | (0x10000U 
                                                                 > vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1)))))))) 
                                        << 4U)) | (
                                                   (8U 
                                                    & ((IData)(
                                                               (0xff55aa00d8d8d8d8ULL 
                                                                >> 
                                                                ((0x20U 
                                                                  & ((IData)(
                                                                             (0x3fffffffffffffULL 
                                                                              & (0x22052700000000ULL 
                                                                                >> (IData)(__VdfgTmp_h37c94a1d__0)))) 
                                                                     << 5U)) 
                                                                 | ((0x10U 
                                                                     & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                        >> 0xfU)) 
                                                                    | ((8U 
                                                                        & vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1) 
                                                                       | ((4U 
                                                                           & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                              >> 0x19U)) 
                                                                          | ((2U 
                                                                              & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                                >> 0xaU)) 
                                                                             | (0x10000U 
                                                                                > vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1)))))))) 
                                                       << 3U)) 
                                                   | ((4U 
                                                       & ((IData)(
                                                                  (0xff55aa00d8d8d8d8ULL 
                                                                   >> 
                                                                   ((0x20U 
                                                                     & ((IData)(
                                                                                (0x3fffffffffffffULL 
                                                                                & (0x22052700000000ULL 
                                                                                >> (IData)(__VdfgTmp_h37c94a1d__0)))) 
                                                                        << 5U)) 
                                                                    | ((0x10U 
                                                                        & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                           >> 0x12U)) 
                                                                       | ((8U 
                                                                           & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                              >> 3U)) 
                                                                          | (IData)(__VdfgTmp_h4a3140c9__0)))))) 
                                                          << 2U)) 
                                                      | ((2U 
                                                          & ((IData)(
                                                                     (0xff55aa00d8d8d8d8ULL 
                                                                      >> 
                                                                      ((0x20U 
                                                                        & ((IData)(
                                                                                (0x3fffffffffffffULL 
                                                                                & (0x22052700000000ULL 
                                                                                >> (IData)(__VdfgTmp_h37c94a1d__0)))) 
                                                                           << 5U)) 
                                                                       | ((0x10U 
                                                                           & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                              >> 0xeU)) 
                                                                          | ((8U 
                                                                              & (vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1 
                                                                                << 1U)) 
                                                                             | (IData)(__VdfgTmp_h1d6eb907__0)))))) 
                                                             << 1U)) 
                                                         | (1U 
                                                            & (0xafa3aca0U 
                                                               >> (IData)(__VdfgTmp_h5bb2e28f__0)))))));
    divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_bit_and_expr_FU_8_8_8_74_i0_fu___05F_udivsi3_5825_7944 
        = (((1U & (0x220527U >> (IData)(__VdfgTmp_h46a667f2__0)))
             ? 1U : 4U) & ((1U & (0xafa3aca0U >> (IData)(__VdfgTmp_h5bb2e28f__0)))
                            ? (IData)(divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_cond_expr_FU_8_8_8_8_81_i2_fu___05F_udivsi3_5825_428788)
                            : (7U & ((IData)(divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_cond_expr_FU_8_8_8_8_81_i2_fu___05F_udivsi3_5825_428788) 
                                     >> 4U))));
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_bit_ior_expr_FU_0_8_8_78_i0_fu___05F_udivsi3_5825_7952 
        = (0x1fU & (((((8U & ((1U >> ((2U & ((1U >> (IData)(__VdfgTmp_h8229692c__0)) 
                                             << 1U)) 
                                      | (1U & (2U >> (IData)(__VdfgTmp_h8229692c__0))))) 
                              << 3U)) | ((0x10000U 
                                          > vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__reg_0__DOT__reg_out1) 
                                         << 4U)) | 
                      (4U & ((0xafa3aca0U >> (IData)(__VdfgTmp_h5bb2e28f__0)) 
                             << 2U))) | (2U & ((0x220527U 
                                                >> (IData)(__VdfgTmp_h46a667f2__0)) 
                                               << 1U))) 
                    | (1U & (1U >> ((2U & ((IData)(divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_bit_and_expr_FU_8_8_8_74_i0_fu___05F_udivsi3_5825_7944) 
                                           >> 1U)) 
                                    | (1U & (IData)(divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Datapath_i__DOT__out_ui_bit_and_expr_FU_8_8_8_74_i0_fu___05F_udivsi3_5825_7944)))))));
}

VL_INLINE_OPT void Vdivider___024root___nba_sequent__TOP__1(Vdivider___024root* vlSelf) {
    if (false && vlSelf) {}  // Prevent unused
    Vdivider__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vdivider___024root___nba_sequent__TOP__1\n"); );
    // Body
    vlSelf->divider__DOT___divider_i0__DOT__done_delayed_REG__DOT__reg_out1 
        = ((IData)(vlSelf->reset) & (IData)(vlSelf->divider__DOT___divider_i0__DOT__done_delayed_REG_signal_in));
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__done_delayed_REG__DOT__reg_out1 
        = ((IData)(vlSelf->reset) & (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__done_delayed_REG_signal_in));
    vlSelf->done_port = vlSelf->divider__DOT___divider_i0__DOT__done_delayed_REG__DOT__reg_out1;
}

VL_INLINE_OPT void Vdivider___024root___nba_sequent__TOP__2(Vdivider___024root* vlSelf) {
    if (false && vlSelf) {}  // Prevent unused
    Vdivider__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vdivider___024root___nba_sequent__TOP__2\n"); );
    // Body
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__done_delayed_REG_signal_in = 0U;
    if ((1U & (~ ((((((((1U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state)) 
                        | (2U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
                       | (4U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
                      | (8U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
                     | (0x10U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
                    | (0x20U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
                   | (0x40U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
                  | (0x80U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state)))))) {
        if ((0x100U != (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) {
            if ((0x200U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) {
                vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__done_delayed_REG_signal_in = 1U;
            }
        }
    }
}

VL_INLINE_OPT void Vdivider___024root___nba_comb__TOP__0(Vdivider___024root* vlSelf) {
    if (false && vlSelf) {}  // Prevent unused
    Vdivider__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vdivider___024root___nba_comb__TOP__0\n"); );
    // Init
    CData/*0:0*/ divider__DOT___divider_i0__DOT__selector_IN_UNBOUNDED_divider_428394_428415;
    divider__DOT___divider_i0__DOT__selector_IN_UNBOUNDED_divider_428394_428415 = 0;
    CData/*5:0*/ __Vtableidx1;
    __Vtableidx1 = 0;
    // Body
    __Vtableidx1 = (((IData)(vlSelf->start_port) << 5U) 
                    | (((IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__done_delayed_REG__DOT__reg_out1) 
                        << 4U) | (IData)(vlSelf->divider__DOT___divider_i0__DOT__Controller_i__DOT___present_state)));
    vlSelf->divider__DOT___divider_i0__DOT__done_delayed_REG_signal_in 
        = Vdivider__ConstPool__TABLE_h0b723b94_0[__Vtableidx1];
    divider__DOT___divider_i0__DOT__selector_IN_UNBOUNDED_divider_428394_428415 
        = Vdivider__ConstPool__TABLE_hcd3067ba_0[__Vtableidx1];
    vlSelf->divider__DOT___divider_i0__DOT__wrenable_reg_0 
        = Vdivider__ConstPool__TABLE_h0a208bc8_0[__Vtableidx1];
    vlSelf->divider__DOT___divider_i0__DOT__wrenable_reg_1 
        = Vdivider__ConstPool__TABLE_h0b723b94_0[__Vtableidx1];
    vlSelf->divider__DOT___divider_i0__DOT__Controller_i__DOT___next_state 
        = Vdivider__ConstPool__TABLE_hef3a3573_0[__Vtableidx1];
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_5 = 0U;
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_3 = 0U;
    vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_2 = 0U;
    if (((((((((1U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state)) 
               | (2U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
              | (4U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
             | (8U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
            | (0x10U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
           | (0x20U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
          | (0x40U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) 
         | (0x80U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state)))) {
        if ((1U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))) {
            if (divider__DOT___divider_i0__DOT__selector_IN_UNBOUNDED_divider_428394_428415) {
                vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_5 = 1U;
                vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_3 = 1U;
                vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___next_state = 2U;
                vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__wrenable_reg_2 = 1U;
            } else {
                vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___next_state = 1U;
            }
        } else {
            vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___next_state 
                = ((2U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))
                    ? 4U : ((4U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))
                             ? 8U : ((8U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))
                                      ? 0x10U : ((0x10U 
                                                  == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))
                                                  ? 0x20U
                                                  : 
                                                 ((0x20U 
                                                   == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))
                                                   ? 0x40U
                                                   : 
                                                  ((0x40U 
                                                    == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))
                                                    ? 0x80U
                                                    : 0x100U))))));
        }
    } else {
        vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___next_state 
            = ((0x100U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))
                ? 0x200U : ((0x200U == (IData)(vlSelf->divider__DOT___divider_i0__DOT__Datapath_i__DOT__fu_divider_428394_428415__DOT__Controller_i__DOT___present_state))
                             ? 0x400U : 1U));
    }
}

void Vdivider___024root___eval_nba(Vdivider___024root* vlSelf) {
    if (false && vlSelf) {}  // Prevent unused
    Vdivider__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vdivider___024root___eval_nba\n"); );
    // Body
    if ((1ULL & vlSelf->__VnbaTriggered.word(0U))) {
        Vdivider___024root___nba_sequent__TOP__0(vlSelf);
    }
    if ((2ULL & vlSelf->__VnbaTriggered.word(0U))) {
        Vdivider___024root___nba_sequent__TOP__1(vlSelf);
    }
    if ((1ULL & vlSelf->__VnbaTriggered.word(0U))) {
        Vdivider___024root___nba_sequent__TOP__2(vlSelf);
    }
    if ((3ULL & vlSelf->__VnbaTriggered.word(0U))) {
        Vdivider___024root___nba_comb__TOP__0(vlSelf);
    }
}

void Vdivider___024root___eval_triggers__act(Vdivider___024root* vlSelf);

bool Vdivider___024root___eval_phase__act(Vdivider___024root* vlSelf) {
    if (false && vlSelf) {}  // Prevent unused
    Vdivider__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vdivider___024root___eval_phase__act\n"); );
    // Init
    VlTriggerVec<2> __VpreTriggered;
    CData/*0:0*/ __VactExecute;
    // Body
    Vdivider___024root___eval_triggers__act(vlSelf);
    __VactExecute = vlSelf->__VactTriggered.any();
    if (__VactExecute) {
        __VpreTriggered.andNot(vlSelf->__VactTriggered, vlSelf->__VnbaTriggered);
        vlSelf->__VnbaTriggered.thisOr(vlSelf->__VactTriggered);
        Vdivider___024root___eval_act(vlSelf);
    }
    return (__VactExecute);
}

bool Vdivider___024root___eval_phase__nba(Vdivider___024root* vlSelf) {
    if (false && vlSelf) {}  // Prevent unused
    Vdivider__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vdivider___024root___eval_phase__nba\n"); );
    // Init
    CData/*0:0*/ __VnbaExecute;
    // Body
    __VnbaExecute = vlSelf->__VnbaTriggered.any();
    if (__VnbaExecute) {
        Vdivider___024root___eval_nba(vlSelf);
        vlSelf->__VnbaTriggered.clear();
    }
    return (__VnbaExecute);
}

#ifdef VL_DEBUG
VL_ATTR_COLD void Vdivider___024root___dump_triggers__ico(Vdivider___024root* vlSelf);
#endif  // VL_DEBUG
#ifdef VL_DEBUG
VL_ATTR_COLD void Vdivider___024root___dump_triggers__nba(Vdivider___024root* vlSelf);
#endif  // VL_DEBUG
#ifdef VL_DEBUG
VL_ATTR_COLD void Vdivider___024root___dump_triggers__act(Vdivider___024root* vlSelf);
#endif  // VL_DEBUG

void Vdivider___024root___eval(Vdivider___024root* vlSelf) {
    if (false && vlSelf) {}  // Prevent unused
    Vdivider__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vdivider___024root___eval\n"); );
    // Init
    IData/*31:0*/ __VicoIterCount;
    CData/*0:0*/ __VicoContinue;
    IData/*31:0*/ __VnbaIterCount;
    CData/*0:0*/ __VnbaContinue;
    // Body
    __VicoIterCount = 0U;
    vlSelf->__VicoFirstIteration = 1U;
    __VicoContinue = 1U;
    while (__VicoContinue) {
        if (VL_UNLIKELY((0x64U < __VicoIterCount))) {
#ifdef VL_DEBUG
            Vdivider___024root___dump_triggers__ico(vlSelf);
#endif
            VL_FATAL_MT("/tmp/nix-shell.dClVll/.tmpN4KzVh/test.v", 2847, "", "Input combinational region did not converge.");
        }
        __VicoIterCount = ((IData)(1U) + __VicoIterCount);
        __VicoContinue = 0U;
        if (Vdivider___024root___eval_phase__ico(vlSelf)) {
            __VicoContinue = 1U;
        }
        vlSelf->__VicoFirstIteration = 0U;
    }
    __VnbaIterCount = 0U;
    __VnbaContinue = 1U;
    while (__VnbaContinue) {
        if (VL_UNLIKELY((0x64U < __VnbaIterCount))) {
#ifdef VL_DEBUG
            Vdivider___024root___dump_triggers__nba(vlSelf);
#endif
            VL_FATAL_MT("/tmp/nix-shell.dClVll/.tmpN4KzVh/test.v", 2847, "", "NBA region did not converge.");
        }
        __VnbaIterCount = ((IData)(1U) + __VnbaIterCount);
        __VnbaContinue = 0U;
        vlSelf->__VactIterCount = 0U;
        vlSelf->__VactContinue = 1U;
        while (vlSelf->__VactContinue) {
            if (VL_UNLIKELY((0x64U < vlSelf->__VactIterCount))) {
#ifdef VL_DEBUG
                Vdivider___024root___dump_triggers__act(vlSelf);
#endif
                VL_FATAL_MT("/tmp/nix-shell.dClVll/.tmpN4KzVh/test.v", 2847, "", "Active region did not converge.");
            }
            vlSelf->__VactIterCount = ((IData)(1U) 
                                       + vlSelf->__VactIterCount);
            vlSelf->__VactContinue = 0U;
            if (Vdivider___024root___eval_phase__act(vlSelf)) {
                vlSelf->__VactContinue = 1U;
            }
        }
        if (Vdivider___024root___eval_phase__nba(vlSelf)) {
            __VnbaContinue = 1U;
        }
    }
}

#ifdef VL_DEBUG
void Vdivider___024root___eval_debug_assertions(Vdivider___024root* vlSelf) {
    if (false && vlSelf) {}  // Prevent unused
    Vdivider__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vdivider___024root___eval_debug_assertions\n"); );
    // Body
    if (VL_UNLIKELY((vlSelf->clk & 0xfeU))) {
        Verilated::overWidthError("clk");}
    if (VL_UNLIKELY((vlSelf->reset & 0xfeU))) {
        Verilated::overWidthError("reset");}
    if (VL_UNLIKELY((vlSelf->start_port & 0xfeU))) {
        Verilated::overWidthError("start_port");}
}
#endif  // VL_DEBUG
