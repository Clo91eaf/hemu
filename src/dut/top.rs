use std::time::Duration;
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

pub fn create_tb() {
  let mut tb: Top = Top::default();
  tb.eval();
  tb.eval();

  tb.open_trace("wave.vcd", 99).unwrap();

  let mut clocks: u64 = 0;
  while clocks < 100 {
    if clocks == 2 {
      tb.reset_toggle();
    }

    tb.clock_toggle();
    tb.eval();
    tb.trace_at(Duration::from_nanos(20 * clocks));

    tb.clock_toggle();
    tb.eval();
    tb.trace_at(Duration::from_nanos(20 * clocks + 10));

    clocks += 1;
  }
  tb.trace_at(Duration::from_nanos(20 * clocks));

  tb.finish();

  println!("Simulation complete");
}
