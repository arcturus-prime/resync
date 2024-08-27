use ratatui::{
    crossterm::{
        event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
        style::Color,
    },
    layout::{Constraint, Layout, Offset, Rect},
    style::Stylize,
    widgets::{block::Title, Block, List, ListItem},
    Frame,
};

use super::super::Component;
use crate::{components::path::PathPrompt, ir::Project};

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
    pub items: [Vec<String>; 3],
    pub cursor: usize,
    pub tab: Tab,
    pub path_prompt: Option<PathPrompt>,
}

impl Component for ProjectMenu {
    type Action = Event;

    fn render(&self, frame: &mut Frame, area: Rect) {
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

        let block = Block::new();
        let layout = Layout::new(ratatui::layout::Direction::Vertical, Constraint::from_percentages([10, 100])).split(area);

        if let Some(prompt) = &self.path_prompt {
            frame.render_widget(block.title("Open an existing project"), layout[0]);
            prompt.render(frame, layout[1]);
        } else {
            frame.render_widget(block.title("Project"), layout[0]);
            frame.render_widget(List::new(items), layout[1]);
        }
    }

    fn update(&mut self, action: Self::Action) {
        match action {
            Event::FocusGained => {}
            Event::FocusLost => todo!(),
            Event::Key(k) => {
                if k.kind == KeyEventKind::Release {
                    return;
                }

                if let Some(prompt) = &mut self.path_prompt {
                    if k.code == KeyCode::Esc {
                        self.path_prompt = None;
                    } else {
                        prompt.update(k.code)
                    }
                } else {
                    self.handle_key(k)
                }
            }
            Event::Mouse(_) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => {}
        }
    }
}

impl ProjectMenu {
    pub fn new() -> Self {
        Self {
            items: [const { Vec::new() }; 3],
            cursor: 0,
            tab: Tab::Functions,
            path_prompt: None,
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Up) => self.update_cursor(Direction::Up),
            (KeyModifiers::NONE, KeyCode::Down) => self.update_cursor(Direction::Down),
            (KeyModifiers::NONE, KeyCode::Char('1')) => self.update_tab(Tab::Types),
            (KeyModifiers::NONE, KeyCode::Char('2')) => self.update_tab(Tab::Functions),
            (KeyModifiers::NONE, KeyCode::Char('3')) => self.update_tab(Tab::Globals),
            (KeyModifiers::NONE, KeyCode::Char('o')) => {
                self.path_prompt = Some(PathPrompt::new())
            }
            _ => {}
        }
    }

    fn update_cursor(&mut self, direction: Direction) {
        let length = self.items[self.tab as usize].len();

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
