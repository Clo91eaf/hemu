use std::io::{self, stdout, Stdout};
use std::{collections::VecDeque, fmt};

use crossterm::{execute, terminal::*};
use ratatui::prelude::*;

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init() -> io::Result<Tui> {
  execute!(stdout(), EnterAlternateScreen)?;
  enable_raw_mode()?;
  Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
  execute!(stdout(), LeaveAlternateScreen)?;
  disable_raw_mode()?;
  Ok(())
}

const INST_BUFFER_SIZE: usize = 10;
const CPU_BUFFER_SIZE: usize = 10;
const DUT_BUFFER_SIZE: usize = 10;
const DIFF_BUFFER_SIZE: usize = 5;

#[derive(Default)]
pub struct Buffer {
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

  pub fn push(&mut self, info: String) {
    if self.info.len() >= self.size {
      self.info.pop_front();
    }
    self.info.push_back(info);
  }

  pub fn clear(&mut self) {
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
pub struct UIBuffer {
  pub inst: Buffer,
  pub cpu: Buffer,
  pub dut: Buffer,
  pub diff: Buffer,
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

pub struct UICommand {
  pub r#continue: bool,
  pub exit: bool,
}

pub struct UI {
  pub buffer: UIBuffer,
  pub cmd: UICommand,
}

impl UI {
  pub fn new() -> Self {
    Self {
      buffer: UIBuffer::new(),
      cmd: UICommand {
        r#continue: false,
        exit: false,
      },
    }
  }
}
