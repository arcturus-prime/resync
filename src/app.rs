use ratatui::{crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers}, layout::Rect, Frame};

use crate::components::{menus::project::ProjectMenu, Component};

pub struct App {
    current: usize,
    menus: Vec<Box<dyn Component<Action = Event>>>,
    
    pub exit: bool,
}

impl App {    
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if self.menus.len() == 0 {
            return
        }
        
        self.menus[self.current].render(frame, area)
    }

    pub fn update(&mut self, action: Event) {
        if let Event::Key(k) = action {
            if k.kind == KeyEventKind::Release {
                return
            }
            
            match (k.modifiers, k.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('p')) => self.new_menu(Box::new(ProjectMenu::new())),
                _ => self.menus[self.current].update(action)
            }
        }
    }

    fn new_menu(&mut self, menu: Box<dyn Component<Action = Event>>) {
        self.menus.push(menu);
        self.current = self.menus.len() - 1;
    }

    pub fn new() -> Self {
        Self {
            current: 0,
            menus: vec![],
            exit: false,
        }
    }
}
