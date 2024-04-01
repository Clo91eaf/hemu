mod top;

// sram interface
struct Dut {
  // trace debug interface
  pub debug_wb_pc: [bool; 32],
  pub debug_wb_rf_wen: [bool; 4],
  pub debug_wb_rf_wnum: [bool; 5],
  pub debug_wb_rf_wdata: [bool; 32],
}

impl Dut {
  fn new(rtl_files: Vec<String>) -> Self {
    Self {
      debug_wb_pc: [false; 32],
      debug_wb_rf_wen: [false; 4],
      debug_wb_rf_wnum: [false; 5],
      debug_wb_rf_wdata: [false; 32],
    }
  }
}
