use std::io::{self, stdout, Stdout};

use crossterm::{execute, terminal::*};
use ratatui::{prelude::*, widgets::*};

mod selected_tab;
use selected_tab::{SelectedTab, SelectedTabEnum};
use strum::IntoEnumIterator;

mod buffers;

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

#[derive(Default)]
pub struct UICommand {
  pub r#continue: bool,
  pub exit: bool,
}

pub struct UI {
  pub cmd: UICommand,
  pub selected_tab: SelectedTab,
}

impl UI {
  pub fn new() -> Self {
    Self {
      cmd: UICommand::default(),
      selected_tab: SelectedTab::new(),
    }
  }

  pub fn next_tab(&mut self) {
    self.selected_tab.state = self.selected_tab.state.next();
  }

  pub fn previous_tab(&mut self) {
    self.selected_tab.state = self.selected_tab.state.previous();
  }

  fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
    let titles = SelectedTabEnum::iter().map(SelectedTabEnum::title);
    let highlight_style = (Color::default(), self.selected_tab.state.palette().c700);
    let selected_tab_index = self.selected_tab.state as usize;
    Tabs::new(titles)
      .highlight_style(highlight_style)
      .select(selected_tab_index)
      .padding("", "")
      .divider(" ")
      .render(area, buf);
  }
}

impl Widget for &UI {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let layout_vertical = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![Constraint::Percentage(5), Constraint::Percentage(95)])
      .split(area);

    self.render_tabs(layout_vertical[0], buf);
    self.selected_tab.clone().render(layout_vertical[1], buf);
  }
}
