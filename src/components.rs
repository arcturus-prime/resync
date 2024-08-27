use ratatui::{layout::Rect, Frame};

pub mod menus;
pub mod path;

pub trait Component {
    type Action;

    fn update(&mut self, action: Self::Action);
    fn render(&self, frame: &mut Frame, area: Rect);
}
