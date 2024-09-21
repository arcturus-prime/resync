use std::{fmt::Display, marker::PhantomData};

use ratatui::{crossterm::{event::{KeyCode, KeyEvent, KeyModifiers}, style::Color}, layout::Rect, style::Stylize, text::Text, widgets::{List, ListItem}, Frame};

use super::Renderable;

pub enum Direction {
    Up,
    Down,
}

pub struct SelectableList<'a, T: Display + Clone + Into<Text<'a>>> {
    items: Vec<T>,
    cursor: usize,
    phantom: PhantomData<&'a ()>
}

impl<'a, T: Display + Clone + Into<Text<'a>>> Renderable for SelectableList<'a, T> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let rows = area.rows().count();

        let start = self.cursor - self.cursor % rows;
        let end = start + rows.min(self.items.len());

        let items: Vec<ListItem> = self.items[start..end]
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

        frame.render_widget(List::new(items).bg(Color::AnsiValue(235)), area)
    }
}

impl<'a, T: Display + Clone + Into<Text<'a>>> SelectableList<'a, T> {
    pub fn move_cursor(&mut self, direction: Direction) {
        let length = self.items.len();

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

    pub fn new() -> Self {
        Self {
            items: vec![],
            cursor: 0,
            phantom: PhantomData,
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item)
    }
    
    pub fn get_current(&self) -> &T {
        &self.items[self.cursor]
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}