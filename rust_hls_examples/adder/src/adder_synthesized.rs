// This file was generated by rust_hls. Please do not edit it manually.
// rust_hls hash: "1eeedabc644c9a30dd40f89f3f5ac3b6"

use ::rust_hdl::prelude::*;
#[allow(dead_code, unused)]
mod adder_verilated {
    use std::path::Path;
    mod ffi {
        #[allow(non_camel_case_types)]
        pub enum adder {}
        extern {
            pub fn adder_new() -> *mut adder;
            pub fn adder_delete(adder: *mut adder);
            pub fn adder_eval(adder: *mut adder);
            pub fn adder_final(adder: *mut adder);
            pub fn adder_set_clk(adder: *mut adder, v: ::std::os::raw::c_uchar);
            pub fn adder_set_reset(adder: *mut adder, v: ::std::os::raw::c_uchar);
            pub fn adder_set_start_port(adder: *mut adder, v: ::std::os::raw::c_uchar);
            pub fn adder_set_Pd5(adder: *mut adder, v: ::std::os::raw::c_uint);
            pub fn adder_set_Pd6(adder: *mut adder, v: ::std::os::raw::c_uint);
            pub fn adder_get_done_port(adder: *mut adder) -> ::std::os::raw::c_uchar;
            pub fn adder_get_return_port(adder: *mut adder) -> ::std::os::raw::c_uint;
        }
    }
    pub struct AdderVerilated(*mut ffi::adder, Option<u32>);
    impl Default for AdderVerilated {
        fn default() -> Self {
            let ptr = unsafe { ffi::adder_new() };
            assert!(! ptr.is_null());
            AdderVerilated(ptr, None)
        }
    }
    impl Drop for AdderVerilated {
        fn drop(&mut self) {
            unsafe {
                ffi::adder_delete(self.0);
            }
        }
    }
    #[allow(dead_code, non_snake_case)]
    impl AdderVerilated {
        pub fn set_clk(&mut self, v: u8) {
            unsafe {
                ffi::adder_set_clk(self.0, v);
            }
        }
        pub fn set_reset(&mut self, v: u8) {
            unsafe {
                ffi::adder_set_reset(self.0, v);
            }
        }
        pub fn set_start_port(&mut self, v: u8) {
            unsafe {
                ffi::adder_set_start_port(self.0, v);
            }
        }
        pub fn set_Pd5(&mut self, v: u32) {
            unsafe {
                ffi::adder_set_Pd5(self.0, v);
            }
        }
        pub fn set_Pd6(&mut self, v: u32) {
            unsafe {
                ffi::adder_set_Pd6(self.0, v);
            }
        }
        pub fn done_port(&self) -> u8 {
            unsafe { ffi::adder_get_done_port(self.0) }
        }
        pub fn return_port(&self) -> u32 {
            unsafe { ffi::adder_get_return_port(self.0) }
        }
        pub fn eval(&mut self) {
            unsafe {
                ffi::adder_eval(self.0);
            }
        }
        pub fn finish(&mut self) {
            unsafe {
                ffi::adder_final(self.0);
            }
        }
        pub fn clock_toggle(&mut self) {
            unimplemented!();
        }
        fn reset_up(&mut self) {
            unimplemented!();
        }
        fn reset_down(&mut self) {
            unimplemented!();
        }
    }
}
#[derive(::std::default::Default)]
pub struct Adder {
    pub clk: rust_hdl::prelude::Signal<
        ::rust_hdl::prelude::In,
        ::rust_hdl::prelude::Clock,
    >,
    pub reset: rust_hdl::prelude::Signal<::rust_hdl::prelude::In, bool>,
    pub start_port: rust_hdl::prelude::Signal<::rust_hdl::prelude::In, bool>,
    pub a: rust_hdl::prelude::Signal<
        ::rust_hdl::prelude::In,
        ::rust_hdl::prelude::Bits<32usize>,
    >,
    pub b: rust_hdl::prelude::Signal<
        ::rust_hdl::prelude::In,
        ::rust_hdl::prelude::Bits<32usize>,
    >,
    pub done_port: rust_hdl::prelude::Signal<::rust_hdl::prelude::Out, bool>,
    pub return_port: rust_hdl::prelude::Signal<
        ::rust_hdl::prelude::Out,
        ::rust_hdl::prelude::Bits<32usize>,
    >,
    verilated_module: ::std::sync::Arc<
        ::std::sync::Mutex<self::adder_verilated::AdderVerilated>,
    >,
}
unsafe impl Send for Adder {}
#[automatically_derived]
impl ::rust_hdl::prelude::block::Block for Adder {
    fn connect_all(&mut self) {
        self.connect();
        self.clk.connect_all();
        self.reset.connect_all();
        self.start_port.connect_all();
        self.a.connect_all();
        self.b.connect_all();
        self.done_port.connect_all();
        self.return_port.connect_all();
    }
    fn update_all(&mut self) {
        self.update();
        self.clk.update_all();
        self.reset.update_all();
        self.start_port.update_all();
        self.a.update_all();
        self.b.update_all();
        self.done_port.update_all();
        self.return_port.update_all();
    }
    fn has_changed(&self) -> bool {
        self.clk.has_changed() || self.reset.has_changed()
            || self.start_port.has_changed() || self.a.has_changed()
            || self.b.has_changed() || self.done_port.has_changed()
            || self.return_port.has_changed() || false || false
    }
    fn accept(&self, name: &str, probe: &mut dyn probe::Probe) {
        probe.visit_start_scope(name, self);
        self.clk.accept("clk", probe);
        self.reset.accept("reset", probe);
        self.start_port.accept("start_port", probe);
        self.a.accept("a", probe);
        self.b.accept("b", probe);
        self.done_port.accept("done_port", probe);
        self.return_port.accept("return_port", probe);
        probe.visit_end_scope(name, self);
    }
}
#[automatically_derived]
impl Adder {
    #[allow(unused)]
    pub fn new() -> Self {
        Self::default()
    }
}
#[automatically_derived]
impl ::rust_hdl::prelude::Logic for Adder {
    fn update(&mut self) {
        let mut verilated_module = match self.verilated_module.lock() {
            Ok(verilated_module) => verilated_module,
            Err(e) => panic!("Failed to aquire verilated_module lock: {}", e),
        };
        verilated_module.set_clk(if self.clk.val().clk { 1u8 } else { 0u8 });
        verilated_module.set_reset(if self.reset.val() { 1u8 } else { 0u8 });
        verilated_module.set_start_port(if self.start_port.val() { 1u8 } else { 0u8 });
        verilated_module.set_Pd5(self.a.val().to_u32());
        verilated_module.set_Pd6(self.b.val().to_u32());
        verilated_module.eval();
        self.done_port.next = verilated_module.done_port() != 0;
        self
            .return_port
            .next = rust_hdl::prelude::ToBits::to_bits::<
            32usize,
        >(verilated_module.return_port() & 4294967295u32);
    }
    fn connect(&mut self) {
        self.done_port.connect();
        self.return_port.connect();
    }
    fn hdl(&self) -> ::rust_hdl::prelude::Verilog {
        ::rust_hdl::prelude::Verilog::Wrapper(::rust_hdl::prelude::Wrapper {
            code: "adder adder_inst(.clk(clk), .reset(reset), .start_port(start_port), .Pd5(a), .Pd6(b), .done_port(done_port), .return_port(return_port));"
                .into(),
            cores: "// \n// Politecnico di Milano\n// Code created using PandA - Version: PandA 2023.2 - Revision 891ec3caed502474cab0813cc4a9fc678deabaa5 - Date 2024-01-07T14:45:17\n// /sbin/bambu executed with: /sbin/bambu --simulator=VERILATOR --top-fname=adder --clock-name=clk --compiler=I386_CLANG16 result.ll \n// \n// Send any bug to: panda-info@polimi.it\n// ************************************************************************\n// The following text holds for all the components tagged with PANDA_LGPLv3.\n// They are all part of the BAMBU/PANDA IP LIBRARY.\n// This library is free software; you can redistribute it and/or\n// modify it under the terms of the GNU Lesser General Public\n// License as published by the Free Software Foundation; either\n// version 3 of the License, or (at your option) any later version.\n// \n// This library is distributed in the hope that it will be useful,\n// but WITHOUT ANY WARRANTY; without even the implied warranty of\n// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU\n// Lesser General Public License for more details.\n// \n// You should have received a copy of the GNU Lesser General Public\n// License along with the PandA framework; see the files COPYING.LIB\n// If not, see <http://www.gnu.org/licenses/>.\n// ************************************************************************\n\n`ifdef __ICARUS__\n  `define _SIM_HAVE_CLOG2\n`endif\n`ifdef VERILATOR\n  `define _SIM_HAVE_CLOG2\n`endif\n`ifdef MODEL_TECH\n  `define _SIM_HAVE_CLOG2\n`endif\n`ifdef VCS\n  `define _SIM_HAVE_CLOG2\n`endif\n`ifdef NCVERILOG\n  `define _SIM_HAVE_CLOG2\n`endif\n`ifdef XILINX_SIMULATOR\n  `define _SIM_HAVE_CLOG2\n`endif\n`ifdef XILINX_ISIM\n  `define _SIM_HAVE_CLOG2\n`endif\n\n// This component is part of the BAMBU/PANDA IP LIBRARY\n// Copyright (C) 2004-2023 Politecnico di Milano\n// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>\n// License: PANDA_LGPLv3\n`timescale 1ns / 1ps\nmodule ui_plus_expr_FU(in1,\n  in2,\n  out1);\n  parameter BITSIZE_in1=1,\n    BITSIZE_in2=1,\n    BITSIZE_out1=1;\n  // IN\n  input [BITSIZE_in1-1:0] in1;\n  input [BITSIZE_in2-1:0] in2;\n  // OUT\n  output [BITSIZE_out1-1:0] out1;\n  assign out1 = in1 + in2;\nendmodule\n\n// Datapath RTL description for adder\n// This component has been derived from the input source code and so it does not fall under the copyright of PandA framework, but it follows the input source code copyright, and may be aggregated with components of the BAMBU/PANDA IP LIBRARY.\n// Author(s): Component automatically generated by bambu\n// License: THIS COMPONENT IS PROVIDED \"AS IS\" AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.\n`timescale 1ns / 1ps\nmodule datapath_adder(clock,\n  reset,\n  in_port_Pd5,\n  in_port_Pd6,\n  return_port);\n  // IN\n  input clock;\n  input reset;\n  input [31:0] in_port_Pd5;\n  input [31:0] in_port_Pd6;\n  // OUT\n  output [31:0] return_port;\n  // Component and signal declarations\n  wire [31:0] out_ui_plus_expr_FU_32_32_32_3_i0_fu_adder_423514_423534;\n  \n  ui_plus_expr_FU #(.BITSIZE_in1(32),\n    .BITSIZE_in2(32),\n    .BITSIZE_out1(32)) fu_adder_423514_423534 (.out1(out_ui_plus_expr_FU_32_32_32_3_i0_fu_adder_423514_423534),\n    .in1(in_port_Pd6),\n    .in2(in_port_Pd5));\n  // io-signal post fix\n  assign return_port = out_ui_plus_expr_FU_32_32_32_3_i0_fu_adder_423514_423534;\n\nendmodule\n\n// FSM based controller description for adder\n// This component has been derived from the input source code and so it does not fall under the copyright of PandA framework, but it follows the input source code copyright, and may be aggregated with components of the BAMBU/PANDA IP LIBRARY.\n// Author(s): Component automatically generated by bambu\n// License: THIS COMPONENT IS PROVIDED \"AS IS\" AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.\n`timescale 1ns / 1ps\nmodule controller_adder(done_port,\n  clock,\n  reset,\n  start_port);\n  // IN\n  input clock;\n  input reset;\n  input start_port;\n  // OUT\n  output done_port;\n  parameter [0:0] S_0 = 1'b1;\n  reg [0:0] _present_state=S_0, _next_state;\n  reg done_port;\n  \n  always @(posedge clock)\n    if (reset == 1'b0) _present_state <= S_0;\n    else _present_state <= _next_state;\n  \n  always @(*)\n  begin\n    done_port = 1'b0;\n    case (_present_state)\n      S_0 :\n        if(start_port == 1'b1)\n        begin\n          _next_state = S_0;\n          done_port = 1'b1;\n        end\n        else\n        begin\n          _next_state = S_0;\n        end\n      default :\n        begin\n          _next_state = S_0;\n        end\n    endcase\n  end\nendmodule\n\n// Top component for adder\n// This component has been derived from the input source code and so it does not fall under the copyright of PandA framework, but it follows the input source code copyright, and may be aggregated with components of the BAMBU/PANDA IP LIBRARY.\n// Author(s): Component automatically generated by bambu\n// License: THIS COMPONENT IS PROVIDED \"AS IS\" AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.\n`timescale 1ns / 1ps\nmodule _adder(clock,\n  reset,\n  start_port,\n  done_port,\n  Pd5,\n  Pd6,\n  return_port);\n  // IN\n  input clock;\n  input reset;\n  input start_port;\n  input [31:0] Pd5;\n  input [31:0] Pd6;\n  // OUT\n  output done_port;\n  output [31:0] return_port;\n  // Component and signal declarations\n  \n  controller_adder Controller_i (.done_port(done_port),\n    .clock(clock),\n    .reset(reset),\n    .start_port(start_port));\n  datapath_adder Datapath_i (.return_port(return_port),\n    .clock(clock),\n    .reset(reset),\n    .in_port_Pd5(Pd5),\n    .in_port_Pd6(Pd6));\n\nendmodule\n\n// This component is part of the BAMBU/PANDA IP LIBRARY\n// Copyright (C) 2004-2023 Politecnico di Milano\n// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>\n// License: PANDA_LGPLv3\n`timescale 1ns / 1ps\nmodule ui_view_convert_expr_FU(in1,\n  out1);\n  parameter BITSIZE_in1=1,\n    BITSIZE_out1=1;\n  // IN\n  input [BITSIZE_in1-1:0] in1;\n  // OUT\n  output [BITSIZE_out1-1:0] out1;\n  assign out1 = in1;\nendmodule\n\n// Minimal interface for function: adder\n// This component has been derived from the input source code and so it does not fall under the copyright of PandA framework, but it follows the input source code copyright, and may be aggregated with components of the BAMBU/PANDA IP LIBRARY.\n// Author(s): Component automatically generated by bambu\n// License: THIS COMPONENT IS PROVIDED \"AS IS\" AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.\n`timescale 1ns / 1ps\nmodule adder(clk,\n  reset,\n  start_port,\n  Pd5,\n  Pd6,\n  done_port,\n  return_port);\n  // IN\n  input clk;\n  input reset;\n  input start_port;\n  input [31:0] Pd5;\n  input [31:0] Pd6;\n  // OUT\n  output done_port;\n  output [31:0] return_port;\n  // Component and signal declarations\n  wire [31:0] out_return_port_ui_view_convert_expr_FU;\n  \n  _adder _adder_i0 (.done_port(done_port),\n    .return_port(out_return_port_ui_view_convert_expr_FU),\n    .clock(clk),\n    .reset(reset),\n    .start_port(start_port),\n    .Pd5(Pd5),\n    .Pd6(Pd6));\n  ui_view_convert_expr_FU #(.BITSIZE_in1(32),\n    .BITSIZE_out1(32)) return_port_ui_view_convert_expr_FU (.out1(return_port),\n    .in1(out_return_port_ui_view_convert_expr_FU));\n\nendmodule\n\n\n"
                .into(),
        })
    }
}


