// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Model implementation (design independent parts)

#include "Vsubtractor__pch.h"
#include "verilated_vcd_c.h"

//============================================================
// Constructors

Vsubtractor::Vsubtractor(VerilatedContext* _vcontextp__, const char* _vcname__)
    : VerilatedModel{*_vcontextp__}
    , vlSymsp{new Vsubtractor__Syms(contextp(), _vcname__, this)}
    , clk{vlSymsp->TOP.clk}
    , reset{vlSymsp->TOP.reset}
    , start_port{vlSymsp->TOP.start_port}
    , done_port{vlSymsp->TOP.done_port}
    , Pd5{vlSymsp->TOP.Pd5}
    , Pd6{vlSymsp->TOP.Pd6}
    , return_port{vlSymsp->TOP.return_port}
    , rootp{&(vlSymsp->TOP)}
{
    // Register model with the context
    contextp()->addModel(this);
}

Vsubtractor::Vsubtractor(const char* _vcname__)
    : Vsubtractor(Verilated::threadContextp(), _vcname__)
{
}

//============================================================
// Destructor

Vsubtractor::~Vsubtractor() {
    delete vlSymsp;
}

//============================================================
// Evaluation function

#ifdef VL_DEBUG
void Vsubtractor___024root___eval_debug_assertions(Vsubtractor___024root* vlSelf);
#endif  // VL_DEBUG
void Vsubtractor___024root___eval_static(Vsubtractor___024root* vlSelf);
void Vsubtractor___024root___eval_initial(Vsubtractor___024root* vlSelf);
void Vsubtractor___024root___eval_settle(Vsubtractor___024root* vlSelf);
void Vsubtractor___024root___eval(Vsubtractor___024root* vlSelf);

void Vsubtractor::eval_step() {
    VL_DEBUG_IF(VL_DBG_MSGF("+++++TOP Evaluate Vsubtractor::eval_step\n"); );
#ifdef VL_DEBUG
    // Debug assertions
    Vsubtractor___024root___eval_debug_assertions(&(vlSymsp->TOP));
#endif  // VL_DEBUG
    vlSymsp->__Vm_activity = true;
    vlSymsp->__Vm_deleter.deleteAll();
    if (VL_UNLIKELY(!vlSymsp->__Vm_didInit)) {
        vlSymsp->__Vm_didInit = true;
        VL_DEBUG_IF(VL_DBG_MSGF("+ Initial\n"););
        Vsubtractor___024root___eval_static(&(vlSymsp->TOP));
        Vsubtractor___024root___eval_initial(&(vlSymsp->TOP));
        Vsubtractor___024root___eval_settle(&(vlSymsp->TOP));
    }
    VL_DEBUG_IF(VL_DBG_MSGF("+ Eval\n"););
    Vsubtractor___024root___eval(&(vlSymsp->TOP));
    // Evaluate cleanup
    Verilated::endOfEval(vlSymsp->__Vm_evalMsgQp);
}

//============================================================
// Events and timing
bool Vsubtractor::eventsPending() { return false; }

uint64_t Vsubtractor::nextTimeSlot() {
    VL_FATAL_MT(__FILE__, __LINE__, "", "%Error: No delays in the design");
    return 0;
}

//============================================================
// Utilities

const char* Vsubtractor::name() const {
    return vlSymsp->name();
}

//============================================================
// Invoke final blocks

void Vsubtractor___024root___eval_final(Vsubtractor___024root* vlSelf);

VL_ATTR_COLD void Vsubtractor::final() {
    Vsubtractor___024root___eval_final(&(vlSymsp->TOP));
}

//============================================================
// Implementations of abstract methods from VerilatedModel

const char* Vsubtractor::hierName() const { return vlSymsp->name(); }
const char* Vsubtractor::modelName() const { return "Vsubtractor"; }
unsigned Vsubtractor::threads() const { return 1; }
void Vsubtractor::prepareClone() const { contextp()->prepareClone(); }
void Vsubtractor::atClone() const {
    contextp()->threadPoolpOnClone();
}
std::unique_ptr<VerilatedTraceConfig> Vsubtractor::traceConfig() const {
    return std::unique_ptr<VerilatedTraceConfig>{new VerilatedTraceConfig{false, false, false}};
};

//============================================================
// Trace configuration

void Vsubtractor___024root__trace_init_top(Vsubtractor___024root* vlSelf, VerilatedVcd* tracep);

VL_ATTR_COLD static void trace_init(void* voidSelf, VerilatedVcd* tracep, uint32_t code) {
    // Callback from tracep->open()
    Vsubtractor___024root* const __restrict vlSelf VL_ATTR_UNUSED = static_cast<Vsubtractor___024root*>(voidSelf);
    Vsubtractor__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    if (!vlSymsp->_vm_contextp__->calcUnusedSigs()) {
        VL_FATAL_MT(__FILE__, __LINE__, __FILE__,
            "Turning on wave traces requires Verilated::traceEverOn(true) call before time 0.");
    }
    vlSymsp->__Vm_baseCode = code;
    tracep->pushPrefix(std::string{vlSymsp->name()}, VerilatedTracePrefixType::SCOPE_MODULE);
    Vsubtractor___024root__trace_init_top(vlSelf, tracep);
    tracep->popPrefix();
}

VL_ATTR_COLD void Vsubtractor___024root__trace_register(Vsubtractor___024root* vlSelf, VerilatedVcd* tracep);

VL_ATTR_COLD void Vsubtractor::trace(VerilatedVcdC* tfp, int levels, int options) {
    if (tfp->isOpen()) {
        vl_fatal(__FILE__, __LINE__, __FILE__,"'Vsubtractor::trace()' shall not be called after 'VerilatedVcdC::open()'.");
    }
    if (false && levels && options) {}  // Prevent unused
    tfp->spTrace()->addModel(this);
    tfp->spTrace()->addInitCb(&trace_init, &(vlSymsp->TOP));
    Vsubtractor___024root__trace_register(&(vlSymsp->TOP), tfp->spTrace());
}
