use crate::traits::Window;

use crossterm::event::{KeyCode, KeyEvent};
use std::env;

use ratatui::{
  layout::{Alignment, Rect},
  style::{Color, Modifier, Style},
  text::{Span, Text},
  widgets::{block::Title, Block, Borders, Padding, Paragraph},
  Frame
};

pub struct StatsWindow {
  pub input: String,

  is_active: bool
}

impl Window for StatsWindow {
  fn default() -> Self {
    let path = env::current_dir().expect("Error getting current path");

    Self {
      input: path.to_string_lossy().into_owned(),

      is_active: false
    }
  }

  fn is_active(&self) -> bool {
    self.is_active
  }

  fn handle_events(&mut self, key: KeyEvent) {
    match key.code {
      KeyCode::Char(c) => self.input.push(c),
      KeyCode::Backspace => { let _ = self.input.pop(); },
      _ => ()
    }
  }

  fn render(&self, frame: &mut Frame, area: Rect) {
    let block = Block::new()
      .borders(Borders::ALL)
      .border_style(Style::default().fg(self.get_border_color()))
      .title(Title::from("Results").alignment(Alignment::Center));

    let p = Paragraph::new("Stats")
      .block(block);

    frame.render_widget(p, area);
  }

}