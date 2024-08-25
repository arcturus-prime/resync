use ratatui::{
    crossterm::style::Color,
    layout::Rect,
    style::Stylize,
    widgets::{List, ListItem},
};

use crate::{
    app::Renderable,
    ir::{Project, ProjectRef},
};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Tab {
    Types,
    Functions,
    Globals,
}

pub enum Direction {
    Up,
    Down,
}

pub struct MergeMenu {
    conflicts: ProjectRef,
    cursor: usize,
    tab: Tab,
}

impl Renderable for MergeMenu {
    fn render(&self, frame: &mut ratatui::Frame, area: Rect) {
        let get_list = |vec: &Vec<String>| -> List {
            let items: Vec<ListItem> = vec
                .iter()
                .zip(0..)
                .map(|pair| {
                    if pair.1 == self.cursor {
                        ListItem::new(pair.0.clone()).bg(Color::Blue)
                    } else {
                        ListItem::new(pair.0.clone()).bg(Color::Black)
                    }
                })
                .collect();

            List::new(items)
        };

        let list = match self.tab {
            Tab::Types => get_list(&self.conflicts.types),
            Tab::Functions => get_list(&self.conflicts.functions),
            Tab::Globals => get_list(&self.conflicts.globals),
        };

        frame.render_widget(list, area);
    }
}

impl MergeMenu {
    pub fn new() -> Self {
        Self {
            conflicts: ProjectRef::new(),
            cursor: 0,
            tab: Tab::Types,
        }
    }

    pub fn apply_project_diff(&mut self, source: &Project, dest: &Project) {
        for pair in source.types.iter() {
            if dest.types.contains_key(pair.0) {
                self.conflicts.types.push(pair.0.clone());
            }
        }

        for pair in source.globals.iter() {
            if dest.globals.contains_key(pair.0) {
                self.conflicts.globals.push(pair.0.clone());
            }
        }

        for pair in source.functions.iter() {
            if dest.functions.contains_key(pair.0) {
                self.conflicts.functions.push(pair.0.clone());
            }
        }
    }

    pub fn update_cursor(&mut self, direction: Direction) {
        let length = match self.tab {
            Tab::Types => self.conflicts.types.len(),
            Tab::Functions => self.conflicts.functions.len(),
            Tab::Globals => self.conflicts.globals.len(),
        };

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

    pub fn update_tab(&mut self, tab: Tab) {

    }

    pub fn get_current(&self) -> &String {
        match self.tab {
            Tab::Types => &self.conflicts.types[self.cursor],
            Tab::Functions => &self.conflicts.functions[self.cursor],
            Tab::Globals => &self.conflicts.globals[self.cursor],
        }
    }
}
