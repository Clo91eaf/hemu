use std::fmt;

use crate::cpu::REGISTERS_COUNT;

/// The floating-point registers.
#[derive(Debug)]
pub struct Fpg {
  fpg: [f64; REGISTERS_COUNT],
}

impl Fpg {
  /// Create a new `Fpg` object.
  pub fn new() -> Self {
    Self {
      fpg: [0.0; REGISTERS_COUNT],
    }
  }

  /// Read the value from a register.
  pub fn read(&self, index: u64) -> f64 {
    self.fpg[index as usize]
  }

  /// Write the value to a register.
  pub fn write(&mut self, index: u64, value: f64) {
    self.fpg[index as usize] = value;
  }

  /// Check fcsr flag.
  /// Floating-point control and status register (frm + fflags).
  /// | 31 .. 8 | 7 .. 5 | 4 | 3 | 2 | 1 | 0 |
  /// |Reserved | frm    | NV| DZ| OF| UF| NX|
  /// NV: Invalid Operation, most about NaN operation.
  /// DZ: Division by Zero, like 1/0.
  /// OF: Overflow, like 2^63.
  /// UF: Underflow, like 2^-64.
  /// NX: Inexact, like 1/3.
  pub fn fflags(&self, index: u64) -> [bool; 5] {
    let f = self.fpg[index as usize];
    [
      false,
      self.fpg[index as usize].is_infinite() && f.is_sign_negative(),
      self.fpg[index as usize].is_infinite() && f.is_sign_positive(),
      self.fpg[index as usize].is_infinite(),
      self.fpg[index as usize].is_nan(),
    ]
  }
}

impl fmt::Display for Fpg {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let abi = [
      // ft0-7: FP temporaries
      " ft0", " ft1", " ft2", " ft3", " ft4", " ft5", " ft6", " ft7", // fs0-1: FP saved registers
      " fs0", " fs1", // fa0-1: FP arguments/return values
      " fa0", " fa1", // fa2–7: FP arguments
      " fa2", " fa3", " fa4", " fa5", " fa6", " fa7", // fs2–11: FP saved registers
      " fs2", " fs3", " fs4", " fs5", " fs6", " fs7", " fs8", " fs9", "fs10", "fs11",
      // ft8–11: FP temporaries
      " ft8", " ft9", "ft10", "ft11",
    ];
    let mut output = String::from("");
    for i in (0..REGISTERS_COUNT).step_by(4) {
      output = format!(
                "{}\n{}",
                output,
                format!(
                    "f{:02}({})={:>width$.prec$} f{:02}({})={:>width$.prec$} f{:02}({})={:>width$.prec$} f{:02}({})={:>width$.prec$}",
                    i,
                    abi[i],
                    self.read(i as u64),
                    i + 1,
                    abi[i + 1],
                    self.read(i as u64 + 1),
                    i + 2,
                    abi[i + 2],
                    self.read(i  as u64+ 2),
                    i + 3,
                    abi[i + 3],
                    self.read(i as u64 + 3),
                    width=18,
                    prec=8,
                )
            );
    }
    // Remove the first new line.
    output.remove(0);
    write!(f, "{}", output)
  }
}
