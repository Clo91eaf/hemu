use ratatui::{prelude::*, style::palette::tailwind, widgets::*};
use std::{collections::VecDeque, fmt};
use strum::{Display, EnumIter, FromRepr};

const INST_BUFFER_SIZE: usize = 10;
const CPU_BUFFER_SIZE: usize = 10;
const DUT_BUFFER_SIZE: usize = 10;
const DIFF_BUFFER_SIZE: usize = 5;

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
pub struct UIBuffer {
  pub inst: InfoBuffer,
  pub cpu: InfoBuffer,
  pub dut: InfoBuffer,
  pub diff: InfoBuffer,
}

impl UIBuffer {
  pub fn new() -> Self {
    UIBuffer {
      inst: InfoBuffer::new(INST_BUFFER_SIZE),
      cpu: InfoBuffer::new(CPU_BUFFER_SIZE),
      dut: InfoBuffer::new(DUT_BUFFER_SIZE),
      diff: InfoBuffer::new(DIFF_BUFFER_SIZE),
    }
  }
}
#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum SelectedTabEnum {
  #[default]
  #[strum(to_string = "Main")]
  Main,
  #[strum(to_string = "Trace")]
  Trace,
  #[strum(to_string = "Difftest")]
  Difftest,
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
      Self::Main => tailwind::BLUE,
      Self::Trace => tailwind::EMERALD,
      Self::Difftest => tailwind::INDIGO,
    }
  }
}

#[derive(Clone)]
pub struct SelectedTab {
  pub diff_buffer: UIBuffer,
  pub state: SelectedTabEnum,
}

impl SelectedTab {
  pub fn new() -> Self {
    SelectedTab {
      diff_buffer: UIBuffer::new(),
      state: SelectedTabEnum::default(),
    }
  }
}

impl Widget for SelectedTab {
  fn render(self, area: Rect, buf: &mut Buffer) {
    // in a real app these might be separate widgets
    match self.state {
      SelectedTabEnum::Main => self.render_main(area, buf),
      SelectedTabEnum::Trace => self.render_trace(area, buf),
      SelectedTabEnum::Difftest => self.render_difftest(area, buf),
    }
  }
}

impl SelectedTab {
  fn render_main(self, area: Rect, buf: &mut Buffer) {
    Paragraph::new("Welcome to the Ratatui tabs example!")
      .block(self.block())
      .render(area, buf);
  }

  fn render_trace(self, area: Rect, buf: &mut Buffer) {
    Paragraph::new("Look! I'm different than others!")
      .block(self.block())
      .render(area, buf);
  }

  fn render_difftest(self, area: Rect, buf: &mut Buffer) {
    // layout
    let layout_vertical = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
        Constraint::Percentage(40),
        Constraint::Percentage(40),
        Constraint::Percentage(20),
      ])
      .split(area);

    let layout_horizontal = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
      .split(layout_vertical[1]);

    // render_frame
    Paragraph::new(self.diff_buffer.inst.to_string())
      .block(
        Block::bordered()
          .title("Instructions")
          .title_alignment(Alignment::Left)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Cyan))
      .left_aligned()
      .render(layout_vertical[0], buf);

    Paragraph::new(self.diff_buffer.cpu.to_string())
      .block(
        Block::bordered()
          .title("CPU")
          .title_alignment(Alignment::Left)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Cyan))
      .left_aligned()
      .render(layout_horizontal[0], buf);

    Paragraph::new(self.diff_buffer.dut.to_string())
      .block(
        Block::bordered()
          .title("DUT")
          .title_alignment(Alignment::Left)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Cyan))
      .left_aligned()
      .render(layout_horizontal[1], buf);

    Paragraph::new(self.diff_buffer.diff.to_string())
      .block(
        Block::bordered()
          .title("Difftest Status")
          .title_alignment(Alignment::Center)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Cyan))
      .centered()
      .render(layout_vertical[2], buf)
  }

  /// A block surrounding the tab's content
  fn block(self) -> Block<'static> {
    Block::bordered()
      .border_set(symbols::border::PROPORTIONAL_TALL)
      .padding(Padding::horizontal(1))
      .border_style(self.state.palette().c700)
  }
}
