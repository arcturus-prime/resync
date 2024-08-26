use std::path::PathBuf;

use ratatui::{crossterm::event::Event, layout::Rect, Frame};

use crate::{error::Error, ir::Project, menus::ProjectMenu};

pub trait Renderable {
    fn render(&self, frame: &mut Frame, area: Rect);
}

pub enum Menu {
    View,
    Merge,
}

pub struct App {
    project: Project,
    project_path: PathBuf,
    current_menu: Menu,

    view_menu: ProjectMenu,
    merge_menu: ProjectMenu,

    pub exit: bool,
}

impl Renderable for App {
    fn render(&self, frame: &mut Frame, area: Rect) {
        match self.current_menu {
            Menu::View => self.view_menu.render(frame, area),
            Menu::Merge => self.merge_menu.render(frame, area),
        }
    }
}

impl App {
    pub fn create(path: PathBuf) -> Result<Self, Error> {
        let project = if path.exists() {
            Project::open(&path)
        } else {
            Ok(Project::new())
        }?;

        let mut view_menu = ProjectMenu::new();
        view_menu.apply_project(&project);

        Ok(Self {
            project,
            project_path: path,
            current_menu: Menu::View,

            view_menu,
            merge_menu: ProjectMenu::new(),
            
            exit: false,
        })
    }

    pub fn update(&mut self, event: Event) {
        match event {
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Key(k) => {},
            Event::Mouse(_) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => {},
        }
    }
}