#[allow(dead_code)]
const VERILOG: &str = r#"// 
// Politecnico di Milano
// Code created using PandA - Version: PandA 2023.2 - Revision 891ec3caed502474cab0813cc4a9fc678deabaa5 - Date 2024-01-07T14:45:17
// /sbin/bambu executed with: /sbin/bambu --simulator=VERILATOR --top-fname=adder --clock-name=clk --compiler=I386_CLANG16 result.ll 
// 
// Send any bug to: panda-info@polimi.it
// ************************************************************************
// The following text holds for all the components tagged with PANDA_LGPLv3.
// They are all part of the BAMBU/PANDA IP LIBRARY.
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 3 of the License, or (at your option) any later version.
// 
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
// 
// You should have received a copy of the GNU Lesser General Public
// License along with the PandA framework; see the files COPYING.LIB
// If not, see <http://www.gnu.org/licenses/>.
// ************************************************************************

`ifdef __ICARUS__
  `define _SIM_HAVE_CLOG2
`endif
`ifdef VERILATOR
  `define _SIM_HAVE_CLOG2
`endif
`ifdef MODEL_TECH
  `define _SIM_HAVE_CLOG2
`endif
`ifdef VCS
  `define _SIM_HAVE_CLOG2
`endif
`ifdef NCVERILOG
  `define _SIM_HAVE_CLOG2
