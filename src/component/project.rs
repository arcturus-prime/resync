use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Layout, Rect},
    Frame,
};

use super::{list::SelectableList, editable_text::EditableText, Component};
use crate::ir::Project;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Tab {
    Types = 0,
    Functions,
    Globals,
}
pub enum Focus {
    List,
    Inspect,
    Open,
}

pub struct ProjectView<'a> {
    items: [SelectableList<'a, String>; 3],
    tab: Tab,
    focus: Focus,
}

impl<'a> Component for ProjectView<'a> {
    type Action = Event;

    fn render(&self, frame: &mut Frame, area: Rect) {
        let list = &self.items[self.tab as usize];

        let layout = Layout::new(
            ratatui::layout::Direction::Horizontal,
            Constraint::from_percentages([50, 50]),
        )
        .split(area);

        list.render(frame, layout[0]);
    }

    fn update(&mut self, action: Self::Action) {
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
}

impl<'a> ProjectView<'a> {
    pub fn new() -> Self {
        Self {
            items: core::array::from_fn(|_| SelectableList::new()),
            tab: Tab::Functions,
            focus: Focus::List,
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match self.focus {
            Focus::List => match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Char('1')) => self.update_tab(Tab::Types),
                (KeyModifiers::NONE, KeyCode::Char('2')) => self.update_tab(Tab::Functions),
                (KeyModifiers::NONE, KeyCode::Char('3')) => self.update_tab(Tab::Globals),
                _ => self.items[self.tab as usize].update(key)
            },
            Focus::Inspect => {},
            Focus::Open => {

            }
        }
    }

    fn update_tab(&mut self, tab: Tab) {
        let length = self.items[self.tab as usize].len();
    }

    pub fn apply_project_diff(&mut self, source: &Project, dest: &Project) {
        for pair in source.types.iter() {
            if dest.types.contains_key(pair.0) {
                self.items[Tab::Types as usize].push(pair.0.clone());
            }
        }

        for pair in source.globals.iter() {
            if dest.globals.contains_key(pair.0) {
                self.items[Tab::Globals as usize].push(pair.0.clone());
            }
        }

        for pair in source.functions.iter() {
            if dest.functions.contains_key(pair.0) {
                self.items[Tab::Functions as usize].push(pair.0.clone());
            }
        }
    }

    pub fn apply_project(&mut self, source: &Project) {
        for pair in source.types.iter() {
            self.items[Tab::Types as usize].push(pair.0.clone());
        }

        for pair in source.globals.iter() {
            self.items[Tab::Globals as usize].push(pair.0.clone());
        }

        for pair in source.functions.iter() {
            self.items[Tab::Functions as usize].push(pair.0.clone());
        }
    }
}
