use std::sync::{Arc, Mutex};

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Layout, Rect},
    Frame,
};

use super::{editable_text::{self, EditableText}, selectable_list::SelectableList, object_display::ObjectDisplay, Renderable};
use crate::ir::Project;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Tab {
    Types = 0,
    Functions,
    Globals,
}
pub struct ProjectDisplay<'a> {
    lists: [SelectableList<'a, String>; 3],
    pub tab: Tab,
}

impl<'a> Renderable for ProjectDisplay<'a> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        self.lists[self.tab as usize].render(frame, area);
    }
}

impl<'a> ProjectDisplay<'a> {
    pub fn new(project: &Project) -> Self {
        let mut s = Self {
            lists: core::array::from_fn(|_| SelectableList::new()),
            tab: Tab::Functions,
        };

        for (k, _) in project.types.iter() {
            s.lists[Tab::Types as usize].push(k.clone());
        }

        for (k, _) in project.globals.iter() {
            s.lists[Tab::Globals as usize].push(k.clone());
        }

        for (k, _) in project.functions.iter() {
            s.lists[Tab::Functions as usize].push(k.clone());
        }

        s
    }

    pub fn get_current(&mut self) -> &mut SelectableList<'a, String> {
        &mut self.lists[self.tab as usize]
    }
}
