//! The emulator module represents an entire computer.

use core::fmt;
use tracing::{error, info, trace};

use crate::cpu::Cpu;
use crate::dut::Dut;
use crate::exception::Trap;

#[derive(Default)]
pub struct DebugInfo {
  pub commit: bool,
  pub pc: u64,
  pub wnum: u8,
  pub wdata: u64,
}

impl fmt::Display for DebugInfo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "pc: {:#x}, wnum: {}, wdata: {:#x}", self.pc, self.wnum, self.wdata)
  }
}

impl PartialEq for DebugInfo {
  fn eq(&self, other: &Self) -> bool {
    self.commit == other.commit && self.pc == other.pc && self.wnum == other.wnum && self.wdata == other.wdata
  }
}

impl Eq for DebugInfo {}

impl DebugInfo {
  pub fn new(commit: bool, pc: u64, wnum: u8, wdata: u64) -> Self {
    DebugInfo {
      commit,
      pc,
      wnum,
      wdata,
    }
  }
}

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

  /// Reset CPU and DUT state.
  pub fn reset(&mut self) {
    self.cpu.reset();
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

  fn dut_step(&mut self) -> DebugInfo {
    let mut ticks = 20;
    loop {
      let (inst_sram, data_sram, debug_info) = self.dut.step(self.dut.inst, self.dut.data).unwrap();

      if data_sram.en {
        let p_addr = self
          .cpu
          .translate(data_sram.addr as u64, crate::cpu::AccessType::Instruction)
          .unwrap();

        // The result of the read method can be `Exception::LoadAccessFault`. In fetch(), an error
        // should be `Exception::InstructionAccessFault`.
        self.dut.data = self.cpu.bus.read(p_addr, crate::cpu::DOUBLEWORD).unwrap();
        trace!(
          "[dut] ticks: {}, data_sram: addr: {:#x}, data: {:#018x}",
          self.dut.ticks,
          data_sram.addr,
          self.dut.data
        );
      }

      if inst_sram.en {
        let p_pc = self
          .cpu
          .translate(inst_sram.addr as u64, crate::cpu::AccessType::Instruction)
          .unwrap();

        // The result of the read method can be `Exception::LoadAccessFault`. In fetch(), an error
        // should be `Exception::InstructionAccessFault`.
        self.dut.inst = self.cpu.bus.read(p_pc, crate::cpu::WORD).unwrap() as u32;

        trace!(
          "[dut] ticks: {}, inst_sram: addr: {:#x}, inst: {:#018x}",
          self.dut.ticks,
          inst_sram.addr,
          self.dut.inst
        );
      }

      if debug_info.commit {
        return debug_info;
      }

      ticks -= 1;
      if ticks == 0 {
        panic!("timeout");
      }
    }
  }

  pub fn start(&mut self) {
    loop {
      let pc = self.cpu.pc;
      let trap = self.execute();
      info!("pc: {:#x}, inst: {}", pc, self.cpu.inst);

      match trap {
        Trap::Fatal => {
          info!("fatal pc: {:#x}, trap {:#?}", self.cpu.pc, trap);
          return;
        }
        _ => {}
      }
    }
  }

  /// Start executing the emulator.
  pub fn start_diff(&mut self) {
    let mut last_diff = DebugInfo::default();
    loop {
      // ================ cpu ====================
      let cpu_diff;
      loop {
        let pc = self.cpu.pc;
        let trap = self.execute();

        match trap {
          Trap::Fatal => {
            info!("[cpu] fatal pc: {:#x}, trap {:#?}", self.cpu.pc, trap);
            return;
          }
          _ => {}
        }

        match self.cpu.gpr.record {
          Some((wnum, wdata)) => {
            cpu_diff = DebugInfo::new(true, pc, wnum, wdata);
            info!("[cpu] record: true, pc: {:#x}, inst: {}", pc, self.cpu.inst);
            break;
          }
          None => {
            info!("[cpu] record: false, pc: {:#x}, inst: {}", pc, self.cpu.inst);
          }
        }
      }

      let dut_diff = self.dut_step();

      // ==================== diff ====================
      if cpu_diff != dut_diff {
        error!("difftest failed");
        error!("last: {}", last_diff);
        error!("cpu : {}", cpu_diff);
        error!("dut : {}", dut_diff);
        return;
      }
      last_diff = cpu_diff;
    }
  }
}
