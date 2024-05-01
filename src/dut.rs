mod top;
use top::Top;

// sram interface
pub struct Dut {
  top: Top,
  pub clocks: u64,
  pub prepare_for_difftest: bool,
}

impl Dut {
  pub fn new() -> Self {
    let mut top = Top::default();
    top.eval();
    top.eval();

    Dut {
      top,
      clocks: 0,
      prepare_for_difftest: false,
    }
  }

  pub fn reset(&mut self) {
    // clocks:|0|0|
    // reset: |-|-|
    // clock: |-|_|
    self.top.reset_toggle();

    self.top.clock_toggle();
    self.top.eval();

    self.top.clock_toggle();
    self.top.eval();

    println!("inst_sram_en: {}", self.top.inst_sram_en());
    println!("inst_sram_addr: 0x{:x}", self.top.inst_sram_addr());

    self.clocks += 1;

    self.top.reset_toggle();
  }

  /// drive the instruction SRAM interface
  pub fn step(&mut self, inst: u32) -> anyhow::Result<bool> {
    self.top.set_inst_sram_rdata(inst);

    self.top.clock_toggle();
    self.top.eval();

    self.top.clock_toggle();
    self.top.eval();

    self.clocks += 1;

    println!("==============================");
    println!("clocks: {}", self.clocks);
    println!("debug_commit: {}", self.top.debug_commit());
    println!("debug_pc: 0x{:x}", self.top.debug_pc());
    println!("debug_reg_wnum: {}", self.top.debug_reg_wnum());
    println!("debug_wdata: 0x{:x}", self.top.debug_wdata());

    Ok(self.top.inst_sram_en() != 0)
  }
}

impl Drop for Dut {
  fn drop(&mut self) {
    self.top.finish();

    println!("Simulation complete");
  }
}
