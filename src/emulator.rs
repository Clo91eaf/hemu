//! The emulator module represents an entire computer.

use core::fmt;
use tracing::{error, info, trace};

use crate::cpu::Cpu;
use crate::dut::Dut;
use crate::exception::Trap;
use crate::tui::{Tui, UI};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

#[derive(Default, Clone, Copy)]
pub struct GprInfo {
  pub commit: bool,
  pub pc: u64,
  pub wnum: u8,
  pub wdata: u64,
}

impl GprInfo {
  pub fn new(commit: bool, pc: u64, wnum: u8, wdata: u64) -> Self {
    GprInfo {
      commit,
      pc,
      wnum,
      wdata,
    }
  }
}

impl PartialEq for GprInfo {
  fn eq(&self, other: &Self) -> bool {
    self.pc == other.pc && (self.wnum == 0 || (self.wnum == other.wnum && self.wdata == other.wdata))
  }
}

impl fmt::Display for GprInfo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "pc: {:#x}, wnum: {}, wdata: {:#x}", self.pc, self.wnum, self.wdata)
  }
}

#[derive(Default, Clone, Copy)]
pub struct MemInfo {
  pub wen: bool,
  pub addr: u32,
  pub data: u64,
}

impl MemInfo {
  pub fn new(wen: bool, addr: u32, data: u64) -> Self {
    MemInfo { wen, addr, data }
  }
}

impl fmt::Display for MemInfo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "waddr: {:#x}, wdata: {:#x}", self.addr, self.data)
  }
}

impl PartialEq for MemInfo {
  fn eq(&self, other: &Self) -> bool {
    self.wen == other.wen && self.addr == other.addr && self.data == other.data
  }
}

#[derive(Default, Clone, Copy)]
pub struct DebugInfo {
  gpr: GprInfo,
  mem: MemInfo,
}

impl PartialEq for DebugInfo {
  fn eq(&self, other: &Self) -> bool {
    self.gpr == other.gpr && self.mem == other.mem
  }
}

impl fmt::Display for DebugInfo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "gpr: {}, mem: {}", self.gpr, self.mem)
  }
}

impl Eq for DebugInfo {}

impl DebugInfo {
  pub fn new(gpr: GprInfo, mem: MemInfo) -> Self {
    DebugInfo { gpr, mem }
  }
}

/// The emulator to hold a CPU.
pub struct Emulator {
  /// The CPU which is the core implementation of this emulator.
  pub cpu: Cpu,

  /// The DUT which is the peripheral devices of this emulator.
  pub dut: Option<Dut>,

  /// UI
  ui: UI,
}

impl Emulator {
  /// Constructor for an emulator.
  pub fn new(trace: bool, diff: bool) -> Emulator {
    let dut = if diff { Some(Dut::new(trace)) } else { None };
    Self {
      cpu: Cpu::new(),
      dut,
      ui: UI::new(),
    }
  }

  /// Reset CPU and DUT state.
  pub fn reset(&mut self) {
    self.cpu.reset();
  }

  fn exit(&mut self) {
    self.ui.cmd.exit = true;
  }

  fn quit(&mut self, terminal: &mut Tui) {
    while !self.ui.cmd.exit {
      terminal
        .draw(|frame| frame.render_widget(&self.ui, frame.size()))
        .unwrap();
      self.handle_events().unwrap();
    }
  }

