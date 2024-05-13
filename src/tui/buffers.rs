use std::{collections::VecDeque, fmt};

const CPU_BUFFER_SIZE: usize = 20;
const DUT_BUFFER_SIZE: usize = 20;
const DIFF_BUFFER_SIZE: usize = 5;
const INST_BUFFER_SIZE: usize = 5;
const MEMORY_BUFFER_SIZE: usize = 20;
const FUNCTION_BUFFER_SIZE: usize = 10;

#[derive(Clone)]
pub struct InfoBuffer {
  info: VecDeque<String>,
  size: usize,
}

impl InfoBuffer {
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

impl fmt::Display for InfoBuffer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for info in &self.info {
      write!(f, "{}\n", info)?;
    }
    Ok(())
  }
}
#[derive(Clone)]
pub struct DiffBuffer {
  pub cpu: InfoBuffer,
  pub dut: InfoBuffer,
  pub diff: InfoBuffer,
}

impl DiffBuffer {
  pub fn new() -> Self {
    DiffBuffer {
      cpu: InfoBuffer::new(CPU_BUFFER_SIZE),
      dut: InfoBuffer::new(DUT_BUFFER_SIZE),
      diff: InfoBuffer::new(DIFF_BUFFER_SIZE),
    }
  }
}

#[derive(Clone)]
pub struct TraceBuffer {
  pub itrace: InfoBuffer,
  pub mtrace: Vec<InfoBuffer>,
  pub ftrace: InfoBuffer,
}

impl TraceBuffer {
  pub fn new() -> Self {
    TraceBuffer {
      itrace: InfoBuffer::new(INST_BUFFER_SIZE),
      mtrace: vec![InfoBuffer::new(MEMORY_BUFFER_SIZE), InfoBuffer::new(MEMORY_BUFFER_SIZE)],
      ftrace: InfoBuffer::new(FUNCTION_BUFFER_SIZE),
    }
  }
}