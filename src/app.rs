use std::path::PathBuf;

use ratatui::{crossterm::event::Event, layout::Rect, Frame};

use crate::{error::Error, ir::Project, menus::ProjectMenu};

pub trait Component {
    type Action;

    fn update(&mut self, action: Self::Action);
    fn render(&self, frame: &mut Frame, area: Rect);
}

pub struct App {
    project: Project,
    project_path: Option<PathBuf>,
    
    current: usize,
    menus: Vec<Box<dyn Component<Action = Event>>>,
    pub exit: bool,
}

impl Component for App {
    type Action = Event;
    
    fn render(&self, frame: &mut Frame, area: Rect) {
        self.menus[self.current].render(frame, area)
    }

    fn update(&mut self, action: Self::Action) {
        self.menus[self.current].update(action)
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            project: Project::new(),
            project_path: None,
            current: 0,
            menus: vec![],
            exit: false,
        }
    }

    pub fn init_with_project(&mut self, path: PathBuf) -> Result<(), Error> {
        self.project = Project::open(&path)?;
        self.project_path = Some(path);

        self.menus.push(Box::new(ProjectMenu::new()));
        Ok(())
    }
}