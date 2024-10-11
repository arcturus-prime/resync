use std::{cell::RefCell, collections::HashMap};

use ratatui::{
    crossterm::{
        event::{Event, KeyCode, KeyEvent, KeyModifiers},
        style::Color,
    },
    layout::Rect,
    style::Stylize,
    widgets::{List, ListItem},
    Frame,
};

use crate::ir::{ObjectKind, Project};
use crate::component::Renderable;

pub enum Direction {
    Up,
    Down,
}

pub struct Menu {
    project: Project,
    cursor: usize,
    tab: ObjectKind,
}

impl Renderable for Menu {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let items = match self.tab {
            ObjectKind::Types => self.get_list(&self.project.types, area),
            ObjectKind::Functions => self.get_list(&self.project.functions, area),
            ObjectKind::Globals => self.get_list(&self.project.globals, area),
        };

        frame.render_widget(List::new(items).bg(Color::AnsiValue(235)), area)
    }
}

impl Menu {
    fn get_list<T>(&self, map: &HashMap<String, T>, area: Rect) -> Vec<ListItem> {
        let rows = area.rows().count();

        let start = self.cursor - self.cursor % rows;
        let end = start + rows.min(map.len());

        let items: Vec<ListItem> = map
            .keys()
            .skip(start)
            .zip(start..end)
            .map(|pair| {
                if pair.1 == self.cursor {
                    ListItem::new(pair.0.clone()).bg(Color::Blue)
                } else {
                    ListItem::new(pair.0.clone()).bg(Color::Black)
                }
            })
            .collect();

        items
    }

    fn get_current_length(&self) -> usize {
        match self.tab {
            ObjectKind::Types => self.project.types.len(),
            ObjectKind::Functions => self.project.functions.len(),
            ObjectKind::Globals => self.project.globals.len(),
        }
    }

    pub fn process_key(&mut self, action: KeyEvent) {
        match (action.modifiers, action.code) {
            (KeyModifiers::NONE, KeyCode::Up) => self.update_cursor(Direction::Up),
            (KeyModifiers::NONE, KeyCode::Down) => self.update_cursor(Direction::Down),
            (KeyModifiers::NONE, KeyCode::Char('1')) => self.tab = ObjectKind::Types,
            (KeyModifiers::NONE, KeyCode::Char('2')) => self.tab = ObjectKind::Functions,
            (KeyModifiers::NONE, KeyCode::Char('3')) => self.tab = ObjectKind::Globals,
            _ => {}
        }
    }

    pub fn update(&mut self, event: Event) {
        match event {
            Event::Key(key_event) => self.process_key(key_event),
            _ => {}
        }
    }

    pub fn update_cursor(&mut self, direction: Direction) {
        let length = self.get_current_length();

        if length == 0 {
            return;
        }

        self.cursor = match direction {
            Direction::Down => self.cursor + 1 % length,
            Direction::Up => {
                if self.cursor == 0 {
                    length - 1
                } else {
                    self.cursor - 1
                }
            }
        }
    }

    pub fn new(project: Project) -> Self {
        Self {
            project,
            tab: ObjectKind::Functions,
            cursor: 0,
        }
    }
}
