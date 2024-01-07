#include <Vdivider.h>

extern "C" {
// CONSTRUCTORS
Vdivider*
divider_new() {
Vdivider*ptr = new Vdivider();
    return ptr;
}

void
divider_delete(Vdivider* __ptr) {
delete __ptr;
}

// API METHODS
void
divider_eval(Vdivider* __ptr) {
__ptr->eval();
}

void
divider_trace(Vdivider* __ptr, VerilatedVcdC* __tfp, int __levels) {
__ptr->trace(__tfp, __levels);
}

void
divider_final(Vdivider* __ptr) {
__ptr->final();
}

  // PORTS
  void
  divider_set_clk(Vdivider* __ptr, vluint8_t __v) {
    __ptr->clk = __v;
  }

  void
  divider_set_reset(Vdivider* __ptr, vluint8_t __v) {
    __ptr->reset = __v;
  }

  void
  divider_set_start_port(Vdivider* __ptr, vluint8_t __v) {
    __ptr->start_port = __v;
  }

  void
  divider_set_Pd5(Vdivider* __ptr, vluint32_t __v) {
    __ptr->Pd5 = __v;
  }

  void
  divider_set_P0(Vdivider* __ptr, vluint32_t __v) {
    __ptr->P0 = __v;
  }

  vluint8_t
  divider_get_done_port(Vdivider* __ptr) {
    return __ptr->done_port;
  }

  vluint32_t
  divider_get_return_port(Vdivider* __ptr) {
    return __ptr->return_port;
  }

}
