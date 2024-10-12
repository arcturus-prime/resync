use ratatui::{prelude::Rect, Frame};

use super::{object_display::ObjectDisplay, Renderable};

pub struct MergeView {
    pub display_a: ObjectDisplay,
    pub display_b: ObjectDisplay,
}

impl Renderable<()> for MergeView {
    fn render(&self, frame: &mut Frame, area: Rect, _: ()) {
        todo!()
    }
}