`endif
`ifdef XILINX_SIMULATOR
  `define _SIM_HAVE_CLOG2
`endif
`ifdef XILINX_ISIM
  `define _SIM_HAVE_CLOG2
`endif

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module ui_plus_expr_FU(in1,
  in2,
  out1);
  parameter BITSIZE_in1=1,
    BITSIZE_in2=1,
    BITSIZE_out1=1;
  // IN
  input [BITSIZE_in1-1:0] in1;
  input [BITSIZE_in2-1:0] in2;
  // OUT
  output [BITSIZE_out1-1:0] out1;
  assign out1 = in1 + in2;
endmodule

// Datapath RTL description for adder
// This component has been derived from the input source code and so it does not fall under the copyright of PandA framework, but it follows the input source code copyright, and may be aggregated with components of the BAMBU/PANDA IP LIBRARY.
// Author(s): Component automatically generated by bambu
// License: THIS COMPONENT IS PROVIDED "AS IS" AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
`timescale 1ns / 1ps
module datapath_adder(clock,
  reset,
  in_port_Pd5,
  in_port_Pd6,
  return_port);
  // IN
  input clock;
  input reset;
  input [31:0] in_port_Pd5;
  input [31:0] in_port_Pd6;
  // OUT
  output [31:0] return_port;
  // Component and signal declarations
  wire [31:0] out_ui_plus_expr_FU_32_32_32_3_i0_fu_adder_423514_423534;
  
  ui_plus_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(32),
    .BITSIZE_out1(32)) fu_adder_423514_423534 (.out1(out_ui_plus_expr_FU_32_32_32_3_i0_fu_adder_423514_423534),
    .in1(in_port_Pd6),
    .in2(in_port_Pd5));
  // io-signal post fix
  assign return_port = out_ui_plus_expr_FU_32_32_32_3_i0_fu_adder_423514_423534;

