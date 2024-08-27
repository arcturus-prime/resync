use ratatui::{crossterm::event::{self, Event, KeyCode}, widgets::Block};

use super::Component;

pub struct PathPrompt {
    buffer: String,
    index: usize,
}

impl Component for PathPrompt {
    type Action = KeyCode;

    fn update(&mut self, action: Self::Action) {
        match action {
            KeyCode::Backspace => {
                self.buffer.remove(self.index);
            },
            KeyCode::Left => self.move_index_left(),
            KeyCode::Right => self.move_index_right(),
            KeyCode::Char(c) => self.buffer.insert(self.index, c),
            _ => {} 
        }
    }

    fn render(&self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let block = Block::new().title(self.buffer.clone());

        frame.render_widget(block, area);
    }
}

impl PathPrompt {
    fn move_index_left(&mut self) {
        if self.index == 0 {
            return
        }
        self.index -= 1
    }

    fn move_index_right(&mut self) {
        if self.index >= self.buffer.len() - 1 {
            return
        }
        self.index += 1
    }

    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            index: 0,
        }
    }
}