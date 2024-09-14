use std::sync::{Arc, Mutex};

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Layout, Rect},
    Frame,
};

use super::{editable_text::{self, EditableText}, list::SelectableList, object_display::ObjectDisplay, Renderable};
use crate::ir::{ObjectKind, Project};

pub enum Focus {
    List,
    Inspect,
    Open,
}

pub struct ProjectMenu<'a> {
    items: [SelectableList<'a, String>; 3],
    tab: ObjectKind,

    focus: Focus,

    inspect: ObjectDisplay,
    project: Arc<Mutex<Project>>,
}

impl<'a> Renderable for ProjectMenu<'a> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let layout = Layout::new(
            ratatui::layout::Direction::Horizontal,
            Constraint::from_percentages([50, 50]),
        )
        .split(area);

        self.items[self.tab as usize].render(frame, layout[0]);
    }
}

impl<'a> ProjectMenu<'a> {
    pub fn new(project: Arc<Mutex<Project>>) -> Self {
        Self {
            items: core::array::from_fn(|_| SelectableList::new()),
            tab: ObjectKind::Functions,
            focus: Focus::List,
            project: project.clone(),
            inspect: ObjectDisplay::new(project),
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match self.focus {
            Focus::List => match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Char('1')) => self.update_tab(ObjectKind::Types),
                (KeyModifiers::NONE, KeyCode::Char('2')) => self.update_tab(ObjectKind::Functions),
                (KeyModifiers::NONE, KeyCode::Char('3')) => self.update_tab(ObjectKind::Globals),
                _ => self.items[self.tab as usize].process_key(key)
            },
            Focus::Inspect => {},
            Focus::Open => {

            }
        }
    }
    
    pub fn update(&mut self, action: Event) {
        match action {
            Event::FocusGained => {}
            Event::FocusLost => todo!(),
            Event::Key(k) => {
                if k.kind == KeyEventKind::Release {
                    return;
                }

                self.handle_key(k)
            }
            Event::Mouse(_) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => {}
        }
    }

    fn update_tab(&mut self, tab: ObjectKind) {
        let length = self.items[self.tab as usize].len();
    }

    pub fn apply_project(&mut self, source: &Project) {
        for pair in source.types.iter() {
            self.items[ObjectKind::Types as usize].push(pair.0.clone());
        }

        for pair in source.globals.iter() {
            self.items[ObjectKind::Globals as usize].push(pair.0.clone());
        }

        for pair in source.functions.iter() {
            self.items[ObjectKind::Functions as usize].push(pair.0.clone());
        }
    }
}
