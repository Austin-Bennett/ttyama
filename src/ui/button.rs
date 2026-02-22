use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Alignment, Color};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::ttyama_app::TTYama;
use crate::ui::{InputContext, UIObject};

pub struct Button {
    area: Rect,
    focused: bool,
    label: String,
    func: Box<dyn FnMut(&mut TTYama)>,
}

impl Button {
    pub fn new<F>(area: Rect, label: impl AsRef<str>, func: F) -> Self
    where F: FnMut(&mut TTYama) + 'static {
        Self{
            area,
            label: label.as_ref().to_string(),
            focused: false,
            func: Box::new(func)
        }
    }
    
    pub fn set_label(&mut self, l: String) {
        self.label = l;
    }
}

impl UIObject for Button {
    fn get_area(&self) -> Rect {
        self.area
    }

    fn render(&self, frame: &mut Frame) {
        let style = if self.focused {
            Style::default().fg(Color::White).bg(Color::Gray)
        } else {
            Style::default().fg(Color::Gray).bg(Color::Black)
        };

        frame.render_widget(
            Paragraph::new(self.label.as_str())
                .style(style)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL)),
            self.area
        );
    }

    fn set_focused(&mut self, v: bool) {
        self.focused = v;
    }

    fn is_focused(&self) -> bool {
        self.focused
    }

    fn handle_input(&mut self, event: KeyEvent, context: &mut TTYama) {
        if event.code == KeyCode::Enter && event.modifiers.is_empty() {
            (self.func)(context)
        }
    }
}