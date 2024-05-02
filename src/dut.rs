mod top;
use crate::emulator::DebugInfo;
use top::Top;
use tracing::info;

pub struct SramRequest {
  pub en: bool,
  pub addr: u32,
}

impl SramRequest {
  pub fn new(en: bool, addr: u32) -> Self {
    SramRequest { en, addr }
  }
}

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

    self.clocks += 1;

    self.top.reset_toggle();
  }

  /// drive the instruction SRAM interface
  pub fn step(&mut self, inst: u32, data: u64) -> anyhow::Result<(SramRequest, SramRequest, DebugInfo)> {
    self.top.set_inst_sram_rdata(inst);
    self.top.set_data_sram_rdata(data);

    self.top.clock_toggle();
    self.top.eval();

    self.top.clock_toggle();
    self.top.eval();

    self.clocks += 1;

    info!(
      "[dut] clocks: {} commit: {} pc: {:#010x} wnum: {} wdata: {:#018x}",
      self.clocks,
      self.top.debug_commit(),
      self.top.debug_pc(),
      self.top.debug_reg_wnum(),
      self.top.debug_wdata()
    );

    Ok({
      (
        SramRequest::new(self.top.inst_sram_en() != 0, self.top.inst_sram_addr()),
        SramRequest::new(self.top.data_sram_en() != 0, self.top.data_sram_addr()),
        DebugInfo::new(
          self.top.debug_commit() != 0,
          self.top.debug_pc(),
          self.top.debug_reg_wnum(),
          self.top.debug_wdata(),
        ),
      )
    })
  }
}

impl Drop for Dut {
  fn drop(&mut self) {
    self.top.finish();

    println!("Simulation complete");
  }
}
