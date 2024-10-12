use ratatui::{layout::Rect, Frame};

pub mod editable_text;
pub mod object_display;
pub mod project_view;
pub mod merge_view;

pub trait Renderable<T> {
    fn render(&self, frame: &mut Frame, area: Rect, data: T);
}