// 
// Politecnico di Milano
// Code created using PandA - Version: PandA 2023.12 - Revision 891ec3caed502474cab0813cc4a9fc678deabaa5 - Date 2024-01-07T17:21:45
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
module ui_minus_expr_FU(in1,
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
  assign out1 = in1 - in2;
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
  wire [31:0] out_ui_minus_expr_FU_32_32_32_3_i0_fu_adder_428394_428414;
  
  ui_minus_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(32),
    .BITSIZE_out1(32)) fu_adder_428394_428414 (.out1(out_ui_minus_expr_FU_32_32_32_3_i0_fu_adder_428394_428414),
    .in1(in_port_Pd6),
    .in2(in_port_Pd5));
  // io-signal post fix
  assign return_port = out_ui_minus_expr_FU_32_32_32_3_i0_fu_adder_428394_428414;

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


