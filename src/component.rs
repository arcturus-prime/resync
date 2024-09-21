use ratatui::{crossterm::event::Event, layout::Rect, Frame};

pub mod project_display;
pub mod editable_text;
pub mod selectable_list;
pub mod object_display;
pub mod merge_view;

pub trait Renderable {
    fn render(&self, frame: &mut Frame, area: Rect);
}