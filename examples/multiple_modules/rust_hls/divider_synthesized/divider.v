// 
// Politecnico di Milano
// Code created using PandA - Version: PandA 2023.12 - Revision 891ec3caed502474cab0813cc4a9fc678deabaa5 - Date 2024-01-07T21:54:44
// /sbin/bambu executed with: /sbin/bambu --simulator=VERILATOR --top-fname=divider --clock-name=clk --compiler=I386_CLANG16 result.ll 
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
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>, Christian Pilato <christian.pilato@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module constant_value(out1);
  parameter BITSIZE_out1=1,
    value=1'b0;
  // OUT
  output [BITSIZE_out1-1:0] out1;
  assign out1 = value;
endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module register_SE(clock,
  reset,
  in1,
  wenable,
  out1);
  parameter BITSIZE_in1=1,
    BITSIZE_out1=1;
  // IN
  input clock;
  input reset;
  input [BITSIZE_in1-1:0] in1;
  input wenable;
  // OUT
  output [BITSIZE_out1-1:0] out1;
  
  reg [BITSIZE_out1-1:0] reg_out1 =0;
  assign out1 = reg_out1;
  always @(posedge clock)
    if (wenable)
      reg_out1 <= in1;
endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module ui_cond_expr_FU(in1,
  in2,
  in3,
  out1);
  parameter BITSIZE_in1=1,
    BITSIZE_in2=1,
    BITSIZE_in3=1,
    BITSIZE_out1=1;
  // IN
  input [BITSIZE_in1-1:0] in1;
  input [BITSIZE_in2-1:0] in2;
  input [BITSIZE_in3-1:0] in3;
  // OUT
  output [BITSIZE_out1-1:0] out1;
  assign out1 = in1 != 0 ? in2 : in3;
endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module ui_eq_expr_FU(in1,
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
  assign out1 = in1 == in2;
endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module register_STD(clock,
  reset,
  in1,
  wenable,
  out1);
  parameter BITSIZE_in1=1,
    BITSIZE_out1=1;
  // IN
  input clock;
  input reset;
  input [BITSIZE_in1-1:0] in1;
  input wenable;
  // OUT
  output [BITSIZE_out1-1:0] out1;
  reg [BITSIZE_out1-1:0] reg_out1 =0;
  assign out1 = reg_out1;
  always @(posedge clock)
    reg_out1 <= in1;

endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module ARRAY_1D_STD_DISTRAM_NN_SDS(clock,
  reset,
  in1,
  in2r,
  in2w,
  in3r,
  in3w,
  in4r,
  in4w,
  out1,
  sel_LOAD,
  sel_STORE,
  S_oe_ram,
  S_we_ram,
  S_addr_ram,
  S_Wdata_ram,
  Sin_Rdata_ram,
  Sout_Rdata_ram,
  S_data_ram_size,
  Sin_DataRdy,
  Sout_DataRdy,
  proxy_in1,
  proxy_in2r,
  proxy_in2w,
  proxy_in3r,
  proxy_in3w,
  proxy_in4r,
  proxy_in4w,
  proxy_sel_LOAD,
  proxy_sel_STORE,
  proxy_out1);
  parameter BITSIZE_in1=1, PORTSIZE_in1=2,
    BITSIZE_in2r=1, PORTSIZE_in2r=2,
    BITSIZE_in2w=1, PORTSIZE_in2w=2,
    BITSIZE_in3r=1, PORTSIZE_in3r=2,
    BITSIZE_in3w=1, PORTSIZE_in3w=2,
    BITSIZE_in4r=1, PORTSIZE_in4r=2,
    BITSIZE_in4w=1, PORTSIZE_in4w=2,
    BITSIZE_sel_LOAD=1, PORTSIZE_sel_LOAD=2,
    BITSIZE_sel_STORE=1, PORTSIZE_sel_STORE=2,
    BITSIZE_S_oe_ram=1, PORTSIZE_S_oe_ram=2,
    BITSIZE_S_we_ram=1, PORTSIZE_S_we_ram=2,
    BITSIZE_out1=1, PORTSIZE_out1=2,
    BITSIZE_S_addr_ram=1, PORTSIZE_S_addr_ram=2,
    BITSIZE_S_Wdata_ram=8, PORTSIZE_S_Wdata_ram=2,
    BITSIZE_Sin_Rdata_ram=8, PORTSIZE_Sin_Rdata_ram=2,
    BITSIZE_Sout_Rdata_ram=8, PORTSIZE_Sout_Rdata_ram=2,
    BITSIZE_S_data_ram_size=1, PORTSIZE_S_data_ram_size=2,
    BITSIZE_Sin_DataRdy=1, PORTSIZE_Sin_DataRdy=2,
    BITSIZE_Sout_DataRdy=1, PORTSIZE_Sout_DataRdy=2,
    MEMORY_INIT_file="array.mem",
    n_elements=1,
    data_size=32,
    address_space_begin=0,
    address_space_rangesize=4,
    BUS_PIPELINED=1,
    PRIVATE_MEMORY=0,
    READ_ONLY_MEMORY=0,
    USE_SPARSE_MEMORY=1,
    ALIGNMENT=32,
    BITSIZE_proxy_in1=1, PORTSIZE_proxy_in1=2,
    BITSIZE_proxy_in2r=1, PORTSIZE_proxy_in2r=2,
    BITSIZE_proxy_in2w=1, PORTSIZE_proxy_in2w=2,
    BITSIZE_proxy_in3r=1, PORTSIZE_proxy_in3r=2,
    BITSIZE_proxy_in3w=1, PORTSIZE_proxy_in3w=2,
    BITSIZE_proxy_in4r=1, PORTSIZE_proxy_in4r=2,
    BITSIZE_proxy_in4w=1, PORTSIZE_proxy_in4w=2,
    BITSIZE_proxy_sel_LOAD=1, PORTSIZE_proxy_sel_LOAD=2,
    BITSIZE_proxy_sel_STORE=1, PORTSIZE_proxy_sel_STORE=2,
    BITSIZE_proxy_out1=1, PORTSIZE_proxy_out1=2;
  // IN
  input clock;
  input reset;
  input [(PORTSIZE_in1*BITSIZE_in1)+(-1):0] in1;
  input [(PORTSIZE_in2r*BITSIZE_in2r)+(-1):0] in2r;
  input [(PORTSIZE_in2w*BITSIZE_in2w)+(-1):0] in2w;
  input [(PORTSIZE_in3r*BITSIZE_in3r)+(-1):0] in3r;
  input [(PORTSIZE_in3w*BITSIZE_in3w)+(-1):0] in3w;
  input [PORTSIZE_in4r-1:0] in4r;
  input [PORTSIZE_in4w-1:0] in4w;
  input [PORTSIZE_sel_LOAD-1:0] sel_LOAD;
  input [PORTSIZE_sel_STORE-1:0] sel_STORE;
  input [PORTSIZE_S_oe_ram-1:0] S_oe_ram;
  input [PORTSIZE_S_we_ram-1:0] S_we_ram;
  input [(PORTSIZE_S_addr_ram*BITSIZE_S_addr_ram)+(-1):0] S_addr_ram;
  input [(PORTSIZE_S_Wdata_ram*BITSIZE_S_Wdata_ram)+(-1):0] S_Wdata_ram;
  input [(PORTSIZE_Sin_Rdata_ram*BITSIZE_Sin_Rdata_ram)+(-1):0] Sin_Rdata_ram;
  input [(PORTSIZE_S_data_ram_size*BITSIZE_S_data_ram_size)+(-1):0] S_data_ram_size;
  input [PORTSIZE_Sin_DataRdy-1:0] Sin_DataRdy;
  input [(PORTSIZE_proxy_in1*BITSIZE_proxy_in1)+(-1):0] proxy_in1;
  input [(PORTSIZE_proxy_in2r*BITSIZE_proxy_in2r)+(-1):0] proxy_in2r;
  input [(PORTSIZE_proxy_in2w*BITSIZE_proxy_in2w)+(-1):0] proxy_in2w;
  input [(PORTSIZE_proxy_in3r*BITSIZE_proxy_in3r)+(-1):0] proxy_in3r;
  input [(PORTSIZE_proxy_in3w*BITSIZE_proxy_in3w)+(-1):0] proxy_in3w;
  input [(PORTSIZE_proxy_in4r*BITSIZE_proxy_in4r)+(-1):0] proxy_in4r;
  input [(PORTSIZE_proxy_in4w*BITSIZE_proxy_in4w)+(-1):0] proxy_in4w;
  input [PORTSIZE_proxy_sel_LOAD-1:0] proxy_sel_LOAD;
  input [PORTSIZE_proxy_sel_STORE-1:0] proxy_sel_STORE;
  // OUT
  output [(PORTSIZE_out1*BITSIZE_out1)+(-1):0] out1;
  output [(PORTSIZE_Sout_Rdata_ram*BITSIZE_Sout_Rdata_ram)+(-1):0] Sout_Rdata_ram;
  output [PORTSIZE_Sout_DataRdy-1:0] Sout_DataRdy;
  output [(PORTSIZE_proxy_out1*BITSIZE_proxy_out1)+(-1):0] proxy_out1;
  
  `ifndef _SIM_HAVE_CLOG2
      function integer log2;
        input integer value;
        integer temp_value;
        begin
        temp_value = value-1;
        for (log2=0; temp_value>0; log2=log2+1)
          temp_value = temp_value>>1;
        end
      endfunction
  `endif
  parameter n_byte_on_databus = ALIGNMENT/8;
  parameter nbit_addr_r = BITSIZE_in2r > BITSIZE_proxy_in2r ? BITSIZE_in2r : BITSIZE_proxy_in2r;
  parameter nbit_addr_w = BITSIZE_in2w > BITSIZE_proxy_in2w ? BITSIZE_in2w : BITSIZE_proxy_in2w;
  `ifdef _SIM_HAVE_CLOG2
    localparam nbit_read_addr = n_elements == 1 ? 1 : $clog2(n_elements);
    localparam nbits_byte_offset = n_byte_on_databus<=1 ? 0 : $clog2(n_byte_on_databus);
  `else
    localparam nbit_read_addr = n_elements == 1 ? 1 : log2(n_elements);
    localparam nbits_byte_offset = n_byte_on_databus<=1 ? 0 : log2(n_byte_on_databus);
  `endif
  parameter max_n_writes = PORTSIZE_sel_STORE;
  parameter max_n_reads = PORTSIZE_sel_LOAD;
  
  wire [max_n_writes-1:0] bram_write;
  
  wire [nbit_read_addr*max_n_reads-1:0] memory_addr_a_r;
  wire [nbit_read_addr*max_n_writes-1:0] memory_addr_a_w;
  
  wire [data_size*max_n_writes-1:0] din_value_aggregated;
  wire [data_size*max_n_reads-1:0] dout_a;
  wire [nbit_addr_r*max_n_reads-1:0] tmp_addr_r;
  wire [nbit_addr_w*max_n_writes-1:0] tmp_addr_w;
  wire [nbit_addr_r*max_n_reads-1:0] relative_addr_r;
  wire [nbit_addr_w*max_n_writes-1:0] relative_addr_w;
  integer index2;
  
  reg [data_size-1:0] memory [0:n_elements-1] /* synthesis syn_ramstyle = "no_rw_check" */;
  
  initial
  begin
    $readmemb(MEMORY_INIT_file,memory,0,n_elements-1);
  end
  
  generate
  genvar ind2_r;
  for (ind2_r=0; ind2_r<max_n_reads; ind2_r=ind2_r+1)
    begin : Lind2_r
      assign tmp_addr_r[(ind2_r+1)*nbit_addr_r-1:ind2_r*nbit_addr_r] = (proxy_sel_LOAD[ind2_r] && proxy_in4r[ind2_r]) ? proxy_in2r[(ind2_r+1)*BITSIZE_proxy_in2r-1:ind2_r*BITSIZE_proxy_in2r] : in2r[(ind2_r+1)*BITSIZE_in2r-1:ind2_r*BITSIZE_in2r];
    end
  endgenerate
  
  generate
  genvar ind2_w;
  for (ind2_w=0; ind2_w<max_n_writes; ind2_w=ind2_w+1)
    begin : Lind2_w
      assign tmp_addr_w[(ind2_w+1)*nbit_addr_w-1:ind2_w*nbit_addr_w] = (proxy_sel_STORE[ind2_w] && proxy_in4w[ind2_w]) ? proxy_in2w[(ind2_w+1)*BITSIZE_proxy_in2w-1:ind2_w*BITSIZE_proxy_in2w] : in2w[(ind2_w+1)*BITSIZE_in2w-1:ind2_w*BITSIZE_in2w];
    end
  endgenerate
  
  generate
  genvar i6_r;
    for (i6_r=0; i6_r<max_n_reads; i6_r=i6_r+1)
    begin : L6_r
      if(USE_SPARSE_MEMORY==1)
        assign relative_addr_r[(i6_r+1)*nbit_addr_r-1:i6_r*nbit_addr_r] = tmp_addr_r[(i6_r+1)*nbit_addr_r-1:i6_r*nbit_addr_r];
      else
        assign relative_addr_r[(i6_r+1)*nbit_addr_r-1:i6_r*nbit_addr_r] = tmp_addr_r[(i6_r+1)*nbit_addr_r-1:i6_r*nbit_addr_r]-address_space_begin;
    end
  endgenerate
  
  generate
  genvar i6_w;
    for (i6_w=0; i6_w<max_n_writes; i6_w=i6_w+1)
    begin : L6_w
      if(USE_SPARSE_MEMORY==1)
        assign relative_addr_w[(i6_w+1)*nbit_addr_w-1:i6_w*nbit_addr_w] = tmp_addr_w[(i6_w+1)*nbit_addr_w-1:i6_w*nbit_addr_w];
      else
        assign relative_addr_w[(i6_w+1)*nbit_addr_w-1:i6_w*nbit_addr_w] = tmp_addr_w[(i6_w+1)*nbit_addr_w-1:i6_w*nbit_addr_w]-address_space_begin;
    end
  endgenerate
  
  generate
  genvar i7_r;
    for (i7_r=0; i7_r<max_n_reads; i7_r=i7_r+1)
    begin : L7_A_r
      if (n_elements==1)
        assign memory_addr_a_r[(i7_r+1)*nbit_read_addr-1:i7_r*nbit_read_addr] = {nbit_read_addr{1'b0}};
      else
        assign memory_addr_a_r[(i7_r+1)*nbit_read_addr-1:i7_r*nbit_read_addr] = relative_addr_r[nbit_read_addr+nbits_byte_offset-1+i7_r*nbit_addr_r:nbits_byte_offset+i7_r*nbit_addr_r];
    end
  endgenerate
  
  generate
  genvar i7_w;
    for (i7_w=0; i7_w<max_n_writes; i7_w=i7_w+1)
    begin : L7_A_w
      if (n_elements==1)
        assign memory_addr_a_w[(i7_w+1)*nbit_read_addr-1:i7_w*nbit_read_addr] = {nbit_read_addr{1'b0}};
      else
        assign memory_addr_a_w[(i7_w+1)*nbit_read_addr-1:i7_w*nbit_read_addr] = relative_addr_w[nbit_read_addr+nbits_byte_offset-1+i7_w*nbit_addr_w:nbits_byte_offset+i7_w*nbit_addr_w];
    end
  endgenerate
  
  generate
  genvar i14;
    for (i14=0; i14<max_n_writes; i14=i14+1)
    begin : L14
      assign din_value_aggregated[(i14+1)*data_size-1:i14*data_size] = (proxy_sel_STORE[i14] && proxy_in4w[i14]) ? proxy_in1[(i14+1)*BITSIZE_proxy_in1-1:i14*BITSIZE_proxy_in1] : in1[(i14+1)*BITSIZE_in1-1:i14*BITSIZE_in1];
    end
  endgenerate
  
  generate
  genvar i11;
    for (i11=0; i11<max_n_reads; i11=i11+1)
    begin : asynchronous_read
      assign dout_a[data_size*i11+:data_size] = memory[memory_addr_a_r[nbit_read_addr*i11+:nbit_read_addr]];
    end
  endgenerate
  
  generate if(READ_ONLY_MEMORY==0)
    always @(posedge clock)
    begin
      for (index2=0; index2<max_n_writes; index2=index2+1)
      begin
        if(bram_write[index2])
          memory[memory_addr_a_w[nbit_read_addr*index2+:nbit_read_addr]] <= din_value_aggregated[data_size*index2+:data_size];
      end
    end
  endgenerate
  
  generate
  genvar i21;
    for (i21=0; i21<max_n_writes; i21=i21+1)
    begin : L21
        assign bram_write[i21] = (sel_STORE[i21] && in4w[i21]) || (proxy_sel_STORE[i21] && proxy_in4w[i21]);
    end
  endgenerate
  
  generate
  genvar i20;
    for (i20=0; i20<max_n_reads; i20=i20+1)
    begin : L20
      assign out1[(i20+1)*BITSIZE_out1-1:i20*BITSIZE_out1] = dout_a[(i20+1)*data_size-1:i20*data_size];
      assign proxy_out1[(i20+1)*BITSIZE_proxy_out1-1:i20*BITSIZE_proxy_out1] = dout_a[(i20+1)*data_size-1:i20*data_size];
    end
  endgenerate
  assign Sout_Rdata_ram =Sin_Rdata_ram;
  assign Sout_DataRdy = Sin_DataRdy;

endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module UUdata_converter_FU(in1,
  out1);
  parameter BITSIZE_in1=1,
    BITSIZE_out1=1;
  // IN
  input [BITSIZE_in1-1:0] in1;
  // OUT
  output [BITSIZE_out1-1:0] out1;
  generate
  if (BITSIZE_out1 <= BITSIZE_in1)
  begin
    assign out1 = in1[BITSIZE_out1-1:0];
  end
  else
  begin
    assign out1 = {{(BITSIZE_out1-BITSIZE_in1){1'b0}},in1};
  end
  endgenerate
endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module addr_expr_FU(in1,
  out1);
  parameter BITSIZE_in1=1,
    BITSIZE_out1=1;
  // IN
  input [BITSIZE_in1-1:0] in1;
  // OUT
  output [BITSIZE_out1-1:0] out1;
  assign out1 = in1;
endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module ASSIGN_UNSIGNED_FU(in1,
  out1);
  parameter BITSIZE_in1=1,
    BITSIZE_out1=1;
  // IN
  input [BITSIZE_in1-1:0] in1;
  // OUT
  output [BITSIZE_out1-1:0] out1;
  assign out1 = in1;
endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2020-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module ui_extract_bit_expr_FU(in1,
  in2,
  out1);
  parameter BITSIZE_in1=1,
    BITSIZE_in2=1;
  // IN
  input [BITSIZE_in1-1:0] in1;
  input [BITSIZE_in2-1:0] in2;
  // OUT
  output out1;
  assign out1 = (in1 >> in2)&1;
endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2016-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module lut_expr_FU(in1,
  in2,
  in3,
  in4,
  in5,
  in6,
  in7,
  in8,
  in9,
  out1);
  parameter BITSIZE_in1=1,
    BITSIZE_out1=1;
  // IN
  input [BITSIZE_in1-1:0] in1;
  input in2;
  input in3;
  input in4;
  input in5;
  input in6;
  input in7;
  input in8;
  input in9;
  // OUT
  output [BITSIZE_out1-1:0] out1;
  reg[7:0] cleaned_in0;
  wire [7:0] in0;
  wire[BITSIZE_in1-1:0] shifted_s;
  assign in0 = {in9, in8, in7, in6, in5, in4, in3, in2};
  generate
    genvar i0;
    for (i0=0; i0<8; i0=i0+1)
    begin : L0
          always @(*)
          begin
             if (in0[i0] == 1'b1)
                cleaned_in0[i0] = 1'b1;
             else
                cleaned_in0[i0] = 1'b0;
          end
    end
  endgenerate
  assign shifted_s = in1 >> cleaned_in0;
  assign out1[0] = shifted_s[0];
  generate
     if(BITSIZE_out1 > 1)
       assign out1[BITSIZE_out1-1:1] = 0;
  endgenerate

endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module ui_bit_and_expr_FU(in1,
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
  assign out1 = in1 & in2;
endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module ui_bit_ior_expr_FU(in1,
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
  assign out1 = in1 | in2;
endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module ui_bit_xor_expr_FU(in1,
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
  assign out1 = in1 ^ in2;
endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module ui_lshift_expr_FU(in1,
  in2,
  out1);
  parameter BITSIZE_in1=1,
    BITSIZE_in2=1,
    BITSIZE_out1=1,
    PRECISION=1;
  // IN
  input [BITSIZE_in1-1:0] in1;
  input [BITSIZE_in2-1:0] in2;
  // OUT
  output [BITSIZE_out1-1:0] out1;
  `ifndef _SIM_HAVE_CLOG2
    function integer log2;
       input integer value;
       integer temp_value;
      begin
        temp_value = value-1;
        for (log2=0; temp_value>0; log2=log2+1)
          temp_value = temp_value>>1;
      end
    endfunction
  `endif
  `ifdef _SIM_HAVE_CLOG2
    localparam arg2_bitsize = $clog2(PRECISION);
  `else
    localparam arg2_bitsize = log2(PRECISION);
  `endif
  generate
    if(BITSIZE_in2 > arg2_bitsize)
      assign out1 = in1 << in2[arg2_bitsize-1:0];
    else
      assign out1 = in1 << in2;
  endgenerate
endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module ui_lt_expr_FU(in1,
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
  assign out1 = in1 < in2;
endmodule

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

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module ui_mult_expr_FU(clock,
  in1,
  in2,
  out1);
  parameter BITSIZE_in1=1,
    BITSIZE_in2=1,
    BITSIZE_out1=1,
    PIPE_PARAMETER=0;
  // IN
  input clock;
  input [BITSIZE_in1-1:0] in1;
  input [BITSIZE_in2-1:0] in2;
  // OUT
  output [BITSIZE_out1-1:0] out1;
  
  generate
    if(PIPE_PARAMETER==1)
    begin
      reg [BITSIZE_out1-1:0] out1_reg;
      assign out1 = out1_reg;
      always @(posedge clock)
      begin
        out1_reg <= in1 * in2;
      end
    end
    else if(PIPE_PARAMETER>1)
    begin
      reg [BITSIZE_in1-1:0] in1_in;
      reg [BITSIZE_in2-1:0] in2_in;
      wire [BITSIZE_out1-1:0] mult_res;
      reg [BITSIZE_out1-1:0] mul [PIPE_PARAMETER-2:0];
      integer i;
      assign mult_res = in1_in * in2_in;
      always @(posedge clock)
      begin
        in1_in <= in1;
        in2_in <= in2;
        mul[PIPE_PARAMETER-2] <= mult_res;
        for (i=0; i<PIPE_PARAMETER-2; i=i+1)
          mul[i] <= mul[i+1];
      end
      assign out1 = mul[0];
    end
    else
    begin
      assign out1 = in1 * in2;
    end
  endgenerate

endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module ui_negate_expr_FU(in1,
  out1);
  parameter BITSIZE_in1=1,
    BITSIZE_out1=1;
  // IN
  input [BITSIZE_in1-1:0] in1;
  // OUT
  output [BITSIZE_out1-1:0] out1;
  assign out1 = -in1;
endmodule

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

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module ui_pointer_plus_expr_FU(in1,
  in2,
  out1);
  parameter BITSIZE_in1=1,
    BITSIZE_in2=1,
    BITSIZE_out1=1,
    LSB_PARAMETER=-1;
  // IN
  input [BITSIZE_in1-1:0] in1;
  input [BITSIZE_in2-1:0] in2;
  // OUT
  output [BITSIZE_out1-1:0] out1;
  wire [BITSIZE_out1-1:0] in1_tmp;
  wire [BITSIZE_out1-1:0] in2_tmp;
  assign in1_tmp = in1;
  assign in2_tmp = in2;generate if (BITSIZE_out1 > LSB_PARAMETER) assign out1[BITSIZE_out1-1:LSB_PARAMETER] = (in1_tmp[BITSIZE_out1-1:LSB_PARAMETER] + in2_tmp[BITSIZE_out1-1:LSB_PARAMETER]); else assign out1 = 0; endgenerate
  generate if (LSB_PARAMETER != 0 && BITSIZE_out1 > LSB_PARAMETER) assign out1[LSB_PARAMETER-1:0] = 0; endgenerate
endmodule

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Fabrizio Ferrandi <fabrizio.ferrandi@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module ui_rshift_expr_FU(in1,
  in2,
  out1);
  parameter BITSIZE_in1=1,
    BITSIZE_in2=1,
    BITSIZE_out1=1,
    PRECISION=1;
  // IN
  input [BITSIZE_in1-1:0] in1;
  input [BITSIZE_in2-1:0] in2;
  // OUT
  output [BITSIZE_out1-1:0] out1;
  `ifndef _SIM_HAVE_CLOG2
    function integer log2;
       input integer value;
       integer temp_value;
      begin
        temp_value = value-1;
        for (log2=0; temp_value>0; log2=log2+1)
          temp_value = temp_value>>1;
      end
    endfunction
  `endif
  `ifdef _SIM_HAVE_CLOG2
    localparam arg2_bitsize = $clog2(PRECISION);
  `else
    localparam arg2_bitsize = log2(PRECISION);
  `endif
  generate
    if(BITSIZE_in2 > arg2_bitsize)
      assign out1 = in1 >> (in2[arg2_bitsize-1:0]);
    else
      assign out1 = in1 >> in2;
  endgenerate

endmodule

// Datapath RTL description for __udivsi3
// This component has been derived from the input source code and so it does not fall under the copyright of PandA framework, but it follows the input source code copyright, and may be aggregated with components of the BAMBU/PANDA IP LIBRARY.
// Author(s): Component automatically generated by bambu
// License: THIS COMPONENT IS PROVIDED "AS IS" AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
`timescale 1ns / 1ps
module datapath___udivsi3(clock,
  reset,
  in_port_u,
  in_port_v,
  return_port,
  fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_LOAD,
  fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_STORE,
  wrenable_reg_0,
  wrenable_reg_1,
  wrenable_reg_10,
  wrenable_reg_11,
  wrenable_reg_12,
  wrenable_reg_13,
  wrenable_reg_14,
  wrenable_reg_15,
  wrenable_reg_16,
  wrenable_reg_2,
  wrenable_reg_3,
  wrenable_reg_4,
  wrenable_reg_5,
  wrenable_reg_6,
  wrenable_reg_7,
  wrenable_reg_8,
  wrenable_reg_9);
  parameter MEM_var_7052_5825=1024;
  // IN
  input clock;
  input reset;
  input [31:0] in_port_u;
  input [31:0] in_port_v;
  input fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_LOAD;
  input fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_STORE;
  input wrenable_reg_0;
  input wrenable_reg_1;
  input wrenable_reg_10;
  input wrenable_reg_11;
  input wrenable_reg_12;
  input wrenable_reg_13;
  input wrenable_reg_14;
  input wrenable_reg_15;
  input wrenable_reg_16;
  input wrenable_reg_2;
  input wrenable_reg_3;
  input wrenable_reg_4;
  input wrenable_reg_5;
  input wrenable_reg_6;
  input wrenable_reg_7;
  input wrenable_reg_8;
  input wrenable_reg_9;
  // OUT
  output [31:0] return_port;
  // Component and signal declarations
  wire null_out_signal_array_7052_0_Sout_DataRdy_0;
  wire null_out_signal_array_7052_0_Sout_DataRdy_1;
  wire [7:0] null_out_signal_array_7052_0_Sout_Rdata_ram_0;
  wire [7:0] null_out_signal_array_7052_0_Sout_Rdata_ram_1;
  wire [7:0] null_out_signal_array_7052_0_out1_1;
  wire [7:0] null_out_signal_array_7052_0_proxy_out1_0;
  wire [7:0] null_out_signal_array_7052_0_proxy_out1_1;
  wire [7:0] out_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_array_7052_0;
  wire [31:0] out_ASSIGN_UNSIGNED_FU_6_i0_fu___udivsi3_5825_428790;
  wire out_UUdata_converter_FU_33_i0_fu___udivsi3_5825_428784;
  wire [31:0] out_UUdata_converter_FU_3_i0_fu___udivsi3_5825_7877;
  wire out_UUdata_converter_FU_4_i0_fu___udivsi3_5825_428728;
  wire out_UUdata_converter_FU_55_i0_fu___udivsi3_5825_428707;
  wire out_UUdata_converter_FU_59_i0_fu___udivsi3_5825_7945;
  wire out_UUdata_converter_FU_60_i0_fu___udivsi3_5825_7946;
  wire out_UUdata_converter_FU_61_i0_fu___udivsi3_5825_428718;
  wire [7:0] out_UUdata_converter_FU_62_i0_fu___udivsi3_5825_7958;
  wire [31:0] out_UUdata_converter_FU_63_i0_fu___udivsi3_5825_7965;
  wire [31:0] out_UUdata_converter_FU_64_i0_fu___udivsi3_5825_7966;
  wire [31:0] out_UUdata_converter_FU_65_i0_fu___udivsi3_5825_7969;
  wire [31:0] out_UUdata_converter_FU_66_i0_fu___udivsi3_5825_7972;
  wire [31:0] out_UUdata_converter_FU_67_i0_fu___udivsi3_5825_7973;
  wire [31:0] out_UUdata_converter_FU_68_i0_fu___udivsi3_5825_7976;
  wire [31:0] out_UUdata_converter_FU_69_i0_fu___udivsi3_5825_7979;
  wire [31:0] out_UUdata_converter_FU_70_i0_fu___udivsi3_5825_7982;
  wire [31:0] out_UUdata_converter_FU_7_i0_fu___udivsi3_5825_7978;
  wire [10:0] out_addr_expr_FU_5_i0_fu___udivsi3_5825_428532;
  wire out_const_0;
  wire [4:0] out_const_1;
  wire [31:0] out_const_10;
  wire [21:0] out_const_11;
  wire [53:0] out_const_12;
  wire [3:0] out_const_13;
  wire [4:0] out_const_14;
  wire [4:0] out_const_15;
  wire [2:0] out_const_16;
  wire [3:0] out_const_17;
  wire [4:0] out_const_18;
  wire [54:0] out_const_19;
  wire out_const_2;
  wire [4:0] out_const_20;
  wire [31:0] out_const_21;
  wire [3:0] out_const_22;
  wire [4:0] out_const_23;
  wire [4:0] out_const_24;
  wire [10:0] out_const_25;
  wire [1:0] out_const_26;
  wire [2:0] out_const_27;
  wire [3:0] out_const_28;
  wire [4:0] out_const_29;
  wire [1:0] out_const_3;
  wire [4:0] out_const_30;
  wire [3:0] out_const_31;
  wire [4:0] out_const_32;
  wire [4:0] out_const_33;
  wire [2:0] out_const_34;
  wire [3:0] out_const_35;
  wire [4:0] out_const_36;
  wire [4:0] out_const_37;
  wire [3:0] out_const_38;
  wire [4:0] out_const_39;
  wire [2:0] out_const_4;
  wire [4:0] out_const_40;
  wire [6:0] out_const_41;
  wire [7:0] out_const_42;
  wire [63:0] out_const_43;
  wire [48:0] out_const_44;
  wire [3:0] out_const_5;
  wire [4:0] out_const_6;
  wire [5:0] out_const_7;
  wire [16:0] out_const_8;
  wire [28:0] out_const_9;
  wire [3:0] out_conv_out_const_1_5_4;
  wire [31:0] out_conv_out_const_25_11_32;
  wire out_lut_expr_FU_24_i0_fu___udivsi3_5825_430386;
  wire out_lut_expr_FU_25_i0_fu___udivsi3_5825_430390;
  wire out_lut_expr_FU_26_i0_fu___udivsi3_5825_430394;
  wire out_lut_expr_FU_27_i0_fu___udivsi3_5825_430397;
  wire out_lut_expr_FU_28_i0_fu___udivsi3_5825_428748;
  wire out_lut_expr_FU_29_i0_fu___udivsi3_5825_428752;
  wire out_lut_expr_FU_30_i0_fu___udivsi3_5825_428773;
  wire out_lut_expr_FU_31_i0_fu___udivsi3_5825_430403;
  wire out_lut_expr_FU_32_i0_fu___udivsi3_5825_428904;
  wire out_lut_expr_FU_42_i0_fu___udivsi3_5825_430407;
  wire out_lut_expr_FU_43_i0_fu___udivsi3_5825_430410;
  wire out_lut_expr_FU_44_i0_fu___udivsi3_5825_430413;
  wire out_lut_expr_FU_45_i0_fu___udivsi3_5825_428493;
  wire out_lut_expr_FU_50_i0_fu___udivsi3_5825_430419;
  wire out_lut_expr_FU_51_i0_fu___udivsi3_5825_430422;
  wire out_lut_expr_FU_52_i0_fu___udivsi3_5825_430425;
  wire out_lut_expr_FU_53_i0_fu___udivsi3_5825_430428;
  wire out_lut_expr_FU_54_i0_fu___udivsi3_5825_428499;
  wire out_lut_expr_FU_58_i0_fu___udivsi3_5825_428504;
  wire [7:0] out_reg_0_reg_0;
  wire [31:0] out_reg_10_reg_10;
  wire [31:0] out_reg_11_reg_11;
  wire [31:0] out_reg_12_reg_12;
  wire [31:0] out_reg_13_reg_13;
  wire [31:0] out_reg_14_reg_14;
  wire [31:0] out_reg_15_reg_15;
  wire [31:0] out_reg_16_reg_16;
  wire [4:0] out_reg_1_reg_1;
  wire [31:0] out_reg_2_reg_2;
  wire [31:0] out_reg_3_reg_3;
  wire [10:0] out_reg_4_reg_4;
  wire [31:0] out_reg_5_reg_5;
  wire [31:0] out_reg_6_reg_6;
  wire [31:0] out_reg_7_reg_7;
  wire [31:0] out_reg_8_reg_8;
  wire [31:0] out_reg_9_reg_9;
  wire [6:0] out_ui_bit_and_expr_FU_8_0_8_72_i0_fu___udivsi3_5825_7880;
  wire [6:0] out_ui_bit_and_expr_FU_8_0_8_72_i1_fu___udivsi3_5825_7906;
  wire [6:0] out_ui_bit_and_expr_FU_8_0_8_72_i2_fu___udivsi3_5825_7916;
  wire [7:0] out_ui_bit_and_expr_FU_8_0_8_73_i0_fu___udivsi3_5825_7955;
  wire [2:0] out_ui_bit_and_expr_FU_8_8_8_74_i0_fu___udivsi3_5825_7944;
  wire [4:0] out_ui_bit_ior_expr_FU_0_8_8_75_i0_fu___udivsi3_5825_7949;
  wire [4:0] out_ui_bit_ior_expr_FU_0_8_8_76_i0_fu___udivsi3_5825_7950;
  wire [4:0] out_ui_bit_ior_expr_FU_0_8_8_77_i0_fu___udivsi3_5825_7951;
  wire [4:0] out_ui_bit_ior_expr_FU_0_8_8_78_i0_fu___udivsi3_5825_7952;
  wire [31:0] out_ui_bit_ior_expr_FU_32_0_32_79_i0_fu___udivsi3_5825_7960;
  wire [4:0] out_ui_bit_xor_expr_FU_8_0_8_80_i0_fu___udivsi3_5825_7961;
  wire [6:0] out_ui_cond_expr_FU_8_8_8_8_81_i0_fu___udivsi3_5825_428770;
  wire [6:0] out_ui_cond_expr_FU_8_8_8_8_81_i1_fu___udivsi3_5825_428782;
  wire [6:0] out_ui_cond_expr_FU_8_8_8_8_81_i2_fu___udivsi3_5825_428788;
  wire [2:0] out_ui_cond_expr_FU_8_8_8_8_81_i3_fu___udivsi3_5825_7939;
  wire [2:0] out_ui_cond_expr_FU_8_8_8_8_81_i4_fu___udivsi3_5825_7943;
  wire [1:0] out_ui_cond_expr_FU_8_8_8_8_81_i5_fu___udivsi3_5825_7988;
  wire [1:0] out_ui_cond_expr_FU_8_8_8_8_81_i6_fu___udivsi3_5825_7989;
  wire out_ui_extract_bit_expr_FU_10_i0_fu___udivsi3_5825_429407;
  wire out_ui_extract_bit_expr_FU_11_i0_fu___udivsi3_5825_429281;
  wire out_ui_extract_bit_expr_FU_12_i0_fu___udivsi3_5825_429411;
  wire out_ui_extract_bit_expr_FU_13_i0_fu___udivsi3_5825_429290;
  wire out_ui_extract_bit_expr_FU_14_i0_fu___udivsi3_5825_429415;
  wire out_ui_extract_bit_expr_FU_15_i0_fu___udivsi3_5825_429299;
  wire out_ui_extract_bit_expr_FU_16_i0_fu___udivsi3_5825_429419;
  wire out_ui_extract_bit_expr_FU_17_i0_fu___udivsi3_5825_429307;
  wire out_ui_extract_bit_expr_FU_18_i0_fu___udivsi3_5825_429423;
  wire out_ui_extract_bit_expr_FU_19_i0_fu___udivsi3_5825_429315;
  wire out_ui_extract_bit_expr_FU_20_i0_fu___udivsi3_5825_429427;
  wire out_ui_extract_bit_expr_FU_21_i0_fu___udivsi3_5825_429323;
  wire out_ui_extract_bit_expr_FU_22_i0_fu___udivsi3_5825_429431;
  wire out_ui_extract_bit_expr_FU_23_i0_fu___udivsi3_5825_429331;
  wire out_ui_extract_bit_expr_FU_34_i0_fu___udivsi3_5825_429504;
  wire out_ui_extract_bit_expr_FU_35_i0_fu___udivsi3_5825_429508;
  wire out_ui_extract_bit_expr_FU_36_i0_fu___udivsi3_5825_429512;
  wire out_ui_extract_bit_expr_FU_37_i0_fu___udivsi3_5825_429516;
  wire out_ui_extract_bit_expr_FU_38_i0_fu___udivsi3_5825_429520;
  wire out_ui_extract_bit_expr_FU_39_i0_fu___udivsi3_5825_429524;
  wire out_ui_extract_bit_expr_FU_40_i0_fu___udivsi3_5825_429528;
  wire out_ui_extract_bit_expr_FU_41_i0_fu___udivsi3_5825_429532;
  wire out_ui_extract_bit_expr_FU_46_i0_fu___udivsi3_5825_430228;
  wire out_ui_extract_bit_expr_FU_47_i0_fu___udivsi3_5825_430294;
  wire out_ui_extract_bit_expr_FU_48_i0_fu___udivsi3_5825_430245;
  wire out_ui_extract_bit_expr_FU_49_i0_fu___udivsi3_5825_430306;
  wire out_ui_extract_bit_expr_FU_56_i0_fu___udivsi3_5825_429260;
  wire out_ui_extract_bit_expr_FU_57_i0_fu___udivsi3_5825_429264;
  wire out_ui_extract_bit_expr_FU_8_i0_fu___udivsi3_5825_429403;
  wire out_ui_extract_bit_expr_FU_9_i0_fu___udivsi3_5825_429272;
  wire [31:0] out_ui_lshift_expr_FU_32_0_32_82_i0_fu___udivsi3_5825_428711;
  wire [31:0] out_ui_lshift_expr_FU_32_0_32_82_i1_fu___udivsi3_5825_428721;
  wire [31:0] out_ui_lshift_expr_FU_32_0_32_82_i2_fu___udivsi3_5825_428731;
  wire [30:0] out_ui_lshift_expr_FU_32_0_32_83_i0_fu___udivsi3_5825_7959;
  wire [30:0] out_ui_lshift_expr_FU_32_32_32_84_i0_fu___udivsi3_5825_7953;
  wire [7:0] out_ui_lshift_expr_FU_8_0_8_85_i0_fu___udivsi3_5825_428582;
  wire [7:0] out_ui_lshift_expr_FU_8_0_8_85_i1_fu___udivsi3_5825_428602;
  wire [3:0] out_ui_lshift_expr_FU_8_0_8_85_i2_fu___udivsi3_5825_428629;
  wire [3:0] out_ui_lshift_expr_FU_8_0_8_85_i3_fu___udivsi3_5825_428659;
  wire [7:0] out_ui_lshift_expr_FU_8_0_8_85_i4_fu___udivsi3_5825_428800;
  wire [7:0] out_ui_lshift_expr_FU_8_0_8_85_i5_fu___udivsi3_5825_430362;
  wire [7:0] out_ui_lshift_expr_FU_8_0_8_85_i6_fu___udivsi3_5825_430372;
  wire [7:0] out_ui_lshift_expr_FU_8_0_8_85_i7_fu___udivsi3_5825_430382;
  wire [1:0] out_ui_lshift_expr_FU_8_0_8_86_i0_fu___udivsi3_5825_428651;
  wire [2:0] out_ui_lshift_expr_FU_8_0_8_87_i0_fu___udivsi3_5825_428681;
  wire [4:0] out_ui_lshift_expr_FU_8_0_8_88_i0_fu___udivsi3_5825_428690;
  wire [3:0] out_ui_lshift_expr_FU_8_0_8_89_i0_fu___udivsi3_5825_428812;
  wire out_ui_lt_expr_FU_32_0_32_90_i0_fu___udivsi3_5825_428462;
  wire out_ui_lt_expr_FU_32_0_32_91_i0_fu___udivsi3_5825_428489;
  wire out_ui_lt_expr_FU_32_32_32_92_i0_fu___udivsi3_5825_428518;
  wire out_ui_lt_expr_FU_32_32_32_92_i1_fu___udivsi3_5825_428521;
  wire [31:0] out_ui_minus_expr_FU_32_32_32_93_i0_fu___udivsi3_5825_7984;
  wire [31:0] out_ui_minus_expr_FU_32_32_32_93_i1_fu___udivsi3_5825_7986;
  wire [31:0] out_ui_mult_expr_FU_32_32_32_0_94_i0_fu___udivsi3_5825_7964;
  wire [63:0] out_ui_mult_expr_FU_32_32_32_0_94_i1_fu___udivsi3_5825_7967;
  wire [31:0] out_ui_mult_expr_FU_32_32_32_0_94_i2_fu___udivsi3_5825_7971;
  wire [63:0] out_ui_mult_expr_FU_32_32_32_0_94_i3_fu___udivsi3_5825_7974;
  wire [63:0] out_ui_mult_expr_FU_32_32_32_0_94_i4_fu___udivsi3_5825_7980;
  wire [31:0] out_ui_mult_expr_FU_32_32_32_0_94_i5_fu___udivsi3_5825_7983;
  wire [31:0] out_ui_negate_expr_FU_32_32_95_i0_fu___udivsi3_5825_7963;
  wire [31:0] out_ui_plus_expr_FU_32_32_32_96_i0_fu___udivsi3_5825_7970;
  wire [31:0] out_ui_plus_expr_FU_32_32_32_96_i1_fu___udivsi3_5825_7977;
  wire [31:0] out_ui_plus_expr_FU_32_32_32_96_i2_fu___udivsi3_5825_7990;
  wire [10:0] out_ui_pointer_plus_expr_FU_16_16_16_97_i0_fu___udivsi3_5825_7956;
  wire [7:0] out_ui_rshift_expr_FU_32_0_32_100_i0_fu___udivsi3_5825_7879;
  wire [7:0] out_ui_rshift_expr_FU_32_0_32_101_i0_fu___udivsi3_5825_7881;
  wire [7:0] out_ui_rshift_expr_FU_32_0_32_102_i0_fu___udivsi3_5825_7915;
  wire [7:0] out_ui_rshift_expr_FU_32_0_32_103_i0_fu___udivsi3_5825_7954;
  wire [6:0] out_ui_rshift_expr_FU_32_0_32_98_i0_fu___udivsi3_5825_428576;
  wire [0:0] out_ui_rshift_expr_FU_32_0_32_99_i0_fu___udivsi3_5825_428714;
  wire [0:0] out_ui_rshift_expr_FU_32_0_32_99_i1_fu___udivsi3_5825_428724;
  wire [0:0] out_ui_rshift_expr_FU_32_0_32_99_i2_fu___udivsi3_5825_428734;
  wire [31:0] out_ui_rshift_expr_FU_32_32_32_104_i0_fu___udivsi3_5825_7962;
  wire [31:0] out_ui_rshift_expr_FU_64_0_64_105_i0_fu___udivsi3_5825_7968;
  wire [31:0] out_ui_rshift_expr_FU_64_0_64_105_i1_fu___udivsi3_5825_7975;
  wire [31:0] out_ui_rshift_expr_FU_64_0_64_105_i2_fu___udivsi3_5825_7981;
  wire [6:0] out_ui_rshift_expr_FU_8_0_8_106_i0_fu___udivsi3_5825_428597;
  wire [6:0] out_ui_rshift_expr_FU_8_0_8_106_i10_fu___udivsi3_5825_430375;
  wire [6:0] out_ui_rshift_expr_FU_8_0_8_106_i11_fu___udivsi3_5825_430378;
  wire [2:0] out_ui_rshift_expr_FU_8_0_8_106_i1_fu___udivsi3_5825_428621;
  wire [2:0] out_ui_rshift_expr_FU_8_0_8_106_i2_fu___udivsi3_5825_428624;
  wire [2:0] out_ui_rshift_expr_FU_8_0_8_106_i3_fu___udivsi3_5825_428662;
  wire [2:0] out_ui_rshift_expr_FU_8_0_8_106_i4_fu___udivsi3_5825_428665;
  wire [6:0] out_ui_rshift_expr_FU_8_0_8_106_i5_fu___udivsi3_5825_428793;
  wire [6:0] out_ui_rshift_expr_FU_8_0_8_106_i6_fu___udivsi3_5825_428796;
  wire [6:0] out_ui_rshift_expr_FU_8_0_8_106_i7_fu___udivsi3_5825_430358;
  wire [6:0] out_ui_rshift_expr_FU_8_0_8_106_i8_fu___udivsi3_5825_430365;
  wire [6:0] out_ui_rshift_expr_FU_8_0_8_106_i9_fu___udivsi3_5825_430368;
  wire [3:0] out_ui_rshift_expr_FU_8_0_8_107_i0_fu___udivsi3_5825_7938;
  
  ARRAY_1D_STD_DISTRAM_NN_SDS #(.BITSIZE_in1(8),
    .PORTSIZE_in1(2),
    .BITSIZE_in2r(11),
    .PORTSIZE_in2r(2),
    .BITSIZE_in2w(11),
    .PORTSIZE_in2w(2),
    .BITSIZE_in3r(4),
    .PORTSIZE_in3r(2),
    .BITSIZE_in3w(4),
    .PORTSIZE_in3w(2),
    .BITSIZE_in4r(1),
    .PORTSIZE_in4r(2),
    .BITSIZE_in4w(1),
    .PORTSIZE_in4w(2),
    .BITSIZE_sel_LOAD(1),
    .PORTSIZE_sel_LOAD(2),
    .BITSIZE_sel_STORE(1),
    .PORTSIZE_sel_STORE(2),
    .BITSIZE_S_oe_ram(1),
    .PORTSIZE_S_oe_ram(2),
    .BITSIZE_S_we_ram(1),
    .PORTSIZE_S_we_ram(2),
    .BITSIZE_out1(8),
    .PORTSIZE_out1(2),
    .BITSIZE_S_addr_ram(11),
    .PORTSIZE_S_addr_ram(2),
    .BITSIZE_S_Wdata_ram(8),
    .PORTSIZE_S_Wdata_ram(2),
    .BITSIZE_Sin_Rdata_ram(8),
    .PORTSIZE_Sin_Rdata_ram(2),
    .BITSIZE_Sout_Rdata_ram(8),
    .PORTSIZE_Sout_Rdata_ram(2),
    .BITSIZE_S_data_ram_size(4),
    .PORTSIZE_S_data_ram_size(2),
    .BITSIZE_Sin_DataRdy(1),
    .PORTSIZE_Sin_DataRdy(2),
    .BITSIZE_Sout_DataRdy(1),
    .PORTSIZE_Sout_DataRdy(2),
    .MEMORY_INIT_file("/home/lennart/.cache/rust-hls/1446bebb41fcdb0a7f9bc63d67e2e28d-lg52t83OPq/array_ref_7052.mem"),
    .n_elements(256),
    .data_size(8),
    .address_space_begin(MEM_var_7052_5825),
    .address_space_rangesize(1024),
    .BUS_PIPELINED(1),
    .PRIVATE_MEMORY(1),
    .READ_ONLY_MEMORY(1),
    .USE_SPARSE_MEMORY(1),
    .ALIGNMENT(8),
    .BITSIZE_proxy_in1(8),
    .PORTSIZE_proxy_in1(2),
    .BITSIZE_proxy_in2r(11),
    .PORTSIZE_proxy_in2r(2),
    .BITSIZE_proxy_in2w(11),
    .PORTSIZE_proxy_in2w(2),
    .BITSIZE_proxy_in3r(4),
    .PORTSIZE_proxy_in3r(2),
    .BITSIZE_proxy_in3w(4),
    .PORTSIZE_proxy_in3w(2),
    .BITSIZE_proxy_in4r(1),
    .PORTSIZE_proxy_in4r(2),
    .BITSIZE_proxy_in4w(1),
    .PORTSIZE_proxy_in4w(2),
    .BITSIZE_proxy_sel_LOAD(1),
    .PORTSIZE_proxy_sel_LOAD(2),
    .BITSIZE_proxy_sel_STORE(1),
    .PORTSIZE_proxy_sel_STORE(2),
    .BITSIZE_proxy_out1(8),
    .PORTSIZE_proxy_out1(2)) array_7052_0 (.out1({null_out_signal_array_7052_0_out1_1,
      out_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_array_7052_0}),
    .Sout_Rdata_ram({null_out_signal_array_7052_0_Sout_Rdata_ram_1,
      null_out_signal_array_7052_0_Sout_Rdata_ram_0}),
    .Sout_DataRdy({null_out_signal_array_7052_0_Sout_DataRdy_1,
      null_out_signal_array_7052_0_Sout_DataRdy_0}),
    .proxy_out1({null_out_signal_array_7052_0_proxy_out1_1,
      null_out_signal_array_7052_0_proxy_out1_0}),
    .clock(clock),
    .reset(reset),
    .in1({8'b00000000,
      8'b00000000}),
    .in2r({11'b00000000000,
      out_ui_pointer_plus_expr_FU_16_16_16_97_i0_fu___udivsi3_5825_7956}),
    .in2w({11'b00000000000,
      11'b00000000000}),
    .in3r({4'b0000,
      out_conv_out_const_1_5_4}),
    .in3w({4'b0000,
      4'b0000}),
    .in4r({1'b0,
      out_const_2}),
    .in4w({1'b0,
      1'b0}),
    .sel_LOAD({1'b0,
      fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_LOAD}),
    .sel_STORE({1'b0,
      fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_STORE}),
    .S_oe_ram({1'b0,
      1'b0}),
    .S_we_ram({1'b0,
      1'b0}),
    .S_addr_ram({11'b00000000000,
      11'b00000000000}),
    .S_Wdata_ram({8'b00000000,
      8'b00000000}),
    .Sin_Rdata_ram({8'b00000000,
      8'b00000000}),
    .S_data_ram_size({4'b0000,
      4'b0000}),
    .Sin_DataRdy({1'b0,
      1'b0}),
    .proxy_in1({8'b00000000,
      8'b00000000}),
    .proxy_in2r({11'b00000000000,
      11'b00000000000}),
    .proxy_in2w({11'b00000000000,
      11'b00000000000}),
    .proxy_in3r({4'b0000,
      4'b0000}),
    .proxy_in3w({4'b0000,
      4'b0000}),
    .proxy_in4r({1'b0,
      1'b0}),
    .proxy_in4w({1'b0,
      1'b0}),
    .proxy_sel_LOAD({1'b0,
      1'b0}),
    .proxy_sel_STORE({1'b0,
      1'b0}));
  constant_value #(.BITSIZE_out1(1),
    .value(1'b0)) const_0 (.out1(out_const_0));
  constant_value #(.BITSIZE_out1(5),
    .value(5'b01000)) const_1 (.out1(out_const_1));
  constant_value #(.BITSIZE_out1(32),
    .value(32'b10000000000000000000000000000000)) const_10 (.out1(out_const_10));
  constant_value #(.BITSIZE_out1(22),
    .value(22'b1000100000010100100111)) const_11 (.out1(out_const_11));
  constant_value #(.BITSIZE_out1(54),
    .value(54'b100010000001010010011100000000000000000000000000000000)) const_12 (.out1(out_const_12));
  constant_value #(.BITSIZE_out1(4),
    .value(4'b1001)) const_13 (.out1(out_const_13));
  constant_value #(.BITSIZE_out1(5),
    .value(5'b10010)) const_14 (.out1(out_const_14));
  constant_value #(.BITSIZE_out1(5),
    .value(5'b10011)) const_15 (.out1(out_const_15));
  constant_value #(.BITSIZE_out1(3),
    .value(3'b101)) const_16 (.out1(out_const_16));
  constant_value #(.BITSIZE_out1(4),
    .value(4'b1010)) const_17 (.out1(out_const_17));
  constant_value #(.BITSIZE_out1(5),
    .value(5'b10100)) const_18 (.out1(out_const_18));
  constant_value #(.BITSIZE_out1(55),
    .value(55'b1010000000000110101001100000000000000000000000000000000)) const_19 (.out1(out_const_19));
  constant_value #(.BITSIZE_out1(1),
    .value(1'b1)) const_2 (.out1(out_const_2));
  constant_value #(.BITSIZE_out1(5),
    .value(5'b10101)) const_20 (.out1(out_const_20));
  constant_value #(.BITSIZE_out1(32),
    .value(32'b10101111101000111010110010100000)) const_21 (.out1(out_const_21));
  constant_value #(.BITSIZE_out1(4),
    .value(4'b1011)) const_22 (.out1(out_const_22));
  constant_value #(.BITSIZE_out1(5),
    .value(5'b10110)) const_23 (.out1(out_const_23));
  constant_value #(.BITSIZE_out1(5),
    .value(5'b10111)) const_24 (.out1(out_const_24));
  constant_value #(.BITSIZE_out1(11),
    .value(MEM_var_7052_5825)) const_25 (.out1(out_const_25));
  constant_value #(.BITSIZE_out1(2),
    .value(2'b11)) const_26 (.out1(out_const_26));
  constant_value #(.BITSIZE_out1(3),
    .value(3'b110)) const_27 (.out1(out_const_27));
  constant_value #(.BITSIZE_out1(4),
    .value(4'b1100)) const_28 (.out1(out_const_28));
  constant_value #(.BITSIZE_out1(5),
    .value(5'b11000)) const_29 (.out1(out_const_29));
  constant_value #(.BITSIZE_out1(2),
    .value(2'b10)) const_3 (.out1(out_const_3));
  constant_value #(.BITSIZE_out1(5),
    .value(5'b11001)) const_30 (.out1(out_const_30));
  constant_value #(.BITSIZE_out1(4),
    .value(4'b1101)) const_31 (.out1(out_const_31));
  constant_value #(.BITSIZE_out1(5),
    .value(5'b11010)) const_32 (.out1(out_const_32));
  constant_value #(.BITSIZE_out1(5),
    .value(5'b11011)) const_33 (.out1(out_const_33));
  constant_value #(.BITSIZE_out1(3),
    .value(3'b111)) const_34 (.out1(out_const_34));
  constant_value #(.BITSIZE_out1(4),
    .value(4'b1110)) const_35 (.out1(out_const_35));
  constant_value #(.BITSIZE_out1(5),
    .value(5'b11100)) const_36 (.out1(out_const_36));
  constant_value #(.BITSIZE_out1(5),
    .value(5'b11101)) const_37 (.out1(out_const_37));
  constant_value #(.BITSIZE_out1(4),
    .value(4'b1111)) const_38 (.out1(out_const_38));
  constant_value #(.BITSIZE_out1(5),
    .value(5'b11110)) const_39 (.out1(out_const_39));
  constant_value #(.BITSIZE_out1(3),
    .value(3'b100)) const_4 (.out1(out_const_4));
  constant_value #(.BITSIZE_out1(5),
    .value(5'b11111)) const_40 (.out1(out_const_40));
  constant_value #(.BITSIZE_out1(7),
    .value(7'b1111111)) const_41 (.out1(out_const_41));
  constant_value #(.BITSIZE_out1(8),
    .value(8'b11111111)) const_42 (.out1(out_const_42));
  constant_value #(.BITSIZE_out1(64),
    .value(64'b1111111101010101101010100000000011011000110110001101100011011000)) const_43 (.out1(out_const_43));
  constant_value #(.BITSIZE_out1(49),
    .value(49'b1111111111111111100000000000000010000000000000000)) const_44 (.out1(out_const_44));
  constant_value #(.BITSIZE_out1(4),
    .value(4'b1000)) const_5 (.out1(out_const_5));
  constant_value #(.BITSIZE_out1(5),
    .value(5'b10000)) const_6 (.out1(out_const_6));
  constant_value #(.BITSIZE_out1(6),
    .value(6'b100000)) const_7 (.out1(out_const_7));
  constant_value #(.BITSIZE_out1(17),
    .value(17'b10000000000000000)) const_8 (.out1(out_const_8));
  constant_value #(.BITSIZE_out1(29),
    .value(29'b10000000000000000000000000000)) const_9 (.out1(out_const_9));
  UUdata_converter_FU #(.BITSIZE_in1(5),
    .BITSIZE_out1(4)) conv_out_const_1_5_4 (.out1(out_conv_out_const_1_5_4),
    .in1(out_const_1));
  UUdata_converter_FU #(.BITSIZE_in1(11),
    .BITSIZE_out1(32)) conv_out_const_25_11_32 (.out1(out_conv_out_const_25_11_32),
    .in1(out_const_25));
  ui_lt_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(17),
    .BITSIZE_out1(1)) fu___udivsi3_5825_428462 (.out1(out_ui_lt_expr_FU_32_0_32_90_i0_fu___udivsi3_5825_428462),
    .in1(in_port_v),
    .in2(out_const_8));
  ui_lt_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(29),
    .BITSIZE_out1(1)) fu___udivsi3_5825_428489 (.out1(out_ui_lt_expr_FU_32_0_32_91_i0_fu___udivsi3_5825_428489),
    .in1(in_port_v),
    .in2(out_const_9));
  lut_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_out1(1)) fu___udivsi3_5825_428493 (.out1(out_lut_expr_FU_45_i0_fu___udivsi3_5825_428493),
    .in1(out_const_21),
    .in2(out_ui_lt_expr_FU_32_0_32_91_i0_fu___udivsi3_5825_428489),
    .in3(out_lut_expr_FU_28_i0_fu___udivsi3_5825_428748),
    .in4(out_lut_expr_FU_31_i0_fu___udivsi3_5825_430403),
    .in5(out_lut_expr_FU_42_i0_fu___udivsi3_5825_430407),
    .in6(out_lut_expr_FU_44_i0_fu___udivsi3_5825_430413),
    .in7(1'b0),
    .in8(1'b0),
    .in9(1'b0));
  lut_expr_FU #(.BITSIZE_in1(22),
    .BITSIZE_out1(1)) fu___udivsi3_5825_428499 (.out1(out_lut_expr_FU_54_i0_fu___udivsi3_5825_428499),
    .in1(out_const_11),
    .in2(out_lut_expr_FU_45_i0_fu___udivsi3_5825_428493),
    .in3(out_lut_expr_FU_50_i0_fu___udivsi3_5825_430419),
    .in4(out_lut_expr_FU_51_i0_fu___udivsi3_5825_430422),
    .in5(out_lut_expr_FU_52_i0_fu___udivsi3_5825_430425),
    .in6(out_lut_expr_FU_53_i0_fu___udivsi3_5825_430428),
    .in7(1'b0),
    .in8(1'b0),
    .in9(1'b0));
  lut_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_out1(1)) fu___udivsi3_5825_428504 (.out1(out_lut_expr_FU_58_i0_fu___udivsi3_5825_428504),
    .in1(out_const_2),
    .in2(out_ui_extract_bit_expr_FU_56_i0_fu___udivsi3_5825_429260),
    .in3(out_ui_extract_bit_expr_FU_57_i0_fu___udivsi3_5825_429264),
    .in4(1'b0),
    .in5(1'b0),
    .in6(1'b0),
    .in7(1'b0),
    .in8(1'b0),
    .in9(1'b0));
  ui_lt_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(32),
    .BITSIZE_out1(1)) fu___udivsi3_5825_428518 (.out1(out_ui_lt_expr_FU_32_32_32_92_i0_fu___udivsi3_5825_428518),
    .in1(out_ui_minus_expr_FU_32_32_32_93_i0_fu___udivsi3_5825_7984),
    .in2(in_port_v));
  ui_lt_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(32),
    .BITSIZE_out1(1)) fu___udivsi3_5825_428521 (.out1(out_ui_lt_expr_FU_32_32_32_92_i1_fu___udivsi3_5825_428521),
    .in1(out_ui_minus_expr_FU_32_32_32_93_i1_fu___udivsi3_5825_7986),
    .in2(in_port_v));
  addr_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_out1(11)) fu___udivsi3_5825_428532 (.out1(out_addr_expr_FU_5_i0_fu___udivsi3_5825_428532),
    .in1(out_conv_out_const_25_11_32));
  ui_rshift_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(1),
    .BITSIZE_out1(7),
    .PRECISION(64)) fu___udivsi3_5825_428576 (.out1(out_ui_rshift_expr_FU_32_0_32_98_i0_fu___udivsi3_5825_428576),
    .in1(out_UUdata_converter_FU_3_i0_fu___udivsi3_5825_7877),
    .in2(out_const_2));
  ui_lshift_expr_FU #(.BITSIZE_in1(7),
    .BITSIZE_in2(1),
    .BITSIZE_out1(8),
    .PRECISION(64)) fu___udivsi3_5825_428582 (.out1(out_ui_lshift_expr_FU_8_0_8_85_i0_fu___udivsi3_5825_428582),
    .in1(out_ui_bit_and_expr_FU_8_0_8_72_i1_fu___udivsi3_5825_7906),
    .in2(out_const_2));
  ui_rshift_expr_FU #(.BITSIZE_in1(8),
    .BITSIZE_in2(1),
    .BITSIZE_out1(7),
    .PRECISION(64)) fu___udivsi3_5825_428597 (.out1(out_ui_rshift_expr_FU_8_0_8_106_i0_fu___udivsi3_5825_428597),
    .in1(out_ui_rshift_expr_FU_32_0_32_102_i0_fu___udivsi3_5825_7915),
    .in2(out_const_2));
  ui_lshift_expr_FU #(.BITSIZE_in1(7),
    .BITSIZE_in2(1),
    .BITSIZE_out1(8),
    .PRECISION(64)) fu___udivsi3_5825_428602 (.out1(out_ui_lshift_expr_FU_8_0_8_85_i1_fu___udivsi3_5825_428602),
    .in1(out_ui_bit_and_expr_FU_8_0_8_72_i2_fu___udivsi3_5825_7916),
    .in2(out_const_2));
  ui_rshift_expr_FU #(.BITSIZE_in1(8),
    .BITSIZE_in2(1),
    .BITSIZE_out1(3),
    .PRECISION(64)) fu___udivsi3_5825_428621 (.out1(out_ui_rshift_expr_FU_8_0_8_106_i1_fu___udivsi3_5825_428621),
    .in1(out_ui_lshift_expr_FU_8_0_8_85_i7_fu___udivsi3_5825_430382),
    .in2(out_const_2));
  ui_rshift_expr_FU #(.BITSIZE_in1(4),
    .BITSIZE_in2(1),
    .BITSIZE_out1(3),
    .PRECISION(64)) fu___udivsi3_5825_428624 (.out1(out_ui_rshift_expr_FU_8_0_8_106_i2_fu___udivsi3_5825_428624),
    .in1(out_ui_rshift_expr_FU_8_0_8_107_i0_fu___udivsi3_5825_7938),
    .in2(out_const_2));
  ui_lshift_expr_FU #(.BITSIZE_in1(3),
    .BITSIZE_in2(1),
    .BITSIZE_out1(4),
    .PRECISION(64)) fu___udivsi3_5825_428629 (.out1(out_ui_lshift_expr_FU_8_0_8_85_i2_fu___udivsi3_5825_428629),
    .in1(out_ui_cond_expr_FU_8_8_8_8_81_i3_fu___udivsi3_5825_7939),
    .in2(out_const_2));
  ui_lshift_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_in2(1),
    .BITSIZE_out1(2),
    .PRECISION(32)) fu___udivsi3_5825_428651 (.out1(out_ui_lshift_expr_FU_8_0_8_86_i0_fu___udivsi3_5825_428651),
    .in1(out_ui_rshift_expr_FU_32_0_32_99_i0_fu___udivsi3_5825_428714),
    .in2(out_const_2));
  ui_lshift_expr_FU #(.BITSIZE_in1(3),
    .BITSIZE_in2(1),
    .BITSIZE_out1(4),
    .PRECISION(64)) fu___udivsi3_5825_428659 (.out1(out_ui_lshift_expr_FU_8_0_8_85_i3_fu___udivsi3_5825_428659),
    .in1(out_ui_cond_expr_FU_8_8_8_8_81_i4_fu___udivsi3_5825_7943),
    .in2(out_const_2));
  ui_rshift_expr_FU #(.BITSIZE_in1(4),
    .BITSIZE_in2(1),
    .BITSIZE_out1(3),
    .PRECISION(64)) fu___udivsi3_5825_428662 (.out1(out_ui_rshift_expr_FU_8_0_8_106_i3_fu___udivsi3_5825_428662),
    .in1(out_ui_lshift_expr_FU_8_0_8_85_i3_fu___udivsi3_5825_428659),
    .in2(out_const_2));
  ui_rshift_expr_FU #(.BITSIZE_in1(4),
    .BITSIZE_in2(1),
    .BITSIZE_out1(3),
    .PRECISION(64)) fu___udivsi3_5825_428665 (.out1(out_ui_rshift_expr_FU_8_0_8_106_i4_fu___udivsi3_5825_428665),
    .in1(out_ui_lshift_expr_FU_8_0_8_85_i2_fu___udivsi3_5825_428629),
    .in2(out_const_2));
  ui_lshift_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_in2(2),
    .BITSIZE_out1(3),
    .PRECISION(32)) fu___udivsi3_5825_428681 (.out1(out_ui_lshift_expr_FU_8_0_8_87_i0_fu___udivsi3_5825_428681),
    .in1(out_ui_rshift_expr_FU_32_0_32_99_i1_fu___udivsi3_5825_428724),
    .in2(out_const_3));
  ui_lshift_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_in2(3),
    .BITSIZE_out1(5),
    .PRECISION(32)) fu___udivsi3_5825_428690 (.out1(out_ui_lshift_expr_FU_8_0_8_88_i0_fu___udivsi3_5825_428690),
    .in1(out_ui_rshift_expr_FU_32_0_32_99_i2_fu___udivsi3_5825_428734),
    .in2(out_const_4));
  UUdata_converter_FU #(.BITSIZE_in1(1),
    .BITSIZE_out1(1)) fu___udivsi3_5825_428707 (.out1(out_UUdata_converter_FU_55_i0_fu___udivsi3_5825_428707),
    .in1(out_lut_expr_FU_54_i0_fu___udivsi3_5825_428499));
  ui_lshift_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_in2(5),
    .BITSIZE_out1(32),
    .PRECISION(32)) fu___udivsi3_5825_428711 (.out1(out_ui_lshift_expr_FU_32_0_32_82_i0_fu___udivsi3_5825_428711),
    .in1(out_UUdata_converter_FU_55_i0_fu___udivsi3_5825_428707),
    .in2(out_const_40));
  ui_rshift_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5),
    .BITSIZE_out1(1),
    .PRECISION(32)) fu___udivsi3_5825_428714 (.out1(out_ui_rshift_expr_FU_32_0_32_99_i0_fu___udivsi3_5825_428714),
    .in1(out_ui_lshift_expr_FU_32_0_32_82_i0_fu___udivsi3_5825_428711),
    .in2(out_const_40));
  UUdata_converter_FU #(.BITSIZE_in1(1),
    .BITSIZE_out1(1)) fu___udivsi3_5825_428718 (.out1(out_UUdata_converter_FU_61_i0_fu___udivsi3_5825_428718),
    .in1(out_lut_expr_FU_45_i0_fu___udivsi3_5825_428493));
  ui_lshift_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_in2(5),
    .BITSIZE_out1(32),
    .PRECISION(32)) fu___udivsi3_5825_428721 (.out1(out_ui_lshift_expr_FU_32_0_32_82_i1_fu___udivsi3_5825_428721),
    .in1(out_UUdata_converter_FU_61_i0_fu___udivsi3_5825_428718),
    .in2(out_const_40));
  ui_rshift_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5),
    .BITSIZE_out1(1),
    .PRECISION(32)) fu___udivsi3_5825_428724 (.out1(out_ui_rshift_expr_FU_32_0_32_99_i1_fu___udivsi3_5825_428724),
    .in1(out_ui_lshift_expr_FU_32_0_32_82_i1_fu___udivsi3_5825_428721),
    .in2(out_const_40));
  UUdata_converter_FU #(.BITSIZE_in1(1),
    .BITSIZE_out1(1)) fu___udivsi3_5825_428728 (.out1(out_UUdata_converter_FU_4_i0_fu___udivsi3_5825_428728),
    .in1(out_ui_lt_expr_FU_32_0_32_90_i0_fu___udivsi3_5825_428462));
  ui_lshift_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_in2(5),
    .BITSIZE_out1(32),
    .PRECISION(32)) fu___udivsi3_5825_428731 (.out1(out_ui_lshift_expr_FU_32_0_32_82_i2_fu___udivsi3_5825_428731),
    .in1(out_UUdata_converter_FU_4_i0_fu___udivsi3_5825_428728),
    .in2(out_const_40));
  ui_rshift_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5),
    .BITSIZE_out1(1),
    .PRECISION(32)) fu___udivsi3_5825_428734 (.out1(out_ui_rshift_expr_FU_32_0_32_99_i2_fu___udivsi3_5825_428734),
    .in1(out_ui_lshift_expr_FU_32_0_32_82_i2_fu___udivsi3_5825_428731),
    .in2(out_const_40));
  lut_expr_FU #(.BITSIZE_in1(2),
    .BITSIZE_out1(1)) fu___udivsi3_5825_428748 (.out1(out_lut_expr_FU_28_i0_fu___udivsi3_5825_428748),
    .in1(out_const_3),
    .in2(out_ui_lt_expr_FU_32_0_32_90_i0_fu___udivsi3_5825_428462),
    .in3(out_lut_expr_FU_27_i0_fu___udivsi3_5825_430397),
    .in4(1'b0),
    .in5(1'b0),
    .in6(1'b0),
    .in7(1'b0),
    .in8(1'b0),
    .in9(1'b0));
  lut_expr_FU #(.BITSIZE_in1(4),
    .BITSIZE_out1(1)) fu___udivsi3_5825_428752 (.out1(out_lut_expr_FU_29_i0_fu___udivsi3_5825_428752),
    .in1(out_const_5),
    .in2(out_ui_lt_expr_FU_32_0_32_90_i0_fu___udivsi3_5825_428462),
    .in3(out_lut_expr_FU_27_i0_fu___udivsi3_5825_430397),
    .in4(1'b0),
    .in5(1'b0),
    .in6(1'b0),
    .in7(1'b0),
    .in8(1'b0),
    .in9(1'b0));
  ui_cond_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_in2(7),
    .BITSIZE_in3(7),
    .BITSIZE_out1(7)) fu___udivsi3_5825_428770 (.out1(out_ui_cond_expr_FU_8_8_8_8_81_i0_fu___udivsi3_5825_428770),
    .in1(out_lut_expr_FU_29_i0_fu___udivsi3_5825_428752),
    .in2(out_ui_rshift_expr_FU_8_0_8_106_i5_fu___udivsi3_5825_428793),
    .in3(out_ui_rshift_expr_FU_8_0_8_106_i6_fu___udivsi3_5825_428796));
  lut_expr_FU #(.BITSIZE_in1(4),
    .BITSIZE_out1(1)) fu___udivsi3_5825_428773 (.out1(out_lut_expr_FU_30_i0_fu___udivsi3_5825_428773),
    .in1(out_const_35),
    .in2(out_ui_lt_expr_FU_32_0_32_90_i0_fu___udivsi3_5825_428462),
    .in3(out_lut_expr_FU_27_i0_fu___udivsi3_5825_430397),
    .in4(1'b0),
    .in5(1'b0),
    .in6(1'b0),
    .in7(1'b0),
    .in8(1'b0),
    .in9(1'b0));
  ui_cond_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_in2(7),
    .BITSIZE_in3(7),
    .BITSIZE_out1(7)) fu___udivsi3_5825_428782 (.out1(out_ui_cond_expr_FU_8_8_8_8_81_i1_fu___udivsi3_5825_428782),
    .in1(out_lut_expr_FU_28_i0_fu___udivsi3_5825_428748),
    .in2(out_ui_rshift_expr_FU_8_0_8_106_i8_fu___udivsi3_5825_430365),
    .in3(out_ui_rshift_expr_FU_8_0_8_106_i9_fu___udivsi3_5825_430368));
  UUdata_converter_FU #(.BITSIZE_in1(1),
    .BITSIZE_out1(1)) fu___udivsi3_5825_428784 (.out1(out_UUdata_converter_FU_33_i0_fu___udivsi3_5825_428784),
    .in1(out_lut_expr_FU_32_i0_fu___udivsi3_5825_428904));
  ui_cond_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_in2(7),
    .BITSIZE_in3(7),
    .BITSIZE_out1(7)) fu___udivsi3_5825_428788 (.out1(out_ui_cond_expr_FU_8_8_8_8_81_i2_fu___udivsi3_5825_428788),
    .in1(out_lut_expr_FU_30_i0_fu___udivsi3_5825_428773),
    .in2(out_ui_rshift_expr_FU_8_0_8_106_i10_fu___udivsi3_5825_430375),
    .in3(out_ui_rshift_expr_FU_8_0_8_106_i11_fu___udivsi3_5825_430378));
  ASSIGN_UNSIGNED_FU #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_428790 (.out1(out_ASSIGN_UNSIGNED_FU_6_i0_fu___udivsi3_5825_428790),
    .in1(out_ui_negate_expr_FU_32_32_95_i0_fu___udivsi3_5825_7963));
  ui_rshift_expr_FU #(.BITSIZE_in1(8),
    .BITSIZE_in2(1),
    .BITSIZE_out1(7),
    .PRECISION(64)) fu___udivsi3_5825_428793 (.out1(out_ui_rshift_expr_FU_8_0_8_106_i5_fu___udivsi3_5825_428793),
    .in1(out_ui_lshift_expr_FU_8_0_8_85_i0_fu___udivsi3_5825_428582),
    .in2(out_const_2));
  ui_rshift_expr_FU #(.BITSIZE_in1(8),
    .BITSIZE_in2(1),
    .BITSIZE_out1(7),
    .PRECISION(64)) fu___udivsi3_5825_428796 (.out1(out_ui_rshift_expr_FU_8_0_8_106_i6_fu___udivsi3_5825_428796),
    .in1(out_ui_lshift_expr_FU_8_0_8_85_i1_fu___udivsi3_5825_428602),
    .in2(out_const_2));
  ui_lshift_expr_FU #(.BITSIZE_in1(7),
    .BITSIZE_in2(1),
    .BITSIZE_out1(8),
    .PRECISION(64)) fu___udivsi3_5825_428800 (.out1(out_ui_lshift_expr_FU_8_0_8_85_i4_fu___udivsi3_5825_428800),
    .in1(out_ui_cond_expr_FU_8_8_8_8_81_i0_fu___udivsi3_5825_428770),
    .in2(out_const_2));
  ui_lshift_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_in2(2),
    .BITSIZE_out1(4),
    .PRECISION(32)) fu___udivsi3_5825_428812 (.out1(out_ui_lshift_expr_FU_8_0_8_89_i0_fu___udivsi3_5825_428812),
    .in1(out_UUdata_converter_FU_33_i0_fu___udivsi3_5825_428784),
    .in2(out_const_26));
  lut_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_out1(1)) fu___udivsi3_5825_428904 (.out1(out_lut_expr_FU_32_i0_fu___udivsi3_5825_428904),
    .in1(out_const_2),
    .in2(out_lut_expr_FU_28_i0_fu___udivsi3_5825_428748),
    .in3(out_lut_expr_FU_31_i0_fu___udivsi3_5825_430403),
    .in4(1'b0),
    .in5(1'b0),
    .in6(1'b0),
    .in7(1'b0),
    .in8(1'b0),
    .in9(1'b0));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(3),
    .BITSIZE_in2(1)) fu___udivsi3_5825_429260 (.out1(out_ui_extract_bit_expr_FU_56_i0_fu___udivsi3_5825_429260),
    .in1(out_ui_bit_and_expr_FU_8_8_8_74_i0_fu___udivsi3_5825_7944),
    .in2(out_const_0));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(3),
    .BITSIZE_in2(2)) fu___udivsi3_5825_429264 (.out1(out_ui_extract_bit_expr_FU_57_i0_fu___udivsi3_5825_429264),
    .in1(out_ui_bit_and_expr_FU_8_8_8_74_i0_fu___udivsi3_5825_7944),
    .in2(out_const_3));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5)) fu___udivsi3_5825_429272 (.out1(out_ui_extract_bit_expr_FU_9_i0_fu___udivsi3_5825_429272),
    .in1(in_port_v),
    .in2(out_const_29));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5)) fu___udivsi3_5825_429281 (.out1(out_ui_extract_bit_expr_FU_11_i0_fu___udivsi3_5825_429281),
    .in1(in_port_v),
    .in2(out_const_30));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5)) fu___udivsi3_5825_429290 (.out1(out_ui_extract_bit_expr_FU_13_i0_fu___udivsi3_5825_429290),
    .in1(in_port_v),
    .in2(out_const_32));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5)) fu___udivsi3_5825_429299 (.out1(out_ui_extract_bit_expr_FU_15_i0_fu___udivsi3_5825_429299),
    .in1(in_port_v),
    .in2(out_const_33));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5)) fu___udivsi3_5825_429307 (.out1(out_ui_extract_bit_expr_FU_17_i0_fu___udivsi3_5825_429307),
    .in1(in_port_v),
    .in2(out_const_36));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5)) fu___udivsi3_5825_429315 (.out1(out_ui_extract_bit_expr_FU_19_i0_fu___udivsi3_5825_429315),
    .in1(in_port_v),
    .in2(out_const_37));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5)) fu___udivsi3_5825_429323 (.out1(out_ui_extract_bit_expr_FU_21_i0_fu___udivsi3_5825_429323),
    .in1(in_port_v),
    .in2(out_const_39));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5)) fu___udivsi3_5825_429331 (.out1(out_ui_extract_bit_expr_FU_23_i0_fu___udivsi3_5825_429331),
    .in1(in_port_v),
    .in2(out_const_40));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(4)) fu___udivsi3_5825_429403 (.out1(out_ui_extract_bit_expr_FU_8_i0_fu___udivsi3_5825_429403),
    .in1(in_port_v),
    .in2(out_const_5));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(4)) fu___udivsi3_5825_429407 (.out1(out_ui_extract_bit_expr_FU_10_i0_fu___udivsi3_5825_429407),
    .in1(in_port_v),
    .in2(out_const_13));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(4)) fu___udivsi3_5825_429411 (.out1(out_ui_extract_bit_expr_FU_12_i0_fu___udivsi3_5825_429411),
    .in1(in_port_v),
    .in2(out_const_17));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(4)) fu___udivsi3_5825_429415 (.out1(out_ui_extract_bit_expr_FU_14_i0_fu___udivsi3_5825_429415),
    .in1(in_port_v),
    .in2(out_const_22));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(4)) fu___udivsi3_5825_429419 (.out1(out_ui_extract_bit_expr_FU_16_i0_fu___udivsi3_5825_429419),
    .in1(in_port_v),
    .in2(out_const_28));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(4)) fu___udivsi3_5825_429423 (.out1(out_ui_extract_bit_expr_FU_18_i0_fu___udivsi3_5825_429423),
    .in1(in_port_v),
    .in2(out_const_31));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(4)) fu___udivsi3_5825_429427 (.out1(out_ui_extract_bit_expr_FU_20_i0_fu___udivsi3_5825_429427),
    .in1(in_port_v),
    .in2(out_const_35));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(4)) fu___udivsi3_5825_429431 (.out1(out_ui_extract_bit_expr_FU_22_i0_fu___udivsi3_5825_429431),
    .in1(in_port_v),
    .in2(out_const_38));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(3)) fu___udivsi3_5825_429504 (.out1(out_ui_extract_bit_expr_FU_34_i0_fu___udivsi3_5825_429504),
    .in1(in_port_v),
    .in2(out_const_4));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(3)) fu___udivsi3_5825_429508 (.out1(out_ui_extract_bit_expr_FU_35_i0_fu___udivsi3_5825_429508),
    .in1(in_port_v),
    .in2(out_const_16));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(3)) fu___udivsi3_5825_429512 (.out1(out_ui_extract_bit_expr_FU_36_i0_fu___udivsi3_5825_429512),
    .in1(in_port_v),
    .in2(out_const_27));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(3)) fu___udivsi3_5825_429516 (.out1(out_ui_extract_bit_expr_FU_37_i0_fu___udivsi3_5825_429516),
    .in1(in_port_v),
    .in2(out_const_34));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5)) fu___udivsi3_5825_429520 (.out1(out_ui_extract_bit_expr_FU_38_i0_fu___udivsi3_5825_429520),
    .in1(in_port_v),
    .in2(out_const_18));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5)) fu___udivsi3_5825_429524 (.out1(out_ui_extract_bit_expr_FU_39_i0_fu___udivsi3_5825_429524),
    .in1(in_port_v),
    .in2(out_const_20));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5)) fu___udivsi3_5825_429528 (.out1(out_ui_extract_bit_expr_FU_40_i0_fu___udivsi3_5825_429528),
    .in1(in_port_v),
    .in2(out_const_23));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5)) fu___udivsi3_5825_429532 (.out1(out_ui_extract_bit_expr_FU_41_i0_fu___udivsi3_5825_429532),
    .in1(in_port_v),
    .in2(out_const_24));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(2)) fu___udivsi3_5825_430228 (.out1(out_ui_extract_bit_expr_FU_46_i0_fu___udivsi3_5825_430228),
    .in1(in_port_v),
    .in2(out_const_3));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(2)) fu___udivsi3_5825_430245 (.out1(out_ui_extract_bit_expr_FU_48_i0_fu___udivsi3_5825_430245),
    .in1(in_port_v),
    .in2(out_const_26));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5)) fu___udivsi3_5825_430294 (.out1(out_ui_extract_bit_expr_FU_47_i0_fu___udivsi3_5825_430294),
    .in1(in_port_v),
    .in2(out_const_14));
  ui_extract_bit_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5)) fu___udivsi3_5825_430306 (.out1(out_ui_extract_bit_expr_FU_49_i0_fu___udivsi3_5825_430306),
    .in1(in_port_v),
    .in2(out_const_15));
  ui_rshift_expr_FU #(.BITSIZE_in1(8),
    .BITSIZE_in2(1),
    .BITSIZE_out1(7),
    .PRECISION(64)) fu___udivsi3_5825_430358 (.out1(out_ui_rshift_expr_FU_8_0_8_106_i7_fu___udivsi3_5825_430358),
    .in1(out_ui_rshift_expr_FU_32_0_32_100_i0_fu___udivsi3_5825_7879),
    .in2(out_const_2));
  ui_lshift_expr_FU #(.BITSIZE_in1(7),
    .BITSIZE_in2(1),
    .BITSIZE_out1(8),
    .PRECISION(64)) fu___udivsi3_5825_430362 (.out1(out_ui_lshift_expr_FU_8_0_8_85_i5_fu___udivsi3_5825_430362),
    .in1(out_ui_bit_and_expr_FU_8_0_8_72_i0_fu___udivsi3_5825_7880),
    .in2(out_const_2));
  ui_rshift_expr_FU #(.BITSIZE_in1(8),
    .BITSIZE_in2(1),
    .BITSIZE_out1(7),
    .PRECISION(64)) fu___udivsi3_5825_430365 (.out1(out_ui_rshift_expr_FU_8_0_8_106_i8_fu___udivsi3_5825_430365),
    .in1(out_ui_lshift_expr_FU_8_0_8_85_i5_fu___udivsi3_5825_430362),
    .in2(out_const_2));
  ui_rshift_expr_FU #(.BITSIZE_in1(8),
    .BITSIZE_in2(1),
    .BITSIZE_out1(7),
    .PRECISION(64)) fu___udivsi3_5825_430368 (.out1(out_ui_rshift_expr_FU_8_0_8_106_i9_fu___udivsi3_5825_430368),
    .in1(out_ui_lshift_expr_FU_8_0_8_85_i4_fu___udivsi3_5825_428800),
    .in2(out_const_2));
  ui_lshift_expr_FU #(.BITSIZE_in1(7),
    .BITSIZE_in2(1),
    .BITSIZE_out1(8),
    .PRECISION(64)) fu___udivsi3_5825_430372 (.out1(out_ui_lshift_expr_FU_8_0_8_85_i6_fu___udivsi3_5825_430372),
    .in1(out_ui_cond_expr_FU_8_8_8_8_81_i1_fu___udivsi3_5825_428782),
    .in2(out_const_2));
  ui_rshift_expr_FU #(.BITSIZE_in1(8),
    .BITSIZE_in2(1),
    .BITSIZE_out1(7),
    .PRECISION(64)) fu___udivsi3_5825_430375 (.out1(out_ui_rshift_expr_FU_8_0_8_106_i10_fu___udivsi3_5825_430375),
    .in1(out_ui_lshift_expr_FU_8_0_8_85_i6_fu___udivsi3_5825_430372),
    .in2(out_const_2));
  ui_rshift_expr_FU #(.BITSIZE_in1(8),
    .BITSIZE_in2(1),
    .BITSIZE_out1(7),
    .PRECISION(64)) fu___udivsi3_5825_430378 (.out1(out_ui_rshift_expr_FU_8_0_8_106_i11_fu___udivsi3_5825_430378),
    .in1(out_ui_rshift_expr_FU_32_0_32_101_i0_fu___udivsi3_5825_7881),
    .in2(out_const_2));
  ui_lshift_expr_FU #(.BITSIZE_in1(7),
    .BITSIZE_in2(1),
    .BITSIZE_out1(8),
    .PRECISION(64)) fu___udivsi3_5825_430382 (.out1(out_ui_lshift_expr_FU_8_0_8_85_i7_fu___udivsi3_5825_430382),
    .in1(out_ui_cond_expr_FU_8_8_8_8_81_i2_fu___udivsi3_5825_428788),
    .in2(out_const_2));
  lut_expr_FU #(.BITSIZE_in1(22),
    .BITSIZE_out1(1)) fu___udivsi3_5825_430386 (.out1(out_lut_expr_FU_24_i0_fu___udivsi3_5825_430386),
    .in1(out_const_11),
    .in2(out_ui_lt_expr_FU_32_0_32_90_i0_fu___udivsi3_5825_428462),
    .in3(out_ui_extract_bit_expr_FU_12_i0_fu___udivsi3_5825_429411),
    .in4(out_ui_extract_bit_expr_FU_13_i0_fu___udivsi3_5825_429290),
    .in5(out_ui_extract_bit_expr_FU_14_i0_fu___udivsi3_5825_429415),
    .in6(out_ui_extract_bit_expr_FU_15_i0_fu___udivsi3_5825_429299),
    .in7(1'b0),
    .in8(1'b0),
    .in9(1'b0));
  lut_expr_FU #(.BITSIZE_in1(55),
    .BITSIZE_out1(1)) fu___udivsi3_5825_430390 (.out1(out_lut_expr_FU_25_i0_fu___udivsi3_5825_430390),
    .in1(out_const_19),
    .in2(out_ui_extract_bit_expr_FU_8_i0_fu___udivsi3_5825_429403),
    .in3(out_ui_extract_bit_expr_FU_9_i0_fu___udivsi3_5825_429272),
    .in4(out_ui_lt_expr_FU_32_0_32_90_i0_fu___udivsi3_5825_428462),
    .in5(out_ui_extract_bit_expr_FU_10_i0_fu___udivsi3_5825_429407),
    .in6(out_ui_extract_bit_expr_FU_11_i0_fu___udivsi3_5825_429281),
    .in7(out_lut_expr_FU_24_i0_fu___udivsi3_5825_430386),
    .in8(1'b0),
    .in9(1'b0));
  lut_expr_FU #(.BITSIZE_in1(54),
    .BITSIZE_out1(1)) fu___udivsi3_5825_430394 (.out1(out_lut_expr_FU_26_i0_fu___udivsi3_5825_430394),
    .in1(out_const_12),
    .in2(out_ui_lt_expr_FU_32_0_32_90_i0_fu___udivsi3_5825_428462),
    .in3(out_ui_extract_bit_expr_FU_20_i0_fu___udivsi3_5825_429427),
    .in4(out_ui_extract_bit_expr_FU_21_i0_fu___udivsi3_5825_429323),
    .in5(out_ui_extract_bit_expr_FU_22_i0_fu___udivsi3_5825_429431),
    .in6(out_ui_extract_bit_expr_FU_23_i0_fu___udivsi3_5825_429331),
    .in7(out_lut_expr_FU_25_i0_fu___udivsi3_5825_430390),
    .in8(1'b0),
    .in9(1'b0));
  lut_expr_FU #(.BITSIZE_in1(54),
    .BITSIZE_out1(1)) fu___udivsi3_5825_430397 (.out1(out_lut_expr_FU_27_i0_fu___udivsi3_5825_430397),
    .in1(out_const_12),
    .in2(out_ui_lt_expr_FU_32_0_32_90_i0_fu___udivsi3_5825_428462),
    .in3(out_ui_extract_bit_expr_FU_16_i0_fu___udivsi3_5825_429419),
    .in4(out_ui_extract_bit_expr_FU_17_i0_fu___udivsi3_5825_429307),
    .in5(out_ui_extract_bit_expr_FU_18_i0_fu___udivsi3_5825_429423),
    .in6(out_ui_extract_bit_expr_FU_19_i0_fu___udivsi3_5825_429315),
    .in7(out_lut_expr_FU_26_i0_fu___udivsi3_5825_430394),
    .in8(1'b0),
    .in9(1'b0));
  lut_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_out1(1)) fu___udivsi3_5825_430403 (.out1(out_lut_expr_FU_31_i0_fu___udivsi3_5825_430403),
    .in1(out_const_2),
    .in2(out_ui_lt_expr_FU_32_0_32_90_i0_fu___udivsi3_5825_428462),
    .in3(out_lut_expr_FU_27_i0_fu___udivsi3_5825_430397),
    .in4(1'b0),
    .in5(1'b0),
    .in6(1'b0),
    .in7(1'b0),
    .in8(1'b0),
    .in9(1'b0));
  lut_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_out1(1)) fu___udivsi3_5825_430407 (.out1(out_lut_expr_FU_42_i0_fu___udivsi3_5825_430407),
    .in1(out_const_2),
    .in2(out_ui_extract_bit_expr_FU_16_i0_fu___udivsi3_5825_429419),
    .in3(out_ui_extract_bit_expr_FU_18_i0_fu___udivsi3_5825_429423),
    .in4(out_ui_extract_bit_expr_FU_20_i0_fu___udivsi3_5825_429427),
    .in5(out_ui_extract_bit_expr_FU_22_i0_fu___udivsi3_5825_429431),
    .in6(1'b0),
    .in7(1'b0),
    .in8(1'b0),
    .in9(1'b0));
  lut_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_out1(1)) fu___udivsi3_5825_430410 (.out1(out_lut_expr_FU_43_i0_fu___udivsi3_5825_430410),
    .in1(out_const_2),
    .in2(out_ui_extract_bit_expr_FU_38_i0_fu___udivsi3_5825_429520),
    .in3(out_ui_extract_bit_expr_FU_39_i0_fu___udivsi3_5825_429524),
    .in4(out_ui_extract_bit_expr_FU_40_i0_fu___udivsi3_5825_429528),
    .in5(out_ui_extract_bit_expr_FU_41_i0_fu___udivsi3_5825_429532),
    .in6(1'b0),
    .in7(1'b0),
    .in8(1'b0),
    .in9(1'b0));
  lut_expr_FU #(.BITSIZE_in1(49),
    .BITSIZE_out1(1)) fu___udivsi3_5825_430413 (.out1(out_lut_expr_FU_44_i0_fu___udivsi3_5825_430413),
    .in1(out_const_44),
    .in2(out_ui_extract_bit_expr_FU_34_i0_fu___udivsi3_5825_429504),
    .in3(out_ui_extract_bit_expr_FU_35_i0_fu___udivsi3_5825_429508),
    .in4(out_ui_extract_bit_expr_FU_36_i0_fu___udivsi3_5825_429512),
    .in5(out_ui_extract_bit_expr_FU_37_i0_fu___udivsi3_5825_429516),
    .in6(out_lut_expr_FU_29_i0_fu___udivsi3_5825_428752),
    .in7(out_lut_expr_FU_43_i0_fu___udivsi3_5825_430410),
    .in8(1'b0),
    .in9(1'b0));
  lut_expr_FU #(.BITSIZE_in1(64),
    .BITSIZE_out1(1)) fu___udivsi3_5825_430419 (.out1(out_lut_expr_FU_50_i0_fu___udivsi3_5825_430419),
    .in1(out_const_43),
    .in2(out_ui_lt_expr_FU_32_0_32_90_i0_fu___udivsi3_5825_428462),
    .in3(out_ui_extract_bit_expr_FU_12_i0_fu___udivsi3_5825_429411),
    .in4(out_ui_extract_bit_expr_FU_13_i0_fu___udivsi3_5825_429290),
    .in5(out_ui_extract_bit_expr_FU_46_i0_fu___udivsi3_5825_430228),
    .in6(out_ui_extract_bit_expr_FU_47_i0_fu___udivsi3_5825_430294),
    .in7(out_lut_expr_FU_27_i0_fu___udivsi3_5825_430397),
    .in8(1'b0),
    .in9(1'b0));
  lut_expr_FU #(.BITSIZE_in1(64),
    .BITSIZE_out1(1)) fu___udivsi3_5825_430422 (.out1(out_lut_expr_FU_51_i0_fu___udivsi3_5825_430422),
    .in1(out_const_43),
    .in2(out_ui_lt_expr_FU_32_0_32_90_i0_fu___udivsi3_5825_428462),
    .in3(out_ui_extract_bit_expr_FU_20_i0_fu___udivsi3_5825_429427),
    .in4(out_ui_extract_bit_expr_FU_21_i0_fu___udivsi3_5825_429323),
    .in5(out_ui_extract_bit_expr_FU_36_i0_fu___udivsi3_5825_429512),
    .in6(out_ui_extract_bit_expr_FU_40_i0_fu___udivsi3_5825_429528),
    .in7(out_lut_expr_FU_27_i0_fu___udivsi3_5825_430397),
    .in8(1'b0),
    .in9(1'b0));
  lut_expr_FU #(.BITSIZE_in1(64),
    .BITSIZE_out1(1)) fu___udivsi3_5825_430425 (.out1(out_lut_expr_FU_52_i0_fu___udivsi3_5825_430425),
    .in1(out_const_43),
    .in2(out_ui_lt_expr_FU_32_0_32_90_i0_fu___udivsi3_5825_428462),
    .in3(out_ui_extract_bit_expr_FU_14_i0_fu___udivsi3_5825_429415),
    .in4(out_ui_extract_bit_expr_FU_15_i0_fu___udivsi3_5825_429299),
    .in5(out_ui_extract_bit_expr_FU_48_i0_fu___udivsi3_5825_430245),
    .in6(out_ui_extract_bit_expr_FU_49_i0_fu___udivsi3_5825_430306),
    .in7(out_lut_expr_FU_27_i0_fu___udivsi3_5825_430397),
    .in8(1'b0),
    .in9(1'b0));
  lut_expr_FU #(.BITSIZE_in1(64),
    .BITSIZE_out1(1)) fu___udivsi3_5825_430428 (.out1(out_lut_expr_FU_53_i0_fu___udivsi3_5825_430428),
    .in1(out_const_43),
    .in2(out_ui_lt_expr_FU_32_0_32_90_i0_fu___udivsi3_5825_428462),
    .in3(out_ui_extract_bit_expr_FU_22_i0_fu___udivsi3_5825_429431),
    .in4(out_ui_extract_bit_expr_FU_23_i0_fu___udivsi3_5825_429331),
    .in5(out_ui_extract_bit_expr_FU_37_i0_fu___udivsi3_5825_429516),
    .in6(out_ui_extract_bit_expr_FU_41_i0_fu___udivsi3_5825_429532),
    .in7(out_lut_expr_FU_27_i0_fu___udivsi3_5825_430397),
    .in8(1'b0),
    .in9(1'b0));
  UUdata_converter_FU #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_7877 (.out1(out_UUdata_converter_FU_3_i0_fu___udivsi3_5825_7877),
    .in1(in_port_v));
  ui_rshift_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(4),
    .BITSIZE_out1(8),
    .PRECISION(64)) fu___udivsi3_5825_7879 (.out1(out_ui_rshift_expr_FU_32_0_32_100_i0_fu___udivsi3_5825_7879),
    .in1(out_UUdata_converter_FU_3_i0_fu___udivsi3_5825_7877),
    .in2(out_const_5));
  ui_bit_and_expr_FU #(.BITSIZE_in1(7),
    .BITSIZE_in2(7),
    .BITSIZE_out1(7)) fu___udivsi3_5825_7880 (.out1(out_ui_bit_and_expr_FU_8_0_8_72_i0_fu___udivsi3_5825_7880),
    .in1(out_ui_rshift_expr_FU_8_0_8_106_i7_fu___udivsi3_5825_430358),
    .in2(out_const_41));
  ui_rshift_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5),
    .BITSIZE_out1(8),
    .PRECISION(64)) fu___udivsi3_5825_7881 (.out1(out_ui_rshift_expr_FU_32_0_32_101_i0_fu___udivsi3_5825_7881),
    .in1(out_UUdata_converter_FU_3_i0_fu___udivsi3_5825_7877),
    .in2(out_const_29));
  ui_bit_and_expr_FU #(.BITSIZE_in1(7),
    .BITSIZE_in2(7),
    .BITSIZE_out1(7)) fu___udivsi3_5825_7906 (.out1(out_ui_bit_and_expr_FU_8_0_8_72_i1_fu___udivsi3_5825_7906),
    .in1(out_ui_rshift_expr_FU_32_0_32_98_i0_fu___udivsi3_5825_428576),
    .in2(out_const_41));
  ui_rshift_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5),
    .BITSIZE_out1(8),
    .PRECISION(64)) fu___udivsi3_5825_7915 (.out1(out_ui_rshift_expr_FU_32_0_32_102_i0_fu___udivsi3_5825_7915),
    .in1(out_UUdata_converter_FU_3_i0_fu___udivsi3_5825_7877),
    .in2(out_const_6));
  ui_bit_and_expr_FU #(.BITSIZE_in1(7),
    .BITSIZE_in2(7),
    .BITSIZE_out1(7)) fu___udivsi3_5825_7916 (.out1(out_ui_bit_and_expr_FU_8_0_8_72_i2_fu___udivsi3_5825_7916),
    .in1(out_ui_rshift_expr_FU_8_0_8_106_i0_fu___udivsi3_5825_428597),
    .in2(out_const_41));
  ui_rshift_expr_FU #(.BITSIZE_in1(8),
    .BITSIZE_in2(3),
    .BITSIZE_out1(4),
    .PRECISION(64)) fu___udivsi3_5825_7938 (.out1(out_ui_rshift_expr_FU_8_0_8_107_i0_fu___udivsi3_5825_7938),
    .in1(out_ui_lshift_expr_FU_8_0_8_85_i7_fu___udivsi3_5825_430382),
    .in2(out_const_4));
  ui_cond_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_in2(3),
    .BITSIZE_in3(3),
    .BITSIZE_out1(3)) fu___udivsi3_5825_7939 (.out1(out_ui_cond_expr_FU_8_8_8_8_81_i3_fu___udivsi3_5825_7939),
    .in1(out_lut_expr_FU_45_i0_fu___udivsi3_5825_428493),
    .in2(out_ui_rshift_expr_FU_8_0_8_106_i1_fu___udivsi3_5825_428621),
    .in3(out_ui_rshift_expr_FU_8_0_8_106_i2_fu___udivsi3_5825_428624));
  ui_cond_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_in2(1),
    .BITSIZE_in3(3),
    .BITSIZE_out1(3)) fu___udivsi3_5825_7943 (.out1(out_ui_cond_expr_FU_8_8_8_8_81_i4_fu___udivsi3_5825_7943),
    .in1(out_lut_expr_FU_54_i0_fu___udivsi3_5825_428499),
    .in2(out_const_2),
    .in3(out_const_4));
  ui_bit_and_expr_FU #(.BITSIZE_in1(3),
    .BITSIZE_in2(3),
    .BITSIZE_out1(3)) fu___udivsi3_5825_7944 (.out1(out_ui_bit_and_expr_FU_8_8_8_74_i0_fu___udivsi3_5825_7944),
    .in1(out_ui_rshift_expr_FU_8_0_8_106_i3_fu___udivsi3_5825_428662),
    .in2(out_ui_rshift_expr_FU_8_0_8_106_i4_fu___udivsi3_5825_428665));
  UUdata_converter_FU #(.BITSIZE_in1(1),
    .BITSIZE_out1(1)) fu___udivsi3_5825_7945 (.out1(out_UUdata_converter_FU_59_i0_fu___udivsi3_5825_7945),
    .in1(out_lut_expr_FU_58_i0_fu___udivsi3_5825_428504));
  UUdata_converter_FU #(.BITSIZE_in1(1),
    .BITSIZE_out1(1)) fu___udivsi3_5825_7946 (.out1(out_UUdata_converter_FU_60_i0_fu___udivsi3_5825_7946),
    .in1(out_UUdata_converter_FU_59_i0_fu___udivsi3_5825_7945));
  ui_bit_ior_expr_FU #(.BITSIZE_in1(4),
    .BITSIZE_in2(5),
    .BITSIZE_out1(5)) fu___udivsi3_5825_7949 (.out1(out_ui_bit_ior_expr_FU_0_8_8_75_i0_fu___udivsi3_5825_7949),
    .in1(out_ui_lshift_expr_FU_8_0_8_89_i0_fu___udivsi3_5825_428812),
    .in2(out_ui_lshift_expr_FU_8_0_8_88_i0_fu___udivsi3_5825_428690));
  ui_bit_ior_expr_FU #(.BITSIZE_in1(5),
    .BITSIZE_in2(3),
    .BITSIZE_out1(5)) fu___udivsi3_5825_7950 (.out1(out_ui_bit_ior_expr_FU_0_8_8_76_i0_fu___udivsi3_5825_7950),
    .in1(out_ui_bit_ior_expr_FU_0_8_8_75_i0_fu___udivsi3_5825_7949),
    .in2(out_ui_lshift_expr_FU_8_0_8_87_i0_fu___udivsi3_5825_428681));
  ui_bit_ior_expr_FU #(.BITSIZE_in1(5),
    .BITSIZE_in2(2),
    .BITSIZE_out1(5)) fu___udivsi3_5825_7951 (.out1(out_ui_bit_ior_expr_FU_0_8_8_77_i0_fu___udivsi3_5825_7951),
    .in1(out_ui_bit_ior_expr_FU_0_8_8_76_i0_fu___udivsi3_5825_7950),
    .in2(out_ui_lshift_expr_FU_8_0_8_86_i0_fu___udivsi3_5825_428651));
  ui_bit_ior_expr_FU #(.BITSIZE_in1(5),
    .BITSIZE_in2(1),
    .BITSIZE_out1(5)) fu___udivsi3_5825_7952 (.out1(out_ui_bit_ior_expr_FU_0_8_8_78_i0_fu___udivsi3_5825_7952),
    .in1(out_ui_bit_ior_expr_FU_0_8_8_77_i0_fu___udivsi3_5825_7951),
    .in2(out_UUdata_converter_FU_60_i0_fu___udivsi3_5825_7946));
  ui_lshift_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5),
    .BITSIZE_out1(31),
    .PRECISION(32)) fu___udivsi3_5825_7953 (.out1(out_ui_lshift_expr_FU_32_32_32_84_i0_fu___udivsi3_5825_7953),
    .in1(in_port_v),
    .in2(out_ui_bit_ior_expr_FU_0_8_8_78_i0_fu___udivsi3_5825_7952));
  ui_rshift_expr_FU #(.BITSIZE_in1(31),
    .BITSIZE_in2(5),
    .BITSIZE_out1(8),
    .PRECISION(32)) fu___udivsi3_5825_7954 (.out1(out_ui_rshift_expr_FU_32_0_32_103_i0_fu___udivsi3_5825_7954),
    .in1(out_ui_lshift_expr_FU_32_32_32_84_i0_fu___udivsi3_5825_7953),
    .in2(out_const_24));
  ui_bit_and_expr_FU #(.BITSIZE_in1(8),
    .BITSIZE_in2(8),
    .BITSIZE_out1(8)) fu___udivsi3_5825_7955 (.out1(out_ui_bit_and_expr_FU_8_0_8_73_i0_fu___udivsi3_5825_7955),
    .in1(out_ui_rshift_expr_FU_32_0_32_103_i0_fu___udivsi3_5825_7954),
    .in2(out_const_42));
  ui_pointer_plus_expr_FU #(.BITSIZE_in1(11),
    .BITSIZE_in2(8),
    .BITSIZE_out1(11),
    .LSB_PARAMETER(0)) fu___udivsi3_5825_7956 (.out1(out_ui_pointer_plus_expr_FU_16_16_16_97_i0_fu___udivsi3_5825_7956),
    .in1(out_reg_4_reg_4),
    .in2(out_reg_0_reg_0));
  UUdata_converter_FU #(.BITSIZE_in1(8),
    .BITSIZE_out1(8)) fu___udivsi3_5825_7958 (.out1(out_UUdata_converter_FU_62_i0_fu___udivsi3_5825_7958),
    .in1(out_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_array_7052_0));
  ui_lshift_expr_FU #(.BITSIZE_in1(8),
    .BITSIZE_in2(5),
    .BITSIZE_out1(31),
    .PRECISION(32)) fu___udivsi3_5825_7959 (.out1(out_ui_lshift_expr_FU_32_0_32_83_i0_fu___udivsi3_5825_7959),
    .in1(out_UUdata_converter_FU_62_i0_fu___udivsi3_5825_7958),
    .in2(out_const_24));
  ui_bit_ior_expr_FU #(.BITSIZE_in1(31),
    .BITSIZE_in2(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_7960 (.out1(out_ui_bit_ior_expr_FU_32_0_32_79_i0_fu___udivsi3_5825_7960),
    .in1(out_ui_lshift_expr_FU_32_0_32_83_i0_fu___udivsi3_5825_7959),
    .in2(out_const_10));
  ui_bit_xor_expr_FU #(.BITSIZE_in1(5),
    .BITSIZE_in2(5),
    .BITSIZE_out1(5)) fu___udivsi3_5825_7961 (.out1(out_ui_bit_xor_expr_FU_8_0_8_80_i0_fu___udivsi3_5825_7961),
    .in1(out_ui_bit_ior_expr_FU_0_8_8_78_i0_fu___udivsi3_5825_7952),
    .in2(out_const_40));
  ui_rshift_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(5),
    .BITSIZE_out1(32),
    .PRECISION(32)) fu___udivsi3_5825_7962 (.out1(out_ui_rshift_expr_FU_32_32_32_104_i0_fu___udivsi3_5825_7962),
    .in1(out_ui_bit_ior_expr_FU_32_0_32_79_i0_fu___udivsi3_5825_7960),
    .in2(out_reg_1_reg_1));
  ui_negate_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_7963 (.out1(out_ui_negate_expr_FU_32_32_95_i0_fu___udivsi3_5825_7963),
    .in1(in_port_v));
  ui_mult_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(32),
    .BITSIZE_out1(32),
    .PIPE_PARAMETER(0)) fu___udivsi3_5825_7964 (.out1(out_ui_mult_expr_FU_32_32_32_0_94_i0_fu___udivsi3_5825_7964),
    .clock(clock),
    .in1(out_reg_6_reg_6),
    .in2(out_reg_2_reg_2));
  UUdata_converter_FU #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_7965 (.out1(out_UUdata_converter_FU_63_i0_fu___udivsi3_5825_7965),
    .in1(out_ui_rshift_expr_FU_32_32_32_104_i0_fu___udivsi3_5825_7962));
  UUdata_converter_FU #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_7966 (.out1(out_UUdata_converter_FU_64_i0_fu___udivsi3_5825_7966),
    .in1(out_ui_mult_expr_FU_32_32_32_0_94_i0_fu___udivsi3_5825_7964));
  ui_mult_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(32),
    .BITSIZE_out1(64),
    .PIPE_PARAMETER(0)) fu___udivsi3_5825_7967 (.out1(out_ui_mult_expr_FU_32_32_32_0_94_i1_fu___udivsi3_5825_7967),
    .clock(clock),
    .in1(out_reg_8_reg_8),
    .in2(out_reg_7_reg_7));
  ui_rshift_expr_FU #(.BITSIZE_in1(64),
    .BITSIZE_in2(6),
    .BITSIZE_out1(32),
    .PRECISION(64)) fu___udivsi3_5825_7968 (.out1(out_ui_rshift_expr_FU_64_0_64_105_i0_fu___udivsi3_5825_7968),
    .in1(out_ui_mult_expr_FU_32_32_32_0_94_i1_fu___udivsi3_5825_7967),
    .in2(out_const_7));
  UUdata_converter_FU #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_7969 (.out1(out_UUdata_converter_FU_65_i0_fu___udivsi3_5825_7969),
    .in1(out_ui_rshift_expr_FU_64_0_64_105_i0_fu___udivsi3_5825_7968));
  ui_plus_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_7970 (.out1(out_ui_plus_expr_FU_32_32_32_96_i0_fu___udivsi3_5825_7970),
    .in1(out_reg_6_reg_6),
    .in2(out_reg_9_reg_9));
  ui_mult_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(32),
    .BITSIZE_out1(32),
    .PIPE_PARAMETER(0)) fu___udivsi3_5825_7971 (.out1(out_ui_mult_expr_FU_32_32_32_0_94_i2_fu___udivsi3_5825_7971),
    .clock(clock),
    .in1(out_reg_10_reg_10),
    .in2(out_reg_5_reg_5));
  UUdata_converter_FU #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_7972 (.out1(out_UUdata_converter_FU_66_i0_fu___udivsi3_5825_7972),
    .in1(out_ui_plus_expr_FU_32_32_32_96_i0_fu___udivsi3_5825_7970));
  UUdata_converter_FU #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_7973 (.out1(out_UUdata_converter_FU_67_i0_fu___udivsi3_5825_7973),
    .in1(out_ui_mult_expr_FU_32_32_32_0_94_i2_fu___udivsi3_5825_7971));
  ui_mult_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(32),
    .BITSIZE_out1(64),
    .PIPE_PARAMETER(0)) fu___udivsi3_5825_7974 (.out1(out_ui_mult_expr_FU_32_32_32_0_94_i3_fu___udivsi3_5825_7974),
    .clock(clock),
    .in1(out_reg_12_reg_12),
    .in2(out_reg_11_reg_11));
  ui_rshift_expr_FU #(.BITSIZE_in1(64),
    .BITSIZE_in2(6),
    .BITSIZE_out1(32),
    .PRECISION(64)) fu___udivsi3_5825_7975 (.out1(out_ui_rshift_expr_FU_64_0_64_105_i1_fu___udivsi3_5825_7975),
    .in1(out_ui_mult_expr_FU_32_32_32_0_94_i3_fu___udivsi3_5825_7974),
    .in2(out_const_7));
  UUdata_converter_FU #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_7976 (.out1(out_UUdata_converter_FU_68_i0_fu___udivsi3_5825_7976),
    .in1(out_ui_rshift_expr_FU_64_0_64_105_i1_fu___udivsi3_5825_7975));
  ui_plus_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_7977 (.out1(out_ui_plus_expr_FU_32_32_32_96_i1_fu___udivsi3_5825_7977),
    .in1(out_reg_10_reg_10),
    .in2(out_reg_13_reg_13));
  UUdata_converter_FU #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_7978 (.out1(out_UUdata_converter_FU_7_i0_fu___udivsi3_5825_7978),
    .in1(in_port_u));
  UUdata_converter_FU #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_7979 (.out1(out_UUdata_converter_FU_69_i0_fu___udivsi3_5825_7979),
    .in1(out_ui_plus_expr_FU_32_32_32_96_i1_fu___udivsi3_5825_7977));
  ui_mult_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(32),
    .BITSIZE_out1(64),
    .PIPE_PARAMETER(0)) fu___udivsi3_5825_7980 (.out1(out_ui_mult_expr_FU_32_32_32_0_94_i4_fu___udivsi3_5825_7980),
    .clock(clock),
    .in1(out_reg_14_reg_14),
    .in2(out_reg_3_reg_3));
  ui_rshift_expr_FU #(.BITSIZE_in1(64),
    .BITSIZE_in2(6),
    .BITSIZE_out1(32),
    .PRECISION(64)) fu___udivsi3_5825_7981 (.out1(out_ui_rshift_expr_FU_64_0_64_105_i2_fu___udivsi3_5825_7981),
    .in1(out_ui_mult_expr_FU_32_32_32_0_94_i4_fu___udivsi3_5825_7980),
    .in2(out_const_7));
  UUdata_converter_FU #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_7982 (.out1(out_UUdata_converter_FU_70_i0_fu___udivsi3_5825_7982),
    .in1(out_ui_rshift_expr_FU_64_0_64_105_i2_fu___udivsi3_5825_7981));
  ui_mult_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(32),
    .BITSIZE_out1(32),
    .PIPE_PARAMETER(0)) fu___udivsi3_5825_7983 (.out1(out_ui_mult_expr_FU_32_32_32_0_94_i5_fu___udivsi3_5825_7983),
    .clock(clock),
    .in1(out_reg_15_reg_15),
    .in2(in_port_v));
  ui_minus_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_7984 (.out1(out_ui_minus_expr_FU_32_32_32_93_i0_fu___udivsi3_5825_7984),
    .in1(in_port_u),
    .in2(out_reg_16_reg_16));
  ui_minus_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_7986 (.out1(out_ui_minus_expr_FU_32_32_32_93_i1_fu___udivsi3_5825_7986),
    .in1(out_ui_minus_expr_FU_32_32_32_93_i0_fu___udivsi3_5825_7984),
    .in2(in_port_v));
  ui_cond_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_in2(1),
    .BITSIZE_in3(2),
    .BITSIZE_out1(2)) fu___udivsi3_5825_7988 (.out1(out_ui_cond_expr_FU_8_8_8_8_81_i5_fu___udivsi3_5825_7988),
    .in1(out_ui_lt_expr_FU_32_32_32_92_i1_fu___udivsi3_5825_428521),
    .in2(out_const_2),
    .in3(out_const_3));
  ui_cond_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_in2(1),
    .BITSIZE_in3(2),
    .BITSIZE_out1(2)) fu___udivsi3_5825_7989 (.out1(out_ui_cond_expr_FU_8_8_8_8_81_i6_fu___udivsi3_5825_7989),
    .in1(out_ui_lt_expr_FU_32_32_32_92_i0_fu___udivsi3_5825_428518),
    .in2(out_const_0),
    .in3(out_ui_cond_expr_FU_8_8_8_8_81_i5_fu___udivsi3_5825_7988));
  ui_plus_expr_FU #(.BITSIZE_in1(2),
    .BITSIZE_in2(32),
    .BITSIZE_out1(32)) fu___udivsi3_5825_7990 (.out1(out_ui_plus_expr_FU_32_32_32_96_i2_fu___udivsi3_5825_7990),
    .in1(out_ui_cond_expr_FU_8_8_8_8_81_i6_fu___udivsi3_5825_7989),
    .in2(out_reg_15_reg_15));
  register_STD #(.BITSIZE_in1(8),
    .BITSIZE_out1(8)) reg_0 (.out1(out_reg_0_reg_0),
    .clock(clock),
    .reset(reset),
    .in1(out_ui_bit_and_expr_FU_8_0_8_73_i0_fu___udivsi3_5825_7955),
    .wenable(wrenable_reg_0));
  register_STD #(.BITSIZE_in1(5),
    .BITSIZE_out1(5)) reg_1 (.out1(out_reg_1_reg_1),
    .clock(clock),
    .reset(reset),
    .in1(out_ui_bit_xor_expr_FU_8_0_8_80_i0_fu___udivsi3_5825_7961),
    .wenable(wrenable_reg_1));
  register_SE #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) reg_10 (.out1(out_reg_10_reg_10),
    .clock(clock),
    .reset(reset),
    .in1(out_ui_plus_expr_FU_32_32_32_96_i0_fu___udivsi3_5825_7970),
    .wenable(wrenable_reg_10));
  register_SE #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) reg_11 (.out1(out_reg_11_reg_11),
    .clock(clock),
    .reset(reset),
    .in1(out_UUdata_converter_FU_66_i0_fu___udivsi3_5825_7972),
    .wenable(wrenable_reg_11));
  register_STD #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) reg_12 (.out1(out_reg_12_reg_12),
    .clock(clock),
    .reset(reset),
    .in1(out_UUdata_converter_FU_67_i0_fu___udivsi3_5825_7973),
    .wenable(wrenable_reg_12));
  register_STD #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) reg_13 (.out1(out_reg_13_reg_13),
    .clock(clock),
    .reset(reset),
    .in1(out_UUdata_converter_FU_68_i0_fu___udivsi3_5825_7976),
    .wenable(wrenable_reg_13));
  register_STD #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) reg_14 (.out1(out_reg_14_reg_14),
    .clock(clock),
    .reset(reset),
    .in1(out_UUdata_converter_FU_69_i0_fu___udivsi3_5825_7979),
    .wenable(wrenable_reg_14));
  register_SE #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) reg_15 (.out1(out_reg_15_reg_15),
    .clock(clock),
    .reset(reset),
    .in1(out_UUdata_converter_FU_70_i0_fu___udivsi3_5825_7982),
    .wenable(wrenable_reg_15));
  register_STD #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) reg_16 (.out1(out_reg_16_reg_16),
    .clock(clock),
    .reset(reset),
    .in1(out_ui_mult_expr_FU_32_32_32_0_94_i5_fu___udivsi3_5825_7983),
    .wenable(wrenable_reg_16));
  register_SE #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) reg_2 (.out1(out_reg_2_reg_2),
    .clock(clock),
    .reset(reset),
    .in1(out_ui_negate_expr_FU_32_32_95_i0_fu___udivsi3_5825_7963),
    .wenable(wrenable_reg_2));
  register_SE #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) reg_3 (.out1(out_reg_3_reg_3),
    .clock(clock),
    .reset(reset),
    .in1(out_UUdata_converter_FU_7_i0_fu___udivsi3_5825_7978),
    .wenable(wrenable_reg_3));
  register_STD #(.BITSIZE_in1(11),
    .BITSIZE_out1(11)) reg_4 (.out1(out_reg_4_reg_4),
    .clock(clock),
    .reset(reset),
    .in1(out_addr_expr_FU_5_i0_fu___udivsi3_5825_428532),
    .wenable(wrenable_reg_4));
  register_SE #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) reg_5 (.out1(out_reg_5_reg_5),
    .clock(clock),
    .reset(reset),
    .in1(out_ASSIGN_UNSIGNED_FU_6_i0_fu___udivsi3_5825_428790),
    .wenable(wrenable_reg_5));
  register_SE #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) reg_6 (.out1(out_reg_6_reg_6),
    .clock(clock),
    .reset(reset),
    .in1(out_ui_rshift_expr_FU_32_32_32_104_i0_fu___udivsi3_5825_7962),
    .wenable(wrenable_reg_6));
  register_SE #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) reg_7 (.out1(out_reg_7_reg_7),
    .clock(clock),
    .reset(reset),
    .in1(out_UUdata_converter_FU_63_i0_fu___udivsi3_5825_7965),
    .wenable(wrenable_reg_7));
  register_STD #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) reg_8 (.out1(out_reg_8_reg_8),
    .clock(clock),
    .reset(reset),
    .in1(out_UUdata_converter_FU_64_i0_fu___udivsi3_5825_7966),
    .wenable(wrenable_reg_8));
  register_STD #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) reg_9 (.out1(out_reg_9_reg_9),
    .clock(clock),
    .reset(reset),
    .in1(out_UUdata_converter_FU_65_i0_fu___udivsi3_5825_7969),
    .wenable(wrenable_reg_9));
  // io-signal post fix
  assign return_port = out_ui_plus_expr_FU_32_32_32_96_i2_fu___udivsi3_5825_7990;

