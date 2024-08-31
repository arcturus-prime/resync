use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Layout, Rect},
    Frame,
};

use super::{list::SelectableList, editable_text::EditableText, Component};
use crate::ir::{ObjectKind, Project};

pub enum Focus {
    List,
    Inspect,
    Open,
}

pub struct ProjectView<'a> {
    items: [SelectableList<'a, String>; 3],
    tab: ObjectKind,
    focus: Focus,
}

impl<'a> Component for ProjectView<'a> {
    type Action = Event;

    fn render(&self, frame: &mut Frame, area: Rect) {
        let layout = Layout::new(
            ratatui::layout::Direction::Horizontal,
            Constraint::from_percentages([50, 50]),
        )
        .split(area);

        self.items[self.tab as usize].render(frame, layout[0]);
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
            tab: ObjectKind::Functions,
            focus: Focus::List,
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match self.focus {
            Focus::List => match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Char('1')) => self.update_tab(ObjectKind::Types),
                (KeyModifiers::NONE, KeyCode::Char('2')) => self.update_tab(ObjectKind::Functions),
                (KeyModifiers::NONE, KeyCode::Char('3')) => self.update_tab(ObjectKind::Globals),
                _ => self.items[self.tab as usize].update(key)
            },
            Focus::Inspect => {},
            Focus::Open => {

            }
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
