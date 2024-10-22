use std::collections::HashMap;

use ratatui::{crossterm::style::Color, layout::{Constraint, Layout, Rect}, style::{Style, Stylize}, text::Text, widgets::{List, ListItem}, Frame};

use crate::ir::{self, ObjectKind, Project};

pub struct EditableText {
    buffer: String,
    index: usize,
}

impl EditableText {
    pub fn render(&self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let block = Text::styled(&self.buffer, Style::default().bg(Color::AnsiValue(235).into()));

        frame.render_widget(block, area);
    }

    pub fn move_index_left(&mut self) {
        if self.index == 0 {
            return
        }
        self.index -= 1
    }

    pub fn move_index_right(&mut self) {
        if self.index >= self.buffer.len() - 1 {
            return
        }
        self.index += 1
    }

    pub fn backspace(&mut self) {
        if self.index == 0 {
            return
        }

        if self.index == self.buffer.len() {
            self.buffer.pop();
            self.index -= 1;
            return
        }
        
        self.buffer.remove(self.index);
    }

    pub fn replace_buffer(&mut self, buffer: String) {
        self.buffer = buffer;
        self.index = 0
    }

    pub fn get(&self) -> &str {
        &self.buffer
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.index = 0
    }

    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            index: 0,
        }
    }
}

pub struct ProjectView {
    cursor: usize,
    pub tab: ObjectKind,
    pub display: EditableText
}

impl ProjectView {
    pub fn render(&self, frame: &mut Frame, area: Rect, project: &Project) {
        let items = match self.tab {
            ObjectKind::Types => self.get_list(&project.types, area),
            ObjectKind::Functions => self.get_list(&project.functions, area),
            ObjectKind::Globals => self.get_list(&project.globals, area),
        };

        let layout = Layout::new(ratatui::layout::Direction::Horizontal, Constraint::from_percentages([50, 50])).split(area);

        frame.render_widget(List::new(items).bg(Color::AnsiValue(235)), layout[0]);
        self.display.render(frame, layout[1]);
    }

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

    fn get_current_length(&self, project: &Project) -> usize {
        match self.tab {
            ObjectKind::Types => project.types.len(),
            ObjectKind::Functions => project.functions.len(),
            ObjectKind::Globals => project.globals.len(),
        }
    }

    pub fn select_current(&mut self, project: &Project) -> Result<(), ir::Error> {
        let result = match self.tab {
            ObjectKind::Types => {
                let obj = project.types.values().skip(self.cursor).next().unwrap().clone();
                serde_json::to_string_pretty(&obj)
            },
            ObjectKind::Functions => {
                let obj = project.functions.values().skip(self.cursor).next().unwrap().clone();
                serde_json::to_string_pretty(&obj)
            },
            ObjectKind::Globals => {
                let obj = project.globals.values().skip(self.cursor).next().unwrap().clone();
                serde_json::to_string_pretty(&obj)
            },
        }?;

        self.display.replace_buffer(result);

        Ok(())
    }

    pub fn move_cursor_up(&mut self, project: &Project) {
        let length = self.get_current_length(project);

        if length == 0 {
            return;
        }

        self.cursor += 1 % length;
    }

    pub fn move_cursor_down(&mut self, project: &Project) {
        let length = self.get_current_length(project);

        if length == 0 {
            return;
        }

        self.cursor = if self.cursor == 0 {
            length - 1
        } else {
            self.cursor - 1
        }
    }

    pub fn new() -> Self {
        Self {
            tab: ObjectKind::Functions,
            cursor: 0,
            display: EditableText::new()
        }
    }
}
