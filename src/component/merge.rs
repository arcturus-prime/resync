use std::{collections::HashMap, fmt::Display, hash::Hash};

use ratatui::{
    crossterm::style::Color,
    layout::Rect,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, List, ListItem},
};

use crate::component::Component;

pub enum Action {
    MoveUp,
    MoveDown,
    Select,
}

pub struct MergeConflictMenu<'a, A: Display + Hash + Eq + Clone + Into<ListItem<'a>>, B: Display> {
    source: HashMap<A, B>,
    destination: &'a mut HashMap<A, B>,

    conflicts: Vec<A>,
    cursor: usize,
}

impl<'a, A: Display + Hash + Eq + Clone + Into<ListItem<'a>>, B: Display> Component
    for MergeConflictMenu<'a, A, B>
{
    type Action = Action;

    fn render(&self, frame: &mut ratatui::Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .conflicts
            .iter()
            .zip(0..)
            .map(|pair| {
                if pair.1 == self.cursor {
                    pair.0.clone().into().bg(Color::Blue)
                } else {
                    pair.0.clone().into().bg(Color::Black)
                }
            })
            .collect();

        let list = List::new(items);

        frame.render_widget(list, area)
    }

    fn update(&mut self, action: Self::Action) {
        match action {
            Action::MoveUp => {
                if self.cursor != 0 {
                    self.cursor -= 1
                }
            }
            Action::MoveDown => {
                if self.cursor < self.conflicts.len() {
                    self.cursor += 1
                }
            }
            Action::Select => {
                let key = &self.conflicts[self.cursor];

                self.destination
                    .insert(key.clone(), self.source.remove(key).unwrap());
            }
        }
    }
}

impl<'a, A: Display + Hash + Eq + Clone + Into<ListItem<'a>>, B: Display>
    MergeConflictMenu<'a, A, B>
{
    pub fn new(source: HashMap<A, B>, destination: &'a mut HashMap<A, B>) -> Self {
        let mut conflicting = Vec::new();

        for pair in source.iter() {
            if destination.contains_key(pair.0) {
                conflicting.push(pair.0.clone());
            }
        }

        Self {
            source,
            destination,
            cursor: 0,
            conflicts: conflicting,
        }
    }
}
