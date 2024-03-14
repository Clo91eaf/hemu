//! The emulator module represents an entire computer.

use crate::cpu::Cpu;
use crate::exception::Trap;

/// The emulator to hold a CPU.
pub struct Emulator {
  /// The CPU which is the core implementation of this emulator.
  pub cpu: Cpu,
}

impl Emulator {
  /// Constructor for an emulator.
  pub fn new() -> Emulator {
    Self {
      cpu: Cpu::new(),
    }
  }

  /// Reset CPU state.
  pub fn reset(&mut self) {
    self.cpu.reset()
  }

  /// Set binary data to the beginning of the DRAM from the emulator console.
  pub fn initialize_dram(&mut self, data: Vec<u8>) {
    self.cpu.bus.initialize_dram(data);
  }

  /// Set binary data to the virtio disk from the emulator console.
  pub fn initialize_disk(&mut self, data: Vec<u8>) {
    self.cpu.bus.initialize_disk(data);
  }

  /// Set the program counter to the CPU field.
  pub fn initialize_pc(&mut self, pc: u64) {
    self.cpu.pc = pc;
  }

  /// Start executing the emulator.
  pub fn start(&mut self) {
    loop {
      // Run a cycle on peripheral devices.
      self.cpu.devices_increment();

      // Take an interrupt.
      match self.cpu.check_pending_interrupt() {
        Some(interrupt) => interrupt.take_trap(&mut self.cpu),
        None => {}
      }

      // Execute an instruction.
      let trap = match self.cpu.execute() {
        Ok(_) => {
          // Return a placeholder trap.
          Trap::Requested
        }
        Err(exception) => exception.take_trap(&mut self.cpu),
      };

      match trap {
        Trap::Fatal => {
          println!("pc: {:#x}, trap {:#?}", self.cpu.pc, trap);
          return;
        }
        _ => {}
      }
    }
  }
}
