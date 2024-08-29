use ratatui::{layout::Rect, Frame};

pub mod project;
pub mod editable_text;
pub mod list;

pub trait Component {
    type Action;

    fn update(&mut self, action: Self::Action);
    fn render(&self, frame: &mut Frame, area: Rect);
}
