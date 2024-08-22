use ratatui::{buffer::Buffer, layout::Rect, Frame};

pub trait Component {
    type Action;

    fn render(&self, frame: &mut Frame, area: Rect);
    fn update(&mut self, action: Self::Action);
}

mod merge;