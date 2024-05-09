//! The emulator module represents an entire computer.

use crate::cpu::Cpu;
use crate::exception::Trap;
use crate::tui;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  prelude::*,
  widgets::{block::*, *},
};
use std::io;
/// The emulator to hold a CPU.
pub struct Emulator {
  /// The CPU which is the core implementation of this emulator.
  pub cpu: Cpu,

  exit: bool,
}

impl Emulator {
  /// Constructor for an emulator.
  pub fn new() -> Emulator {
    Self {
      cpu: Cpu::new(),
      exit: false,
    }
  }

  /// Reset CPU state.
  pub fn reset(&mut self) {
    self.cpu.reset()
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
      .constraints(vec![
        Constraint::Percentage(50), 
        Constraint::Percentage(50)
      ])
      .split(layout_vertical[1]);

    // render
    let pc = self.cpu.pc;
    let disasm = self.cpu.inst.disassemble(pc);
    frame.render_widget(
      Paragraph::new(format!("pc: {:#x}, inst: {}", pc, disasm))
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
      Paragraph::new(format!("[cpu] record: true, pc=0x8000000, inst=00000417",))
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
      Paragraph::new(format!("[dut] commit: true, pc=0x8000000, inst=00000417",))
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
      Paragraph::new(format!(
        // running or stop or quit
        "running"
      ))
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

  /// Start executing the emulator.
  pub fn start(&mut self, terminal: &mut tui::Tui) {
    while !self.exit {
      // let pc = self.cpu.pc;
      let trap = self.execute();
      // println!("=================================");
      // println!("pc: {:#x}, inst: {}", pc, self.cpu.inst.disassemble(pc));
      // println!("{}", self.cpu.gpr.to_string());
      // println!("{}", self.cpu.csr.to_string());
      terminal.draw(|frame| self.render_frame(frame)).unwrap();
      self.handle_events().unwrap();

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
