use std::sync::{Arc, Mutex};

use ratatui::text::Text;

use crate::ir::{Function, Global, Project, Type};

use super::Renderable;

pub enum Object {
    Function(Function),
    Global(Global),
    Type(Type),
}
pub struct ObjectDisplay {
    pub key: String,
    pub object: Object
}

impl Renderable for ObjectDisplay {
    fn render(&self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let buffer = match &self.object {
            Object::Function(f) => serde_json::to_string_pretty(f),
            Object::Global(g) => serde_json::to_string_pretty(g),
            Object::Type(t) => serde_json::to_string_pretty(t),
        }.unwrap();

        let text = Text::from(buffer);
        
        frame.render_widget(text, area);
    }
}

impl ObjectDisplay {
    pub fn new(object: Object) -> Self {
        Self {
            key: String::new(),
            object
        }
    }
}