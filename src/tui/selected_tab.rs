use ratatui::{prelude::*, style::palette::tailwind, widgets::*};
use strum::{Display, EnumIter, FromRepr};
use super::buffers::{DiffBuffer, TraceBuffer};

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

    Paragraph::new(self.trace.mtrace.to_string())
      .block(
        Block::bordered()
          .title("CPU Memory Trace")
          .title_alignment(Alignment::Center)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Cyan))
      .left_aligned()
      .render(layout_vertical[1], buf);

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
