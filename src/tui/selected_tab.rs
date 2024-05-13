use ratatui::{prelude::*, style::palette::tailwind, widgets::*};
use std::{collections::VecDeque, fmt};
use strum::{Display, EnumIter, FromRepr};

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

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum SelectedTabEnum {
  #[default]
  #[strum(to_string = "Difftest")]
  Difftest,
  #[strum(to_string = "Trace")]
  Trace,
}

impl SelectedTabEnum {
  pub fn title(self) -> Line<'static> {
    format!("  {self}  ")
      .fg(tailwind::SLATE.c200)
      .bg(self.palette().c900)
      .into()
  }

  /// Get the previous tab, if there is no previous tab return the current tab.
  pub fn previous(self) -> Self {
    let current_index: usize = self as usize;
    let previous_index = current_index.saturating_sub(1);
    Self::from_repr(previous_index).unwrap_or(self)
  }

  /// Get the next tab, if there is no next tab return the current tab.
  pub fn next(self) -> Self {
    let current_index = self as usize;
    let next_index = current_index.saturating_add(1);
    Self::from_repr(next_index).unwrap_or(self)
  }

  pub const fn palette(self) -> tailwind::Palette {
    match self {
      Self::Difftest => tailwind::INDIGO,
      Self::Trace => tailwind::EMERALD,
    }
  }
}

#[derive(Clone)]
pub struct SelectedTab {
  pub diff: DiffBuffer,
  pub trace: TraceBuffer,
  pub state: SelectedTabEnum,
}

impl SelectedTab {
  pub fn new() -> Self {
    SelectedTab {
      diff: DiffBuffer::new(),
      trace: TraceBuffer::new(),
      state: SelectedTabEnum::default(),
    }
  }
}

impl Widget for SelectedTab {
  fn render(self, area: Rect, buf: &mut Buffer) {
    // in a real app these might be separate widgets
    match self.state {
      SelectedTabEnum::Difftest => self.render_difftest(area, buf),
      SelectedTabEnum::Trace => self.render_trace(area, buf),
    }
  }
}

impl SelectedTab {
  fn render_trace(self, area: Rect, buf: &mut Buffer) {
    // layout
    let layout_vertical = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
      .split(area);

    let layout_horizontal = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
      .split(layout_vertical[1]);

    Paragraph::new(self.trace.itrace.to_string())
      .block(
        Block::bordered()
          .title("Instruction Trace")
          .title_alignment(Alignment::Center)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Cyan))
      .left_aligned()
      .render(layout_vertical[0], buf);

    Paragraph::new(self.trace.mtrace[0].to_string())
      .block(
        Block::bordered()
          .title("CPU Memory Trace")
          .title_alignment(Alignment::Center)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Cyan))
      .left_aligned()
      .render(layout_horizontal[0], buf);

    Paragraph::new(self.trace.mtrace[1].to_string())
      .block(
        Block::bordered()
          .title("DUT Memory Trace")
          .title_alignment(Alignment::Center)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Cyan))
      .left_aligned()
      .render(layout_horizontal[1], buf);

      // TODO: Funtion trace
    // Paragraph::new(self.trace.ftrace.to_string())
    //   .block(
    //     Block::bordered()
    //       .title("Function Trace")
    //       .title_alignment(Alignment::Center)
    //       .border_type(BorderType::Rounded),
    //   )
    //   .style(Style::default().fg(Color::Cyan))
    //   .left_aligned()
    //   .render(layout_vertical[2], buf);
  }

  fn render_difftest(self, area: Rect, buf: &mut Buffer) {
    // layout
    let layout_vertical = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
      .split(area);

    let layout_horizontal = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
      .split(layout_vertical[0]);

    Paragraph::new(self.diff.cpu.to_string())
      .block(
        Block::bordered()
          .title("CPU")
          .title_alignment(Alignment::Left)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Cyan))
      .left_aligned()
      .render(layout_horizontal[0], buf);

    Paragraph::new(self.diff.dut.to_string())
      .block(
        Block::bordered()
          .title("DUT")
          .title_alignment(Alignment::Left)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Cyan))
      .left_aligned()
      .render(layout_horizontal[1], buf);

    Paragraph::new(self.diff.diff.to_string())
      .block(
        Block::bordered()
          .title("Difftest Status")
          .title_alignment(Alignment::Center)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Cyan))
      .centered()
      .render(layout_vertical[1], buf)
  }
}