endmodule

// FSM based controller description for adder
// This component has been derived from the input source code and so it does not fall under the copyright of PandA framework, but it follows the input source code copyright, and may be aggregated with components of the BAMBU/PANDA IP LIBRARY.
// Author(s): Component automatically generated by bambu
// License: THIS COMPONENT IS PROVIDED "AS IS" AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
`timescale 1ns / 1ps
module controller_adder(done_port,
  clock,
  reset,
  start_port);
  // IN
  input clock;
  input reset;
  input start_port;
  // OUT
  output done_port;
  parameter [0:0] S_0 = 1'b1;
  reg [0:0] _present_state=S_0, _next_state;
  reg done_port;
  
  always @(posedge clock)
    if (reset == 1'b0) _present_state <= S_0;
    else _present_state <= _next_state;
  
  always @(*)
  begin
    done_port = 1'b0;
    case (_present_state)
      S_0 :
        if(start_port == 1'b1)
        begin
          _next_state = S_0;
          done_port = 1'b1;
        end
        else
        begin
          _next_state = S_0;
        end
      default :
        begin
          _next_state = S_0;
        end
    endcase
  end
endmodule

// Top component for adder
// This component has been derived from the input source code and so it does not fall under the copyright of PandA framework, but it follows the input source code copyright, and may be aggregated with components of the BAMBU/PANDA IP LIBRARY.
// Author(s): Component automatically generated by bambu
// License: THIS COMPONENT IS PROVIDED "AS IS" AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
`timescale 1ns / 1ps
module _adder(clock,
  reset,
  start_port,
  done_port,
  Pd5,
  Pd6,
  return_port);
  // IN
  input clock;
  input reset;
  input start_port;
  input [31:0] Pd5;
  input [31:0] Pd6;
  // OUT
  output done_port;
  output [31:0] return_port;
  // Component and signal declarations
  
  controller_adder Controller_i (.done_port(done_port),
    .clock(clock),
    .reset(reset),
    .start_port(start_port));
  datapath_adder Datapath_i (.return_port(return_port),
    .clock(clock),
    .reset(reset),
    .in_port_Pd5(Pd5),
    .in_port_Pd6(Pd6));

endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module ui_view_convert_expr_FU(in1,
  out1);
  parameter BITSIZE_in1=1,
    BITSIZE_out1=1;
  // IN
  input [BITSIZE_in1-1:0] in1;
  // OUT
  output [BITSIZE_out1-1:0] out1;
  assign out1 = in1;
endmodule

// Minimal interface for function: adder
// This component has been derived from the input source code and so it does not fall under the copyright of PandA framework, but it follows the input source code copyright, and may be aggregated with components of the BAMBU/PANDA IP LIBRARY.
// Author(s): Component automatically generated by bambu
// License: THIS COMPONENT IS PROVIDED "AS IS" AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
`timescale 1ns / 1ps
module adder(clk,
  reset,
  start_port,
  Pd5,
  Pd6,
  done_port,
  return_port);
  // IN
  input clk;
  input reset;
  input start_port;
  input [31:0] Pd5;
  input [31:0] Pd6;
  // OUT
  output done_port;
  output [31:0] return_port;
  // Component and signal declarations
  wire [31:0] out_return_port_ui_view_convert_expr_FU;
  
  _adder _adder_i0 (.done_port(done_port),
    .return_port(out_return_port_ui_view_convert_expr_FU),
    .clock(clk),
    .reset(reset),
    .start_port(start_port),
    .Pd5(Pd5),
    .Pd6(Pd6));
  ui_view_convert_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) return_port_ui_view_convert_expr_FU (.out1(return_port),
    .in1(out_return_port_ui_view_convert_expr_FU));

endmodule


"#;