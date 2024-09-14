use std::sync::{Arc, Mutex};

use crate::ir::{ObjectKind, Project};

use super::Renderable;

pub struct ObjectDisplay {
    key: String,
    kind: ObjectKind,

    project: Arc<Mutex<Project>>,
}

impl Renderable for ObjectDisplay {
    fn render(&self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        todo!()
    }
}

impl ObjectDisplay {
    pub fn new(project: Arc<Mutex<Project>>) -> Self {
        Self {
            key: String::new(),
            kind: ObjectKind::Functions,
            project
        }
    }
}