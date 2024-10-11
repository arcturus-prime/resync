use std::collections::HashMap;

use ratatui::{
    crossterm::style::Color,
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
    pub project: Project,
    cursor: usize,
    pub tab: ObjectKind,
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
