#include <Vadder.h>

extern "C" {
// CONSTRUCTORS
Vadder*
adder_new() {
Vadder*ptr = new Vadder();
    return ptr;
}

void
adder_delete(Vadder* __ptr) {
delete __ptr;
}

// API METHODS
void
adder_eval(Vadder* __ptr) {
__ptr->eval();
}

void
adder_trace(Vadder* __ptr, VerilatedVcdC* __tfp, int __levels) {
__ptr->trace(__tfp, __levels);
}

void
adder_final(Vadder* __ptr) {
__ptr->final();
}

  // PORTS
  void
  adder_set_clk(Vadder* __ptr, vluint8_t __v) {
    __ptr->clk = __v;
  }

  void
  adder_set_reset(Vadder* __ptr, vluint8_t __v) {
    __ptr->reset = __v;
  }

  void
  adder_set_start_port(Vadder* __ptr, vluint8_t __v) {
    __ptr->start_port = __v;
  }

  void
  adder_set_Pd5(Vadder* __ptr, vluint32_t __v) {
    __ptr->Pd5 = __v;
  }

  void
  adder_set_Pd6(Vadder* __ptr, vluint32_t __v) {
    __ptr->Pd6 = __v;
  }

  vluint8_t
  adder_get_done_port(Vadder* __ptr) {
    return __ptr->done_port;
  }

  vluint32_t
  adder_get_return_port(Vadder* __ptr) {
    return __ptr->return_port;
  }

}
