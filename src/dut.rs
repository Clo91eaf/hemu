mod top;
use std::time::Duration;

use top::Top;

// sram interface
pub struct Dut {
  top: Top,
  clocks: u64,
}

impl Dut {
  pub fn new() -> Self {
    let mut top = Top::default();
    top.eval();
    top.eval();

    top.open_trace("counter.vcd", 99).unwrap();

    Dut { top, clocks: 0 }
  }

  fn trace(&mut self) {
    self.top.trace_at(Duration::from_nanos(20 * self.clocks));
  }

  pub fn step(&mut self) -> anyhow::Result<()> {
    if self.clocks == 0 {
      self.top.reset_toggle();
    } else if self.clocks == 2 {
      self.top.reset_toggle();
    }

    self.top.clock_toggle();
    self.top.eval();
    self.trace();

    self.top.clock_toggle();
    self.top.eval();
    self.trace();

    self.clocks += 1;

    Ok(())
  }
}

impl Drop for Dut {
  fn drop(&mut self) {
    self.top.finish();

    println!("Simulation complete");
  }
}

// test for dut
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_dut() {
    let mut dut = Dut::new();
    for _ in 0..10 {
      dut.step().unwrap();
    }
  }
}