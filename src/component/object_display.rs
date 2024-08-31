use crate::ir::ObjectKind;

use super::Component;


pub struct ObjectDisplay {
    key: String,
    kind: ObjectKind,
}

pub enum Action {
    UpdateObject {
        key: String,
        kind: ObjectKind
    }
}

impl Component for ObjectDisplay {
    type Action = Action;

    fn update(&mut self, action: Self::Action) {
        todo!()
    }

    fn render(&self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        todo!()
    }
}