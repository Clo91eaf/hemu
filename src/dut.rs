mod top;
use crate::bus::Bus;
use crate::cpu::{BYTE, WORD, HALFWORD, DOUBLEWORD};
use crate::emulator::{DebugInfo, GprInfo, MemInfo};
use std::time::Duration;
use top::Top;

fn extend_to_64(n: u8) -> u64 {
  let mut ans: u64 = 0;
  for i in 0..8 {
    ans |= ((n as u64 >> i & 1) * 0xff) << (i * 8);
  }
  ans
}

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
  pub top: Top,
  pub bus: Bus,
  pub ticks: u64,
  pub inst: u32,
  pub data: u64,
  pub trace: bool,
}

impl Dut {
  pub fn new(trace: bool) -> Self {
    let mut top = Top::default();

    if trace {
      top.open_trace("wave.vcd", 99).unwrap();
    }

    Dut {
      top,
      ticks: 0,
      bus: Bus::new(),
      inst: 0,
      data: 0,
      trace,
    }
  }

  fn trace(&mut self, ticks: u64) {
    if self.trace {
      self.top.trace_at(Duration::from_nanos(ticks));
    }
  }

  /// drive the instruction SRAM interface
  pub fn step(&mut self, inst: u32, data: u64) -> anyhow::Result<(SramRequest, SramRequest, DebugInfo)> {
    match self.ticks {
      0 | 2 => self.top.reset_toggle(),
      _ => {}
    }

    // a little trick: there must be 2 state transitions after clock posedge
    self.top.clock_toggle();
    self.top.eval();
    if self.ticks >= 2 {
      self.top.set_inst_sram_rdata(inst);
      self.top.set_data_sram_rdata(data);
      self.top.eval();
    }
    self.trace(self.ticks * 2);

    self.top.clock_toggle();
    self.top.eval();
    self.trace(self.ticks * 2 + 1);

    self.ticks += 1;

    // write data sram
    if self.top.data_sram_wen() != 0 {
      let data_sram_mask = extend_to_64(self.top.data_sram_wen());
      let data_sram_align = data_sram_mask.trailing_zeros();
      let data_sram_aligned = if data_sram_align == 0 {
        0
      } else {
        (self.top.data_sram_wdata() & data_sram_mask) >> data_sram_align
      };
      let data_sram_size = match data_sram_mask >> data_sram_align {
        0b0000_0001 => BYTE,
        0b0000_0011 => HALFWORD,
        0b0000_1111 => WORD,
        0b1111_1111 => DOUBLEWORD,
        _ => panic!("Invalid data sram size"),
      };
      let _ = self.bus.write(self.top.data_sram_addr() as u64, data_sram_aligned, data_sram_size);
    }


    Ok({
      (
        SramRequest::new(self.top.inst_sram_en() != 0, self.top.inst_sram_addr()),
        SramRequest::new(self.top.data_sram_en() != 0, self.top.data_sram_addr()),
        DebugInfo::new(
          GprInfo::new(
            self.top.debug_commit() != 0,
            self.top.debug_pc(),
            self.top.debug_rf_wnum(),
            self.top.debug_rf_wdata(),
          ),
          MemInfo::new(
            self.top.debug_sram_wen() != 0,
            if self.top.debug_sram_wen() != 0 {
              self.top.debug_sram_waddr()
            } else {
              0
            },
            if self.top.debug_sram_wen() != 0 {
              let wdata_mask = extend_to_64(self.top.debug_sram_wen());
              let align = wdata_mask.trailing_zeros();
              (self.top.debug_sram_wdata() & wdata_mask) >> align
            } else {
              0
            },
          ),
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_extend_to_64() {
    assert_eq!(extend_to_64(0b1111_1111), 0xffffffff_ffffffff);
    assert_eq!(extend_to_64(0b0000_1111), 0x00000000_ffffffff);
    assert_eq!(extend_to_64(0b0000_0001), 0x00000000_000000ff);
    assert_eq!(extend_to_64(0b0001_0001), 0x000000ff_000000ff);
  }
}
