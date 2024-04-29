//! The emulator module represents an entire computer.

use crate::cpu::Cpu;
use crate::dut::Dut;
use crate::exception::Trap;

/// The emulator to hold a CPU.
pub struct Emulator {
  /// The CPU which is the core implementation of this emulator.
  pub cpu: Cpu,

  /// The DUT which is the peripheral devices of this emulator.
  pub dut: Dut,
}

impl Emulator {
  /// Constructor for an emulator.
  pub fn new() -> Emulator {
    Self {
      cpu: Cpu::new(),
      dut: Dut::new(),
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

  fn execute(&mut self) -> Trap {
    // Run a cycle on peripheral devices.
    self.cpu.devices_increment();

    // Take an interrupt.
    match self.cpu.check_pending_interrupt() {
      Some(interrupt) => interrupt.take_trap(&mut self.cpu),
      None => {}
    }

    // Execute an instruction.
    match self.cpu.execute() {
      // Return a placeholder trap.
      Ok(_) => Trap::Requested,
      Err(exception) => exception.take_trap(&mut self.cpu),
    }
  }

  /// Start executing the emulator.
  pub fn start(&mut self) {
    loop {
      // let pc = self.cpu.pc;
      let trap = self.execute();
      // println!("=================================");
      // println!("pc: {:#x}, inst: {}", pc, self.cpu.inst.disassemble(pc));
      // println!("{}", self.cpu.gpr.to_string());
      // println!("{}", self.cpu.csr.to_string());

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
