use ratatui::layout::{Constraint, Layout};

use crate::ir::Project;

use super::{project_display::{ProjectDisplay, Tab}, selectable_list::Direction, Renderable};

pub struct MergeView<'a> {
    projects: [ProjectDisplay<'a>; 2], 
    focus: usize,
}

impl<'a> Renderable for MergeView<'a> {
    fn render(&self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let layout = Layout::new(
            ratatui::layout::Direction::Horizontal,
            Constraint::from_percentages([50, 50]),
        )
        .split(area);

        self.projects[0].render(frame, layout[0]);
        self.projects[1].render(frame, layout[1]);
    }
}

impl<'a> MergeView<'a> {
    pub fn change_focus(&mut self) {
        self.focus = !self.focus
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        self.projects[self.focus].get_current().move_cursor(direction)
    }

    pub fn change_tab(&mut self, tab: Tab) {
        self.projects[self.focus].tab = tab;
    }

    pub fn new(project_a: &Project, project_b: &Project) -> Self {
        Self {
            projects: [ProjectDisplay::new(project_a), ProjectDisplay::new(project_b)],
            focus: 0
        }
    }

}