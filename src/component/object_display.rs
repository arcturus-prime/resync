use crate::ir::ObjectKind;

use super::Renderable;

pub struct ObjectDisplay {
    key: String,
    kind: ObjectKind,
}

impl Renderable for ObjectDisplay {
    fn render(&self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        todo!()
    }
}

impl ObjectDisplay {
    pub fn new() -> Self {
        Self {
            key: String::new(),
            kind: ObjectKind::Functions,
        }
    }
}