  fn r#continue(&mut self) {
    self.ui.cmd.r#continue = true;
  }

  /// Set binary data to the beginning of the DRAM from the emulator console.
  pub fn initialize_dram(&mut self, data: Vec<u8>) {
    self.cpu.bus.initialize_dram(data.clone());
    if let Some(ref mut dut) = self.dut {
      dut.bus.initialize_dram(data);
    }
  }

  /// Set binary data to the virtio disk from the emulator console.
  pub fn initialize_disk(&mut self, data: Vec<u8>) {
    self.cpu.bus.initialize_disk(data.clone());
    if let Some(ref mut dut) = self.dut {
      dut.bus.initialize_dram(data);
    }
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

  fn handle_key_event(&mut self, key_event: KeyEvent) {
    match key_event.code {
      KeyCode::Char('q') | KeyCode::Char('Q') => self.exit(),
      KeyCode::Char('c') | KeyCode::Char('C') => self.r#continue(),
      KeyCode::Char('h') | KeyCode::Left => self.ui.previous_tab(),
      KeyCode::Char('l') | KeyCode::Right => self.ui.next_tab(),
      _ => {}
    }
  }

  /// updates the application's state based on user input
  fn handle_events(&mut self) -> io::Result<()> {
    match event::read()? {
      // it's important to check that the event is a key press event as
      // crossterm also emits key release and repeat events on Windows.
      Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self.handle_key_event(key_event),
      _ => {}
    };
    Ok(())
  }

  /// Start executing the emulator without difftest.
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

  fn cpu_exec(&mut self) -> (DebugInfo, Trap) {
    let pc = self.cpu.pc;
    let trap = self.execute();
    let gpr_info = match self.cpu.gpr.record {
      Some((wnum, wdata)) => GprInfo::new(true, pc, wnum, wdata),
      None => GprInfo::new(true, pc, 0, 0),
    };
    let mem_info = match self.cpu.bus.record {
      Some((addr, data)) => MemInfo::new(true, addr, data),
      None => MemInfo::new(false, 0, 0),
    };
    let cpu_diff = DebugInfo::new(gpr_info, mem_info);

    return (cpu_diff, trap);
  }

  fn dut_exec(&mut self) -> DebugInfo {
    let dut = self.dut.as_mut().unwrap();

    loop {
      let (inst_sram, data_sram, debug_info) = dut.step(dut.inst, dut.data).unwrap();

      if data_sram.en {
        let p_addr = self
          .cpu
          .translate((data_sram.addr >> 3 << 3) as u64, crate::cpu::AccessType::Instruction)
          .unwrap();

        // The result of the read method can be `Exception::LoadAccessFault`. In fetch(), an error
        // should be `Exception::InstructionAccessFault`.
        dut.data = dut.bus.read(p_addr, crate::cpu::DOUBLEWORD).unwrap();
        trace!(
          "[dut] ticks: {}, data_sram: addr: {:#x}, data: {:#018x}",
          dut.ticks,
          data_sram.addr,
          dut.data
        );
      }

      if inst_sram.en && inst_sram.addr != 0 {
        let p_pc = self
          .cpu
          .translate(inst_sram.addr as u64, crate::cpu::AccessType::Instruction)
          .unwrap();

        // The result of the read method can be `Exception::LoadAccessFault`. In fetch(), an error
        // should be `Exception::InstructionAccessFault`.
        dut.inst = self.cpu.bus.read(p_pc, crate::cpu::WORD).unwrap() as u32;

        trace!(
          "[dut] ticks: {}, inst_sram: addr: {:#x}, inst: {:#018x}",
          dut.ticks,
          inst_sram.addr,
          dut.inst
        );
      }

      if debug_info.gpr.commit {
        return debug_info;
      }
    }
  }

  /// Start executing the emulator.
  pub fn start_diff(&mut self) {
    let mut last_diff = DebugInfo::default();
    loop {
      let (cpu_diff, trap) = self.cpu_exec();
      info!("[cpu] pc: {:#x}, inst: {}", cpu_diff.gpr.pc, self.cpu.inst);
      trace!("cpu_diff: {}", self.cpu.gpr);

      let dut_diff = self.dut_exec();
      if let Some(ref dut) = self.dut {
        let pc = dut.top.debug_pc();
        let wnum = dut.top.debug_rf_wnum();
        let wdata = dut.top.debug_rf_wdata();
        let ticks = dut.ticks;
        info!(
          "[dut] pc: {:#010x}, wnum: {} wdata: {:#018x} ticks: {}",
          pc, wnum, wdata, ticks
        );
      }

      match trap {
        Trap::Fatal => {
          info!("[cpu] fatal pc: {:#x}, trap {:#?}", self.cpu.pc, trap);
          return;
        }
        _ => {}
      }

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

  pub fn ui_update(&mut self, cpu_diff: Option<DebugInfo>, dut_diff: Option<DebugInfo>, trap: bool) {
    match trap {
      true => {
        self.ui.selected_tab.diff.diff.clear();

        self
          .ui
          .selected_tab
          .diff
          .diff
          .push(format!("fatal pc: {:#x}, trap {:#?}", self.cpu.pc, trap));

        return;
      }
      false => {}
    }

    // difftest ui dut inst
    if let Some(ref dut) = self.dut {
      let pc = dut.top.debug_pc();
      let wnum = dut.top.debug_rf_wnum();
      let wdata = dut.top.debug_rf_wdata();
      let ticks = dut.ticks;
      self.ui.selected_tab.diff.dut.push(format!(
        "pc: {:#010x}, wnum: {:02} wdata: {:#018x} ticks: {}",
        pc, wnum, wdata, ticks
      ));
    }

    // trace ui mtrace
    if let Some(cpu_diff) = cpu_diff {
      if cpu_diff.mem.wen {
        self.ui.selected_tab.trace.mtrace[0].push(format!("pc: {:#010x}, {}", self.cpu.pc, cpu_diff.mem.to_string()));
      }

      // difftest ui cpu inst
      self
        .ui
        .selected_tab
        .diff
        .cpu
        .push(format!("pc: {:#x}, inst: {}", cpu_diff.gpr.pc, self.cpu.inst));

      // trace ui itrace
      self
        .ui
        .selected_tab
        .trace
        .itrace
        .push(format!("pc: {:#x}, inst: {}", cpu_diff.gpr.pc, self.cpu.inst));
    }

    // trace ui mtrace
    if let Some(dut_diff) = dut_diff {
      if dut_diff.mem.wen {
        self.ui.selected_tab.trace.mtrace[1].push(format!("pc: {:#010x}, {}", self.cpu.pc, dut_diff.mem.to_string()));
      }
    }
  }

  /// Start executing the emulator with difftest and tui.
  pub fn start_diff_tui(&mut self, terminal: &mut Tui) {
    self.ui.selected_tab.diff.diff.push("running".to_string());
    let mut last_diff = DebugInfo::default();

    while !self.ui.cmd.exit {
      // ================ cpu ====================
      let (cpu_diff, trap) = self.cpu_exec();

      match trap {
        Trap::Fatal => {
          self.ui_update(None, None, true);
          self.quit(terminal);
          return;
        }
        _ => {}
      }

      let dut_diff = self.dut_exec();

      // ==================== diff ====================
      if cpu_diff != dut_diff {
        self.ui.selected_tab.diff.diff.clear();

        self
          .ui
          .selected_tab
          .diff
          .diff
          .push("difftest failed. press 'q' or 'Q' to quit. ".to_string());
        self.ui.selected_tab.diff.diff.push(format!("last: {}", last_diff));
        self.ui.selected_tab.diff.diff.push(format!("cpu : {}", cpu_diff));
        self.ui.selected_tab.diff.diff.push(format!("dut : {}", dut_diff));

        self.quit(terminal);

        return;
      }
      last_diff = cpu_diff;

      self.ui_update(Some(cpu_diff), Some(dut_diff), false);
      // tui
      terminal
        .draw(|frame| frame.render_widget(&self.ui, frame.size()))
        .unwrap();
      if !self.ui.cmd.r#continue {
        self.handle_events().unwrap();
      }
    }
  }
}
