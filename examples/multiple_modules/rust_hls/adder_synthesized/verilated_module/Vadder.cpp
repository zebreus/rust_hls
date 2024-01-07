// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Model implementation (design independent parts)

#include "Vadder__pch.h"
#include "verilated_vcd_c.h"

//============================================================
// Constructors

Vadder::Vadder(VerilatedContext* _vcontextp__, const char* _vcname__)
    : VerilatedModel{*_vcontextp__}
    , vlSymsp{new Vadder__Syms(contextp(), _vcname__, this)}
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

Vadder::Vadder(const char* _vcname__)
    : Vadder(Verilated::threadContextp(), _vcname__)
{
}

//============================================================
// Destructor

Vadder::~Vadder() {
    delete vlSymsp;
}

//============================================================
// Evaluation function

#ifdef VL_DEBUG
void Vadder___024root___eval_debug_assertions(Vadder___024root* vlSelf);
#endif  // VL_DEBUG
void Vadder___024root___eval_static(Vadder___024root* vlSelf);
void Vadder___024root___eval_initial(Vadder___024root* vlSelf);
void Vadder___024root___eval_settle(Vadder___024root* vlSelf);
void Vadder___024root___eval(Vadder___024root* vlSelf);

void Vadder::eval_step() {
    VL_DEBUG_IF(VL_DBG_MSGF("+++++TOP Evaluate Vadder::eval_step\n"); );
#ifdef VL_DEBUG
    // Debug assertions
    Vadder___024root___eval_debug_assertions(&(vlSymsp->TOP));
#endif  // VL_DEBUG
    vlSymsp->__Vm_activity = true;
    vlSymsp->__Vm_deleter.deleteAll();
    if (VL_UNLIKELY(!vlSymsp->__Vm_didInit)) {
        vlSymsp->__Vm_didInit = true;
        VL_DEBUG_IF(VL_DBG_MSGF("+ Initial\n"););
        Vadder___024root___eval_static(&(vlSymsp->TOP));
        Vadder___024root___eval_initial(&(vlSymsp->TOP));
        Vadder___024root___eval_settle(&(vlSymsp->TOP));
    }
    VL_DEBUG_IF(VL_DBG_MSGF("+ Eval\n"););
    Vadder___024root___eval(&(vlSymsp->TOP));
    // Evaluate cleanup
    Verilated::endOfEval(vlSymsp->__Vm_evalMsgQp);
}

//============================================================
// Events and timing
bool Vadder::eventsPending() { return false; }

uint64_t Vadder::nextTimeSlot() {
    VL_FATAL_MT(__FILE__, __LINE__, "", "%Error: No delays in the design");
    return 0;
}

//============================================================
// Utilities

const char* Vadder::name() const {
    return vlSymsp->name();
}

//============================================================
// Invoke final blocks

void Vadder___024root___eval_final(Vadder___024root* vlSelf);

VL_ATTR_COLD void Vadder::final() {
    Vadder___024root___eval_final(&(vlSymsp->TOP));
}

//============================================================
// Implementations of abstract methods from VerilatedModel

const char* Vadder::hierName() const { return vlSymsp->name(); }
const char* Vadder::modelName() const { return "Vadder"; }
unsigned Vadder::threads() const { return 1; }
void Vadder::prepareClone() const { contextp()->prepareClone(); }
void Vadder::atClone() const {
    contextp()->threadPoolpOnClone();
}
std::unique_ptr<VerilatedTraceConfig> Vadder::traceConfig() const {
    return std::unique_ptr<VerilatedTraceConfig>{new VerilatedTraceConfig{false, false, false}};
};

//============================================================
// Trace configuration

void Vadder___024root__trace_init_top(Vadder___024root* vlSelf, VerilatedVcd* tracep);

VL_ATTR_COLD static void trace_init(void* voidSelf, VerilatedVcd* tracep, uint32_t code) {
    // Callback from tracep->open()
    Vadder___024root* const __restrict vlSelf VL_ATTR_UNUSED = static_cast<Vadder___024root*>(voidSelf);
    Vadder__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    if (!vlSymsp->_vm_contextp__->calcUnusedSigs()) {
        VL_FATAL_MT(__FILE__, __LINE__, __FILE__,
            "Turning on wave traces requires Verilated::traceEverOn(true) call before time 0.");
    }
    vlSymsp->__Vm_baseCode = code;
    tracep->pushPrefix(std::string{vlSymsp->name()}, VerilatedTracePrefixType::SCOPE_MODULE);
    Vadder___024root__trace_init_top(vlSelf, tracep);
    tracep->popPrefix();
}

VL_ATTR_COLD void Vadder___024root__trace_register(Vadder___024root* vlSelf, VerilatedVcd* tracep);

VL_ATTR_COLD void Vadder::trace(VerilatedVcdC* tfp, int levels, int options) {
    if (tfp->isOpen()) {
        vl_fatal(__FILE__, __LINE__, __FILE__,"'Vadder::trace()' shall not be called after 'VerilatedVcdC::open()'.");
    }
    if (false && levels && options) {}  // Prevent unused
    tfp->spTrace()->addModel(this);
    tfp->spTrace()->addInitCb(&trace_init, &(vlSymsp->TOP));
    Vadder___024root__trace_register(&(vlSymsp->TOP), tfp->spTrace());
}
