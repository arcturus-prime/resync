use ratatui::{
    crossterm::style::Color,
    layout::Rect,
    style::Stylize,
    widgets::{List, ListItem},
};

use crate::{ir::{Project, ProjectRef}, app::Renderable};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Tab {
    Types,
    Functions,
    Globals,
}

pub struct MergeConflictMenu {
    pub conflicts: ProjectRef,
    pub cursor: usize,
    pub tab: Tab,
}

impl Renderable for MergeConflictMenu {
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

impl MergeConflictMenu {
    pub fn create_from_project_diff(source: &Project, dest: &mut Project) -> Self {
        let mut project_ref = ProjectRef::new();

        for pair in source.types.iter() {
            if dest.types.contains_key(pair.0) {
                project_ref.types.push(pair.0.clone());
            }
        }
             
        for pair in source.globals.iter() {
            if dest.globals.contains_key(pair.0) {
                project_ref.globals.push(pair.0.clone());
            }
        }

        for pair in source.functions.iter() {
            if dest.functions.contains_key(pair.0) {
                project_ref.functions.push(pair.0.clone());
            }
        }

        Self {
            cursor: 0,
            tab: Tab::Types,
            conflicts: project_ref,
        }
    }

    pub fn update_cursor(&mut self, index: usize) -> Result<(), ()> {
        let is_in_bounds = match self.tab {
            Tab::Types => index < self.conflicts.types.len(),
            Tab::Functions => index < self.conflicts.functions.len(),
            Tab::Globals => index < self.conflicts.globals.len(),
        };

        if !is_in_bounds {
            return Err(())
        }

        self.cursor = index;
        
        Ok(())
    }
}
