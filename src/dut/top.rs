use verilated_module::module;

#[module(top)]
pub struct Top {
  #[port(clock)]
  pub clock: bool,
  #[port(reset)]
  pub reset: bool,
  // inst sram interface
  #[port(output)]
  pub inst_sram_en: bool,
  #[port(output)]
  pub inst_sram_wen: [bool; 4],
  #[port(output)]
  pub inst_sram_addr: [bool; 32],
  #[port(output)]
  pub inst_sram_wdata: [bool; 32],
  #[port(input)]
  pub inst_sram_rdata: [bool; 32],
  // data sram interface
  #[port(output)]
  pub data_sram_en: bool,
  #[port(output)]
  pub data_sram_wen: [bool; 8],
  #[port(output)]
  pub data_sram_addr: [bool; 32],
  #[port(output)]
  pub data_sram_wdata: [bool; 64],
  #[port(input)]
  pub data_sram_rdata: [bool; 64],
  // trace debug interface
  #[port(output)]
  pub debug_commit: bool,
  #[port(output)]
  pub debug_pc: [bool; 64],
  #[port(output)]
  pub debug_reg_wnum: [bool; 5],
  #[port(output)]
  pub debug_wdata: [bool; 64],
}
