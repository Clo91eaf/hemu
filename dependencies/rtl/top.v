module top(
    input         clock,
    input         reset,
    // inst sram interface
    output        inst_sram_en,
    output [ 3:0] inst_sram_wen,
    output [31:0] inst_sram_addr,
    output [31:0] inst_sram_wdata,
    input  [31:0] inst_sram_rdata,
    // data sram interface
    output        data_sram_en,
    output [ 7:0] data_sram_wen,
    output [31:0] data_sram_addr,
    output [63:0] data_sram_wdata,
    input  [63:0] data_sram_rdata,
    // trace debug interface
    output        debug_commit,
    output [63:0] debug_pc,
    output [4:0 ] debug_reg_wnum,
    output [63:0] debug_wdata
);

PuaCpu core(
    .clock                    (clock),
    .reset                    (reset),
    // interrupts     
    .io_ext_int_ei            (1'b0), // to PLIC
    .io_ext_int_si            (1'b0), // to CLINT
    .io_ext_int_ti            (1'b0), // to CLINT
    // inst sram interface 
    .io_inst_sram_en          (inst_sram_en),
    .io_inst_sram_wen         (inst_sram_wen),
    .io_inst_sram_addr        (inst_sram_addr),
    .io_inst_sram_wdata       (inst_sram_wdata),
    .io_inst_sram_rdata       (inst_sram_rdata),
    // data sram interface
    .io_data_sram_en          (data_sram_en),
    .io_data_sram_wen         (data_sram_wen),
    .io_data_sram_addr        (data_sram_addr),
    .io_data_sram_wdata       (data_sram_wdata),
    .io_data_sram_rdata       (data_sram_rdata),
    // debug
    .io_debug_wb_pc           (debug_pc),
    .io_debug_wb_rf_wen       (debug_commit),
    .io_debug_wb_rf_wnum      (debug_reg_wnum),
    .io_debug_wb_rf_wdata     (debug_wdata)
);

endmodule
