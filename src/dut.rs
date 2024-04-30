mod top;
use std::time::Duration;

use top::Top;

// sram interface
pub struct Dut {
  top: Top,
  pub clocks: u64,
}

impl Dut {
  pub fn new() -> Self {
    let mut top = Top::default();
    top.eval();
    top.eval();

    Dut { top, clocks: 0 }
  }

  pub fn exec(&mut self) -> anyhow::Result<()> {
    
    Ok(())
  }

  pub fn step(&mut self) -> anyhow::Result<()> {
    // clocks:|0|1|2|3|45678
    // reset: |-|-|_|_|_____
    // clock: |-|_|-|_|-_-_-
    if self.clocks == 0 {
      self.top.reset_toggle();
    } else if self.clocks == 2 {
      self.top.reset_toggle();
    }

    self.top.clock_toggle();
    self.top.eval();

    self.top.clock_toggle();
    self.top.eval();

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