endmodule

// FSM based controller description for __udivsi3
// This component has been derived from the input source code and so it does not fall under the copyright of PandA framework, but it follows the input source code copyright, and may be aggregated with components of the BAMBU/PANDA IP LIBRARY.
// Author(s): Component automatically generated by bambu
// License: THIS COMPONENT IS PROVIDED "AS IS" AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
`timescale 1ns / 1ps
module controller___udivsi3(done_port,
  fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_LOAD,
  fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_STORE,
  wrenable_reg_0,
  wrenable_reg_1,
  wrenable_reg_10,
  wrenable_reg_11,
  wrenable_reg_12,
  wrenable_reg_13,
  wrenable_reg_14,
  wrenable_reg_15,
  wrenable_reg_16,
  wrenable_reg_2,
  wrenable_reg_3,
  wrenable_reg_4,
  wrenable_reg_5,
  wrenable_reg_6,
  wrenable_reg_7,
  wrenable_reg_8,
  wrenable_reg_9,
  clock,
  reset,
  start_port);
  // IN
  input clock;
  input reset;
  input start_port;
  // OUT
  output done_port;
  output fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_LOAD;
  output fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_STORE;
  output wrenable_reg_0;
  output wrenable_reg_1;
  output wrenable_reg_10;
  output wrenable_reg_11;
  output wrenable_reg_12;
  output wrenable_reg_13;
  output wrenable_reg_14;
  output wrenable_reg_15;
  output wrenable_reg_16;
  output wrenable_reg_2;
  output wrenable_reg_3;
  output wrenable_reg_4;
  output wrenable_reg_5;
  output wrenable_reg_6;
  output wrenable_reg_7;
  output wrenable_reg_8;
  output wrenable_reg_9;
  parameter [10:0] S_0 = 11'b00000000001,
    S_1 = 11'b00000000010,
    S_2 = 11'b00000000100,
    S_3 = 11'b00000001000,
    S_4 = 11'b00000010000,
    S_5 = 11'b00000100000,
    S_6 = 11'b00001000000,
    S_7 = 11'b00010000000,
    S_8 = 11'b00100000000,
    S_9 = 11'b01000000000,
    S_10 = 11'b10000000000;
  reg [10:0] _present_state=S_0, _next_state;
  reg done_port;
  reg fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_LOAD;
  reg fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_STORE;
  reg wrenable_reg_0;
  reg wrenable_reg_1;
  reg wrenable_reg_10;
  reg wrenable_reg_11;
  reg wrenable_reg_12;
  reg wrenable_reg_13;
  reg wrenable_reg_14;
  reg wrenable_reg_15;
  reg wrenable_reg_16;
  reg wrenable_reg_2;
  reg wrenable_reg_3;
  reg wrenable_reg_4;
  reg wrenable_reg_5;
  reg wrenable_reg_6;
  reg wrenable_reg_7;
  reg wrenable_reg_8;
  reg wrenable_reg_9;
  
  always @(posedge clock)
    if (reset == 1'b0) _present_state <= S_0;
    else _present_state <= _next_state;
  
  always @(*)
  begin
    done_port = 1'b0;
    fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_LOAD = 1'b0;
    fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_STORE = 1'b0;
    wrenable_reg_0 = 1'b0;
    wrenable_reg_1 = 1'b0;
    wrenable_reg_10 = 1'b0;
    wrenable_reg_11 = 1'b0;
    wrenable_reg_12 = 1'b0;
    wrenable_reg_13 = 1'b0;
    wrenable_reg_14 = 1'b0;
    wrenable_reg_15 = 1'b0;
    wrenable_reg_16 = 1'b0;
    wrenable_reg_2 = 1'b0;
    wrenable_reg_3 = 1'b0;
    wrenable_reg_4 = 1'b0;
    wrenable_reg_5 = 1'b0;
    wrenable_reg_6 = 1'b0;
    wrenable_reg_7 = 1'b0;
    wrenable_reg_8 = 1'b0;
    wrenable_reg_9 = 1'b0;
    case (_present_state)
      S_0 :
        if(start_port == 1'b1)
        begin
          wrenable_reg_0 = 1'b1;
          wrenable_reg_1 = 1'b1;
          wrenable_reg_2 = 1'b1;
          wrenable_reg_3 = 1'b1;
          wrenable_reg_4 = 1'b1;
          wrenable_reg_5 = 1'b1;
          _next_state = S_1;
        end
        else
        begin
          _next_state = S_0;
        end
      S_1 :
        begin
          fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_LOAD = 1'b1;
          wrenable_reg_6 = 1'b1;
          wrenable_reg_7 = 1'b1;
          _next_state = S_2;
        end
      S_2 :
        begin
          wrenable_reg_8 = 1'b1;
          _next_state = S_3;
        end
      S_3 :
        begin
          wrenable_reg_9 = 1'b1;
          _next_state = S_4;
        end
      S_4 :
        begin
          wrenable_reg_10 = 1'b1;
          wrenable_reg_11 = 1'b1;
          _next_state = S_5;
        end
      S_5 :
        begin
          wrenable_reg_12 = 1'b1;
          _next_state = S_6;
        end
      S_6 :
        begin
          wrenable_reg_13 = 1'b1;
          _next_state = S_7;
        end
      S_7 :
        begin
          wrenable_reg_14 = 1'b1;
          _next_state = S_8;
        end
      S_8 :
        begin
          wrenable_reg_15 = 1'b1;
          _next_state = S_9;
        end
      S_9 :
        begin
          wrenable_reg_16 = 1'b1;
          _next_state = S_10;
          done_port = 1'b1;
        end
      S_10 :
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

// This component is part of the BAMBU/PANDA IP LIBRARY
// Copyright (C) 2004-2023 Politecnico di Milano
// Author(s): Marco Lattuada <marco.lattuada@polimi.it>
// License: PANDA_LGPLv3
`timescale 1ns / 1ps
module flipflop_AR(clock,
  reset,
  in1,
  out1);
  parameter BITSIZE_in1=1,
    BITSIZE_out1=1;
  // IN
  input clock;
  input reset;
  input in1;
  // OUT
  output out1;
  
  reg reg_out1 =0;
  assign out1 = reg_out1;
  always @(posedge clock or negedge reset)
    if (reset == 1'b0)
      reg_out1 <= {BITSIZE_out1{1'b0}};
    else
      reg_out1 <= in1;
endmodule

// Top component for __udivsi3
// This component has been derived from the input source code and so it does not fall under the copyright of PandA framework, but it follows the input source code copyright, and may be aggregated with components of the BAMBU/PANDA IP LIBRARY.
// Author(s): Component automatically generated by bambu
// License: THIS COMPONENT IS PROVIDED "AS IS" AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
`timescale 1ns / 1ps
module __udivsi3(clock,
  reset,
  start_port,
  done_port,
  u,
  v,
  return_port);
  parameter MEM_var_7052_5825=1024;
  // IN
  input clock;
  input reset;
  input start_port;
  input [31:0] u;
  input [31:0] v;
  // OUT
  output done_port;
  output [31:0] return_port;
  // Component and signal declarations
  wire done_delayed_REG_signal_in;
  wire done_delayed_REG_signal_out;
  wire fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_LOAD;
  wire fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_STORE;
  wire wrenable_reg_0;
  wire wrenable_reg_1;
  wire wrenable_reg_10;
  wire wrenable_reg_11;
  wire wrenable_reg_12;
  wire wrenable_reg_13;
  wire wrenable_reg_14;
  wire wrenable_reg_15;
  wire wrenable_reg_16;
  wire wrenable_reg_2;
  wire wrenable_reg_3;
  wire wrenable_reg_4;
  wire wrenable_reg_5;
  wire wrenable_reg_6;
  wire wrenable_reg_7;
  wire wrenable_reg_8;
  wire wrenable_reg_9;
  
  controller___udivsi3 Controller_i (.done_port(done_delayed_REG_signal_in),
    .fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_LOAD(fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_LOAD),
    .fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_STORE(fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_STORE),
    .wrenable_reg_0(wrenable_reg_0),
    .wrenable_reg_1(wrenable_reg_1),
    .wrenable_reg_10(wrenable_reg_10),
    .wrenable_reg_11(wrenable_reg_11),
    .wrenable_reg_12(wrenable_reg_12),
    .wrenable_reg_13(wrenable_reg_13),
    .wrenable_reg_14(wrenable_reg_14),
    .wrenable_reg_15(wrenable_reg_15),
    .wrenable_reg_16(wrenable_reg_16),
    .wrenable_reg_2(wrenable_reg_2),
    .wrenable_reg_3(wrenable_reg_3),
    .wrenable_reg_4(wrenable_reg_4),
    .wrenable_reg_5(wrenable_reg_5),
    .wrenable_reg_6(wrenable_reg_6),
    .wrenable_reg_7(wrenable_reg_7),
    .wrenable_reg_8(wrenable_reg_8),
    .wrenable_reg_9(wrenable_reg_9),
    .clock(clock),
    .reset(reset),
    .start_port(start_port));
  datapath___udivsi3 #(.MEM_var_7052_5825(MEM_var_7052_5825)) Datapath_i (.return_port(return_port),
    .clock(clock),
    .reset(reset),
    .in_port_u(u),
    .in_port_v(v),
    .fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_LOAD(fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_LOAD),
    .fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_STORE(fuselector_ARRAY_1D_STD_DISTRAM_NN_SDS_0_i0_STORE),
    .wrenable_reg_0(wrenable_reg_0),
    .wrenable_reg_1(wrenable_reg_1),
    .wrenable_reg_10(wrenable_reg_10),
    .wrenable_reg_11(wrenable_reg_11),
    .wrenable_reg_12(wrenable_reg_12),
    .wrenable_reg_13(wrenable_reg_13),
    .wrenable_reg_14(wrenable_reg_14),
    .wrenable_reg_15(wrenable_reg_15),
    .wrenable_reg_16(wrenable_reg_16),
    .wrenable_reg_2(wrenable_reg_2),
    .wrenable_reg_3(wrenable_reg_3),
    .wrenable_reg_4(wrenable_reg_4),
    .wrenable_reg_5(wrenable_reg_5),
    .wrenable_reg_6(wrenable_reg_6),
    .wrenable_reg_7(wrenable_reg_7),
    .wrenable_reg_8(wrenable_reg_8),
    .wrenable_reg_9(wrenable_reg_9));
  flipflop_AR #(.BITSIZE_in1(1),
    .BITSIZE_out1(1)) done_delayed_REG (.out1(done_delayed_REG_signal_out),
    .clock(clock),
    .reset(reset),
    .in1(done_delayed_REG_signal_in));
  // io-signal post fix
  assign done_port = done_delayed_REG_signal_out;

endmodule

// Datapath RTL description for divider
// This component has been derived from the input source code and so it does not fall under the copyright of PandA framework, but it follows the input source code copyright, and may be aggregated with components of the BAMBU/PANDA IP LIBRARY.
// Author(s): Component automatically generated by bambu
// License: THIS COMPONENT IS PROVIDED "AS IS" AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
`timescale 1ns / 1ps
module datapath_divider(clock,
  reset,
  in_port_Pd5,
  in_port_P0,
  return_port,
  selector_IN_UNBOUNDED_divider_428394_428415,
  wrenable_reg_0,
  wrenable_reg_1,
  OUT_UNBOUNDED_divider_428394_428415);
  parameter MEM_var_7052_5825=1024;
  // IN
  input clock;
  input reset;
  input [31:0] in_port_Pd5;
  input [31:0] in_port_P0;
  input selector_IN_UNBOUNDED_divider_428394_428415;
  input wrenable_reg_0;
  input wrenable_reg_1;
  // OUT
  output [31:0] return_port;
  output OUT_UNBOUNDED_divider_428394_428415;
  // Component and signal declarations
  wire [31:0] out___udivsi3_5_i0_fu_divider_428394_428415;
  wire out_const_0;
  wire [1:0] out_const_1;
  wire [31:0] out_reg_0_reg_0;
  wire [31:0] out_reg_1_reg_1;
  wire [31:0] out_ui_cond_expr_FU_32_32_32_32_3_i0_fu_divider_428394_428414;
  wire out_ui_eq_expr_FU_32_0_32_4_i0_fu_divider_428394_428451;
  wire s_done_fu_divider_428394_428415;
  
  constant_value #(.BITSIZE_out1(1),
    .value(1'b0)) const_0 (.out1(out_const_0));
  constant_value #(.BITSIZE_out1(2),
    .value(2'b10)) const_1 (.out1(out_const_1));
  ui_cond_expr_FU #(.BITSIZE_in1(1),
    .BITSIZE_in2(2),
    .BITSIZE_in3(32),
    .BITSIZE_out1(32)) fu_divider_428394_428414 (.out1(out_ui_cond_expr_FU_32_32_32_32_3_i0_fu_divider_428394_428414),
    .in1(out_ui_eq_expr_FU_32_0_32_4_i0_fu_divider_428394_428451),
    .in2(out_const_1),
    .in3(in_port_P0));
  __udivsi3 #(.MEM_var_7052_5825(MEM_var_7052_5825)) fu_divider_428394_428415 (.done_port(s_done_fu_divider_428394_428415),
    .return_port(out___udivsi3_5_i0_fu_divider_428394_428415),
    .clock(clock),
    .reset(reset),
    .start_port(selector_IN_UNBOUNDED_divider_428394_428415),
    .u(in_port_Pd5),
    .v(out_reg_0_reg_0));
  ui_eq_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_in2(1),
    .BITSIZE_out1(1)) fu_divider_428394_428451 (.out1(out_ui_eq_expr_FU_32_0_32_4_i0_fu_divider_428394_428451),
    .in1(in_port_P0),
    .in2(out_const_0));
  register_SE #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) reg_0 (.out1(out_reg_0_reg_0),
    .clock(clock),
    .reset(reset),
    .in1(out_ui_cond_expr_FU_32_32_32_32_3_i0_fu_divider_428394_428414),
    .wenable(wrenable_reg_0));
  register_SE #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) reg_1 (.out1(out_reg_1_reg_1),
    .clock(clock),
    .reset(reset),
    .in1(out___udivsi3_5_i0_fu_divider_428394_428415),
    .wenable(wrenable_reg_1));
  // io-signal post fix
  assign return_port = out_reg_1_reg_1;
  assign OUT_UNBOUNDED_divider_428394_428415 = s_done_fu_divider_428394_428415;

