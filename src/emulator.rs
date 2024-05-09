//! The emulator module represents an entire computer.

use core::fmt;
use tracing::{error, info, trace};

use crate::cpu::Cpu;
use crate::dut::Dut;
use crate::exception::Trap;
use crate::tui;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  prelude::*,
  widgets::{block::*, *},
};
use std::{collections::VecDeque, io};

const INST_BUFFER_SIZE: usize = 10;
const CPU_BUFFER_SIZE: usize = 10;
const DUT_BUFFER_SIZE: usize = 10;
const DIFF_BUFFER_SIZE: usize = 5;

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

#[derive(Default)]
struct Buffer {
  info: VecDeque<String>,
  size: usize,
}

impl Buffer {
  fn new(size: usize) -> Self {
    Self {
      info: VecDeque::new(),
      size,
    }
  }

  fn push(&mut self, info: String) {
    if self.info.len() >= self.size {
      self.info.pop_front();
    }
    self.info.push_back(info);
  }

  fn clear(&mut self) {
    self.info.clear();
  }
}

impl fmt::Display for Buffer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for info in &self.info {
      write!(f, "{}\n", info)?;
    }
    Ok(())
  }
}

#[derive(Default)]
struct UIBuffer {
  inst: Buffer,
  cpu: Buffer,
  dut: Buffer,
  diff: Buffer,
}

impl UIBuffer {
  pub fn new() -> Self {
    UIBuffer {
      inst: Buffer::new(INST_BUFFER_SIZE),
      cpu: Buffer::new(CPU_BUFFER_SIZE),
      dut: Buffer::new(DUT_BUFFER_SIZE),
      diff: Buffer::new(DIFF_BUFFER_SIZE),
    }
  }
}

/// The emulator to hold a CPU.
pub struct Emulator {
  /// The CPU which is the core implementation of this emulator.
  pub cpu: Cpu,

  /// The DUT which is the peripheral devices of this emulator.
  pub dut: Dut,

  /// The flag to exit the emulator.
  exit: bool,

  /// UI information
  ui_buffer: UIBuffer,
}

impl Emulator {
  /// Constructor for an emulator.
  pub fn new() -> Emulator {
    Self {
      cpu: Cpu::new(),
      dut: Dut::new(),
      exit: false,
      ui_buffer: UIBuffer::new(),
    }
  }

  /// Reset CPU and DUT state.
  pub fn reset(&mut self) {
    self.cpu.reset();
  }

  fn exit(&mut self) {
    self.exit = true;
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

  fn render_frame(&self, frame: &mut Frame) {
    // layout
    let layout_vertical = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
        Constraint::Percentage(40),
        Constraint::Percentage(40),
        Constraint::Percentage(20),
      ])
      .split(frame.size());

    let layout_horizontal = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
      .split(layout_vertical[1]);

    // render
    frame.render_widget(
      Paragraph::new(self.ui_buffer.inst.to_string())
        .block(
          Block::bordered()
            .title("Instructions")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan))
        .left_aligned(),
      layout_vertical[0],
    );

    frame.render_widget(
      Paragraph::new(self.ui_buffer.cpu.to_string())
        .block(
          Block::bordered()
            .title("CPU")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan))
        .left_aligned(),
      layout_horizontal[0],
    );

    frame.render_widget(
      Paragraph::new(self.ui_buffer.dut.to_string())
        .block(
          Block::bordered()
            .title("DUT")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan))
        .left_aligned(),
      layout_horizontal[1],
    );

    frame.render_widget(
      Paragraph::new(self.ui_buffer.diff.to_string())
      .block(
        Block::bordered()
          .title("Difftest Status")
          .title_alignment(Alignment::Center)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Cyan))
      .centered(),
      layout_vertical[2],
    );
  }

  fn handle_key_event(&mut self, key_event: KeyEvent) {
    match key_event.code {
      KeyCode::Char('q') => self.exit(),
      KeyCode::Char('Q') => self.exit(),
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

      let dut_diff;

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
          dut_diff = debug_info;
          break;
        }
      }
      info!(
        "[dut] ticks: {} commit: {} pc: {:#010x} wnum: {} wdata: {:#018x}",
        self.dut.ticks,
        self.dut.top.debug_commit(),
        self.dut.top.debug_pc(),
        self.dut.top.debug_reg_wnum(),
        self.dut.top.debug_wdata()
      );

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

  /// Start executing the emulator with difftest and tui.
  pub fn start_diff_tui(&mut self, terminal: &mut tui::Tui) {
    self.ui_buffer.diff.push("running".to_string());
    let mut last_diff = DebugInfo::default();

    while !self.exit {
      // ================ cpu ====================
      let cpu_diff;
      loop {
        let pc = self.cpu.pc;
        let trap = self.execute();

        self
          .ui_buffer
          .inst
          .push(format!("pc: {:#x}, inst: {}", pc, self.cpu.inst));

        match trap {
          Trap::Fatal => {
            self
              .ui_buffer
              .diff
              .push(format!("fatal pc: {:#x}, trap {:#?}", self.cpu.pc, trap));
            return;
          }
          _ => {}
        }

        match self.cpu.gpr.record {
          Some((wnum, wdata)) => {
            cpu_diff = DebugInfo::new(true, pc, wnum, wdata);
            self
              .ui_buffer
              .cpu
              .push(format!("record: true, pc: {:#x}, inst: {}", pc, self.cpu.inst));
            break;
          }
          None => {
            self
              .ui_buffer
              .cpu
              .push(format!("record: false, pc: {:#x}, inst: {}", pc, self.cpu.inst));
          }
        }
      }

      let dut_diff;

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

          self.ui_buffer.dut.push(format!(
            "{}, data_sram: addr: {:#x}, data: {:#018x}",
            self.dut.ticks, data_sram.addr, self.dut.data
          ))
        }

        if inst_sram.en {
          let p_pc = self
            .cpu
            .translate(inst_sram.addr as u64, crate::cpu::AccessType::Instruction)
            .unwrap();

          // The result of the read method can be `Exception::LoadAccessFault`. In fetch(), an error
          // should be `Exception::InstructionAccessFault`.
          self.dut.inst = self.cpu.bus.read(p_pc, crate::cpu::WORD).unwrap() as u32;

          self.ui_buffer.dut.push(format!(
            "{}, inst_sram: pc: {:#x}, inst: {:#010x}",
            self.dut.ticks, inst_sram.addr, self.dut.inst
          ))
        }

        if debug_info.commit {
          dut_diff = debug_info;
          break;
        }
      }
      self.ui_buffer.dut.push(format!(
        "{}, pc: {:#010x} wnum: {} wdata: {:#018x}",
        self.dut.ticks,
        self.dut.top.debug_pc(),
        self.dut.top.debug_reg_wnum(),
        self.dut.top.debug_wdata()
      ));

      // ==================== diff ====================
      if cpu_diff != dut_diff {
        self.ui_buffer.diff.clear();

        self.ui_buffer.diff.push("difftest failed".to_string());
        self.ui_buffer.diff.push(format!("last: {}", last_diff));
        self.ui_buffer.diff.push(format!("cpu : {}", cpu_diff));
        self.ui_buffer.diff.push(format!("dut : {}", dut_diff));

        while !self.exit {
          terminal.draw(|frame| self.render_frame(frame)).unwrap();
          self.handle_events().unwrap();
        }

        return;
      }
      last_diff = cpu_diff;

      // tui
      terminal.draw(|frame| self.render_frame(frame)).unwrap();
      self.handle_events().unwrap();
    }
  }
}
