#include <Vsubtractor.h>

extern "C" {
// CONSTRUCTORS
Vsubtractor*
subtractor_new() {
Vsubtractor*ptr = new Vsubtractor();
    return ptr;
}

void
subtractor_delete(Vsubtractor* __ptr) {
delete __ptr;
}

// API METHODS
void
subtractor_eval(Vsubtractor* __ptr) {
__ptr->eval();
}

void
subtractor_trace(Vsubtractor* __ptr, VerilatedVcdC* __tfp, int __levels) {
__ptr->trace(__tfp, __levels);
}

void
subtractor_final(Vsubtractor* __ptr) {
__ptr->final();
}

  // PORTS
  void
  subtractor_set_clk(Vsubtractor* __ptr, vluint8_t __v) {
    __ptr->clk = __v;
  }

  void
  subtractor_set_reset(Vsubtractor* __ptr, vluint8_t __v) {
    __ptr->reset = __v;
  }

  void
  subtractor_set_start_port(Vsubtractor* __ptr, vluint8_t __v) {
    __ptr->start_port = __v;
  }

  void
  subtractor_set_Pd5(Vsubtractor* __ptr, vluint32_t __v) {
    __ptr->Pd5 = __v;
  }

  void
  subtractor_set_Pd6(Vsubtractor* __ptr, vluint32_t __v) {
    __ptr->Pd6 = __v;
  }

  vluint8_t
  subtractor_get_done_port(Vsubtractor* __ptr) {
    return __ptr->done_port;
  }

  vluint32_t
  subtractor_get_return_port(Vsubtractor* __ptr) {
    return __ptr->return_port;
  }

}
