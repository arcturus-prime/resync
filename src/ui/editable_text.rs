use ratatui::{crossterm::{event::{KeyCode, KeyEvent}, style::Color}, style::Style, text::Text};

use super::Renderable;

pub struct EditableText {
    pub buffer: String,
    pub index: usize,
}

impl Renderable for EditableText {
    fn render(&self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let block = Text::styled(&self.buffer, Style::default().bg(Color::AnsiValue(235).into()));

        frame.render_widget(block, area);
    }
}

impl EditableText {
    pub fn move_index_left(&mut self) {
        if self.index == 0 {
            return
        }
        self.index -= 1
    }

    pub fn move_index_right(&mut self) {
        if self.index >= self.buffer.len() - 1 {
            return
        }
        self.index += 1
    }

    pub fn backspace(&mut self) {
        if self.index == 0 {
            return
        }

        if self.index == self.buffer.len() {
            self.buffer.pop();
            self.index -= 1;
            return
        }
        
        self.buffer.remove(self.index);
    }

    pub fn get(&self) -> &str {
        &self.buffer
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.index = 0
    }

    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            index: 0,
        }
    }
}
