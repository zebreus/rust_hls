#include <Vother_function.h>

extern "C" {
// CONSTRUCTORS
Vother_function*
other_function_new() {
Vother_function*ptr = new Vother_function();
    return ptr;
}

void
other_function_delete(Vother_function* __ptr) {
delete __ptr;
}

// API METHODS
void
other_function_eval(Vother_function* __ptr) {
__ptr->eval();
}

void
other_function_trace(Vother_function* __ptr, VerilatedVcdC* __tfp, int __levels) {
__ptr->trace(__tfp, __levels);
}

void
other_function_final(Vother_function* __ptr) {
__ptr->final();
}

  // PORTS
  void
  other_function_set_clk(Vother_function* __ptr, vluint8_t __v) {
    __ptr->clk = __v;
  }

  void
  other_function_set_reset(Vother_function* __ptr, vluint8_t __v) {
    __ptr->reset = __v;
  }

  void
  other_function_set_start_port(Vother_function* __ptr, vluint8_t __v) {
    __ptr->start_port = __v;
  }

  void
  other_function_set_Pd5(Vother_function* __ptr, vluint32_t __v) {
    __ptr->Pd5 = __v;
  }

  void
  other_function_set_Pd6(Vother_function* __ptr, vluint32_t __v) {
    __ptr->Pd6 = __v;
  }

  vluint8_t
  other_function_get_done_port(Vother_function* __ptr) {
    return __ptr->done_port;
  }

  vluint32_t
  other_function_get_return_port(Vother_function* __ptr) {
    return __ptr->return_port;
  }

}
