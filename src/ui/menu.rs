use std::collections::HashMap;

use ratatui::{
    crossterm::style::Color,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{List, ListItem},
    Frame,
};

use crate::ir::{Function, Global, ObjectKind, Project, Type, TypeInfo};
use crate::ui::Renderable;

use super::object_display::{Object, ObjectDisplay};

pub struct Menu {
    pub project: Project,
    pub conflicts: Project,

    cursor: usize,
    pub tab: ObjectKind,

    object_disp_a: ObjectDisplay,
    object_disp_b: ObjectDisplay,
}

impl Renderable for Menu {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let items = match self.tab {
            ObjectKind::Types => self.get_list(&self.project.types, area),
            ObjectKind::Functions => self.get_list(&self.project.functions, area),
            ObjectKind::Globals => self.get_list(&self.project.globals, area),
        };

        if !self.conflicts.is_empty() {
            let layout = Layout::new(ratatui::layout::Direction::Horizontal, Constraint::from_percentages([50, 50])).split(area);

            self.object_disp_a.render(frame, layout[0]);
            self.object_disp_b.render(frame, layout[1])
        } else {
            frame.render_widget(List::new(items).bg(Color::AnsiValue(235)), area)
        }
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

    pub fn move_cursor_up(&mut self) {
        let length = self.get_current_length();

        if length == 0 {
            return;
        }

        self.cursor += 1 % length;
    }

    pub fn move_cursor_down(&mut self) {
        let length = self.get_current_length();

        if length == 0 {
            return;
        }

        self.cursor = if self.cursor == 0 {
            length - 1
        } else {
            self.cursor - 1
        }
    }

    pub fn new(project: Project) -> Self {
        Self {
            project,
            conflicts: Project::new(),
            tab: ObjectKind::Functions,
            cursor: 0,
            object_disp_a: ObjectDisplay::new(),
            object_disp_b: ObjectDisplay::new()
        }
    }
}
