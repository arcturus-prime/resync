use ratatui::{crossterm::event::Event, layout::Rect, Frame};

pub mod project;
pub mod editable_text;
pub mod list;
pub mod object_display;

pub trait Renderable {
    fn render(&self, frame: &mut Frame, area: Rect);
}

pub trait Menu: Renderable {
    fn update(&mut self, action: Event);
}