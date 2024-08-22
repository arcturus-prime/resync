use ratatui::layout::Rect;

use crate::component::Component;

enum Action {
    Select,
    Unselect,
    Toggle,
}

struct Switch {
    enabled: bool,
    selected: bool,
}

impl Component for Switch {
    fn render(&self, frame: &mut ratatui::Frame, area: Rect) {
    }

    fn update(&mut self, action: Self::Action) {
        match action {
            Action::Select => self.selected = true,
            Action::Unselect => self.selected = false,
            Action::Toggle => self.enabled = !self.enabled,
        }
    }
    
    type Action = Action;
}