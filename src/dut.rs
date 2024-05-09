mod top;
use crate::emulator::DebugInfo;
use std::time::Duration;
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
  clock: bool,
  reset: bool,
  pub ticks: u64,
  pub prepare_for_difftest: bool,
  pub inst: u32,
  pub data: u64
}

impl Dut {
  pub fn new() -> Self {
    let mut top = Top::default();

    top.open_trace("wave.vcd", 99).unwrap();

    Dut {
      top,
      clock: false,
      reset: false,
      ticks: 0,
      prepare_for_difftest: false,
      inst: 0,
      data: 0
    }
  }

  fn clock_toggle(&mut self) {
    self.clock = !self.clock;
    self.top.clock_toggle();
  }

  fn reset_toggle(&mut self) {
    self.reset = !self.reset;
    self.top.reset_toggle();
  }

  /// drive the instruction SRAM interface
  pub fn step(&mut self, inst: u32, data: u64) -> anyhow::Result<(SramRequest, SramRequest, DebugInfo)> {
    match self.ticks {
      0 | 2 => self.reset_toggle(),
      _ => {},
    }

    // a little trick: there must be 2 state transitions after clock posedge
    self.clock_toggle();
    self.top.eval();
    if self.ticks >= 2 {
      self.top.set_inst_sram_rdata(inst);
      self.top.set_data_sram_rdata(data);
      self.top.eval();
    } 
    self.top.trace_at(Duration::from_nanos(self.ticks * 2));

    self.clock_toggle();
    self.top.eval();
    self.top.trace_at(Duration::from_nanos(self.ticks * 2 + 1));

    self.ticks += 1;

    info!(
      "[dut] ticks: {} commit: {} pc: {:#010x} wnum: {} wdata: {:#018x}",
      self.ticks,
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
          self.top.debug_commit() != 0 && self.top.debug_reg_wnum() != 0,
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
