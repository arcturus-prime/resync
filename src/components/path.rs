use ratatui::crossterm::event::{self, Event, KeyCode};

use super::Component;

pub struct PathPrompt {
    buffer: String,
    index: usize,
}

impl Component for PathPrompt {
    type Action = KeyCode;

    fn update(&mut self, action: Self::Action) {
        match action {
            KeyCode::Backspace => todo!(),
            KeyCode::Left => todo!(),
            KeyCode::Right => todo!(),
            KeyCode::Char(c) => self.buffer.push(c),
            _ => {} 
        }
    }

    fn render(&self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        todo!()
    }
}