endmodule

// FSM based controller description for divider
// This component has been derived from the input source code and so it does not fall under the copyright of PandA framework, but it follows the input source code copyright, and may be aggregated with components of the BAMBU/PANDA IP LIBRARY.
// Author(s): Component automatically generated by bambu
// License: THIS COMPONENT IS PROVIDED "AS IS" AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
`timescale 1ns / 1ps
module controller_divider(done_port,
  selector_IN_UNBOUNDED_divider_428394_428415,
  wrenable_reg_0,
  wrenable_reg_1,
  OUT_UNBOUNDED_divider_428394_428415,
  clock,
  reset,
  start_port);
  // IN
  input OUT_UNBOUNDED_divider_428394_428415;
  input clock;
  input reset;
  input start_port;
  // OUT
  output done_port;
  output selector_IN_UNBOUNDED_divider_428394_428415;
  output wrenable_reg_0;
  output wrenable_reg_1;
  parameter [3:0] S_0 = 4'b0001,
    S_1 = 4'b0010,
    S_2 = 4'b0100,
    S_3 = 4'b1000;
  reg [3:0] _present_state=S_0, _next_state;
  reg done_port;
  reg selector_IN_UNBOUNDED_divider_428394_428415;
  reg wrenable_reg_0;
  reg wrenable_reg_1;
  
  always @(posedge clock)
    if (reset == 1'b0) _present_state <= S_0;
    else _present_state <= _next_state;
  
  always @(*)
  begin
    done_port = 1'b0;
    selector_IN_UNBOUNDED_divider_428394_428415 = 1'b0;
    wrenable_reg_0 = 1'b0;
    wrenable_reg_1 = 1'b0;
    case (_present_state)
      S_0 :
        if(start_port == 1'b1)
        begin
          wrenable_reg_0 = 1'b1;
          _next_state = S_1;
        end
        else
        begin
          _next_state = S_0;
        end
      S_1 :
        begin
          selector_IN_UNBOUNDED_divider_428394_428415 = 1'b1;
          wrenable_reg_1 = OUT_UNBOUNDED_divider_428394_428415;
          if (OUT_UNBOUNDED_divider_428394_428415 == 1'b0)
            begin
              _next_state = S_2;
            end
          else
            begin
              _next_state = S_3;
              done_port = 1'b1;
            end
        end
      S_2 :
        begin
          wrenable_reg_1 = OUT_UNBOUNDED_divider_428394_428415;
          if (OUT_UNBOUNDED_divider_428394_428415 == 1'b0)
            begin
              _next_state = S_2;
            end
          else
            begin
              _next_state = S_3;
              done_port = 1'b1;
            end
        end
      S_3 :
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

// Top component for divider
// This component has been derived from the input source code and so it does not fall under the copyright of PandA framework, but it follows the input source code copyright, and may be aggregated with components of the BAMBU/PANDA IP LIBRARY.
// Author(s): Component automatically generated by bambu
// License: THIS COMPONENT IS PROVIDED "AS IS" AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
`timescale 1ns / 1ps
module _divider(clock,
  reset,
  start_port,
  done_port,
  Pd5,
  P0,
  return_port);
  // IN
  input clock;
  input reset;
  input start_port;
  input [31:0] Pd5;
  input [31:0] P0;
  // OUT
  output done_port;
  output [31:0] return_port;
  // Component and signal declarations
  wire OUT_UNBOUNDED_divider_428394_428415;
  wire done_delayed_REG_signal_in;
  wire done_delayed_REG_signal_out;
  wire selector_IN_UNBOUNDED_divider_428394_428415;
  wire wrenable_reg_0;
  wire wrenable_reg_1;
  
  controller_divider Controller_i (.done_port(done_delayed_REG_signal_in),
    .selector_IN_UNBOUNDED_divider_428394_428415(selector_IN_UNBOUNDED_divider_428394_428415),
    .wrenable_reg_0(wrenable_reg_0),
    .wrenable_reg_1(wrenable_reg_1),
    .OUT_UNBOUNDED_divider_428394_428415(OUT_UNBOUNDED_divider_428394_428415),
    .clock(clock),
    .reset(reset),
    .start_port(start_port));
  datapath_divider #(.MEM_var_7052_5825(1024)) Datapath_i (.return_port(return_port),
    .OUT_UNBOUNDED_divider_428394_428415(OUT_UNBOUNDED_divider_428394_428415),
    .clock(clock),
    .reset(reset),
    .in_port_Pd5(Pd5),
    .in_port_P0(P0),
    .selector_IN_UNBOUNDED_divider_428394_428415(selector_IN_UNBOUNDED_divider_428394_428415),
    .wrenable_reg_0(wrenable_reg_0),
    .wrenable_reg_1(wrenable_reg_1));
  flipflop_AR #(.BITSIZE_in1(1),
    .BITSIZE_out1(1)) done_delayed_REG (.out1(done_delayed_REG_signal_out),
    .clock(clock),
    .reset(reset),
    .in1(done_delayed_REG_signal_in));
  // io-signal post fix
  assign done_port = done_delayed_REG_signal_out;

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

// Minimal interface for function: divider
// This component has been derived from the input source code and so it does not fall under the copyright of PandA framework, but it follows the input source code copyright, and may be aggregated with components of the BAMBU/PANDA IP LIBRARY.
// Author(s): Component automatically generated by bambu
// License: THIS COMPONENT IS PROVIDED "AS IS" AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
`timescale 1ns / 1ps
module divider(clk,
  reset,
  start_port,
  Pd5,
  P0,
  done_port,
  return_port);
  // IN
  input clk;
  input reset;
  input start_port;
  input [31:0] Pd5;
  input [31:0] P0;
  // OUT
  output done_port;
  output [31:0] return_port;
  // Component and signal declarations
  wire [31:0] out_return_port_ui_view_convert_expr_FU;
  
  _divider _divider_i0 (.done_port(done_port),
    .return_port(out_return_port_ui_view_convert_expr_FU),
    .clock(clk),
    .reset(reset),
    .start_port(start_port),
    .Pd5(Pd5),
    .P0(P0));
  ui_view_convert_expr_FU #(.BITSIZE_in1(32),
    .BITSIZE_out1(32)) return_port_ui_view_convert_expr_FU (.out1(return_port),
    .in1(out_return_port_ui_view_convert_expr_FU));

endmodule


