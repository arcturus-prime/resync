use ratatui::{layout::Rect, Frame};

pub mod editable_text;
pub mod object_display;
pub mod menu;

pub trait Renderable {
    fn render(&self, frame: &mut Frame, area: Rect);
}