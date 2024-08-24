use ratatui::{buffer::Buffer, layout::Rect, Frame};

pub trait Renderable {
    fn render(&self, frame: &mut Frame, area: Rect);
}

mod merge;
mod switch;
mod app;
