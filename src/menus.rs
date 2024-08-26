use ratatui::{
    crossterm::{event::{Event, KeyCode, KeyEventKind}, style::Color},
    layout::Rect,
    style::Stylize,
    widgets::{List, ListItem},
    Frame,
};

use crate::{app::Component, ir::Project};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Tab {
    Types = 0,
    Functions,
    Globals,
}

pub enum Direction {
    Up,
    Down,
}

pub struct ProjectMenu {
    items: [Vec<String>; 3],
    cursor: usize,
    tab: Tab,
}

impl Component for ProjectMenu {
    type Action = Event;

    fn render(&self, frame: &mut ratatui::Frame, area: Rect) {
        let vec = &self.items[self.tab as usize];

        let start = self.cursor - self.cursor % 20;
        let end = start + 20.min(vec.len());

        let items: Vec<ListItem> = vec[start..end]
            .iter()
            .zip(start..)
            .map(|pair| {
                if pair.1 == self.cursor {
                    ListItem::new(pair.0.clone()).bg(Color::Blue)
                } else {
                    ListItem::new(pair.0.clone()).bg(Color::Black)
                }
            })
            .collect();

        frame.render_widget(List::new(items), area);
    }
        
    fn update(&mut self, action: Self::Action) {
        match action {
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Key(k) => {
                if k.kind == KeyEventKind::Release {
                    return
                }

                match k.code {
                    KeyCode::Up => self.update_cursor(Direction::Up),
                    KeyCode::Down => self.update_cursor(Direction::Down),
                    KeyCode::Char('1') => self.update_tab(Tab::Types),
                    KeyCode::Char('2') => self.update_tab(Tab::Functions),
                    KeyCode::Char('3') => self.update_tab(Tab::Globals),
                    _ => {}
                }
            },
            Event::Mouse(_) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => todo!(),
        }
    }
}

impl ProjectMenu {
    pub fn new() -> Self {
        Self {
            items: [const { Vec::new() }; 3],
            cursor: 0,
            tab: Tab::Functions,
        }
    }

    fn update_cursor(&mut self, direction: Direction) {
        let length = self.items[self.tab as usize].len();

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

    fn update_tab(&mut self, tab: Tab) {
        let length = self.items[self.tab as usize].len();
    }

    pub fn get_current(&self) -> &String {
        &self.items[self.tab as usize][self.cursor]
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

    pub fn clear(&mut self) {
        for vec in &mut self.items {
            vec.clear();
        }
    }
}
