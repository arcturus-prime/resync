mod ir;
mod net;
mod ui;

use std::{
    env,
    io::{stdout, Stdout},
    net::{Ipv4Addr, SocketAddr},
    path::PathBuf,
    time::Duration,
};

use ir::{ObjectKind, Project};
use net::{Client, Message};
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Layout},
    prelude::CrosstermBackend,
    text::Text,
    Terminal,
};

use ui::ProjectView; 

struct App {
    path: PathBuf,
    project: Project,
    conflict: Project,
    client: Client,
}

impl App {
    pub fn enter_start_menu(
        &mut self,
        term: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), ir::Error> {
        let mut menu = ProjectView::new();

        loop {
            loop {
                if !self.update_project() {
                    break;
                }
            }

            if !self.conflict.is_empty() {
                self.enter_merge_screen(term)?;
            }

            term.draw(|frame| {
                menu.render(frame, frame.area(), &self.project);
            })?;

            if !event::poll(Duration::from_millis(16))? {
                continue;
            }

            let event = event::read()?;

            let k = match event {
                Event::Key(key_event) => key_event,
                _ => continue,
            };

            if k.kind == KeyEventKind::Release {
                continue;
            }

            match (k.modifiers, k.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('c')) => break,
                (KeyModifiers::CONTROL, KeyCode::Char('s')) => self.project.save(&self.path)?,
                (KeyModifiers::NONE, KeyCode::Up) => menu.move_cursor_up(&self.project),
                (KeyModifiers::NONE, KeyCode::Down) => menu.move_cursor_down(&self.project),
                (KeyModifiers::NONE, KeyCode::Char('1')) => menu.tab = ObjectKind::Types,
                (KeyModifiers::NONE, KeyCode::Char('2')) => menu.tab = ObjectKind::Functions,
                (KeyModifiers::NONE, KeyCode::Char('3')) => menu.tab = ObjectKind::Globals,
                _ => {}
            }
        }

        Ok(())
    }

    fn enter_merge_screen(
        &mut self,
        term: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), ir::Error> {
        let display_a;
        let display_b;

        if !self.conflict.functions.is_empty() {
            let pair = self.conflict.functions.iter().next().unwrap();

            display_a = Text::raw(serde_json::to_string_pretty(pair.1)?);
            display_b = Text::raw(serde_json::to_string_pretty(
                &self.project.functions[pair.0],
            )?);
        } else if !self.conflict.types.is_empty() {
            let pair = self.conflict.types.iter().next().unwrap();

            display_a = Text::raw(serde_json::to_string_pretty(pair.1)?);
            display_b = Text::raw(serde_json::to_string_pretty(&self.project.types[pair.0])?);
        } else if !self.conflict.globals.is_empty() {
            let pair = self.conflict.globals.iter().next().unwrap();

            display_a = Text::raw(serde_json::to_string_pretty(pair.1)?);
            display_b = Text::raw(serde_json::to_string_pretty(&self.project.globals[pair.0])?);
        } else {
            return Ok(());
        }

        loop {
            term.draw(|frame| {
                let layout = Layout::new(
                    ratatui::layout::Direction::Horizontal,
                    Constraint::from_percentages([50, 50]),
                )
                .split(frame.area());

                frame.render_widget(&display_a, layout[0]);
                frame.render_widget(&display_b, layout[1]);
            })?;

            if !event::poll(Duration::from_millis(16))? {
                continue;
            }

            let event = event::read()?;

            let k = match event {
                Event::Key(key_event) => key_event,
                _ => continue,
            };

            if k.kind == KeyEventKind::Release {
                continue;
            }

            match (k.modifiers, k.code) {
                (KeyModifiers::NONE, KeyCode::Esc) => break,
                _ => {}
            }
        }

        Ok(())
    }

    fn update_project(&mut self) -> bool {
        let Ok(message) = self.client.rx.try_recv() else {
            return false;
        };

        match message {
            Message::SyncFunction(name, function) => {
                if self.project.functions.contains_key(&name) {
                    if self.project.functions[&name] == function {
                        return true;
                    }

                    self.conflict.functions.insert(name, function);
                } else {
                    self.project.functions.insert(name, function);
                }
            }
            Message::SyncGlobal(name, global) => {
                if self.project.globals.contains_key(&name) {
                    if self.project.globals[&name] == global {
                        return true;
                    }

                    self.conflict.globals.insert(name, global);
                } else {
                    self.project.globals.insert(name, global);
                }
            }
            Message::SyncType(name, type_) => {
                if self.project.types.contains_key(&name) {
                    if self.project.types[&name] == type_ {
                        return true;
                    }

                    self.conflict.types.insert(name, type_);
                } else {
                    self.project.types.insert(name, type_);
                }
            }
            Message::PushFunction(name, function) => {
                self.project.functions.insert(name, function);
            }
            Message::PushGlobal(name, global) => {
                self.project.globals.insert(name, global);
            }
            Message::PushType(name, type_) => {
                self.project.types.insert(name, type_);
            }
        }

        true
    }
}

fn main() -> Result<(), ir::Error> {
    let Some(path_string) = env::args().skip(1).next() else {
        println!("Expected a path as an argument!");
        return Ok(());
    };
    let path = PathBuf::from(path_string);

    let project = match Project::open(&path) {
        Ok(o) => o,
        Err(e) => {
            println!("Could not open project, creating new one. Reason: {}", e);
            Project::new()
        }
    };

    let client = Client::connect(SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 30012))?;
    let conflict = Project::new();

    let mut app = App {
        path,
        project,
        conflict,
        client,
    };

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;

    app.enter_start_menu(&mut term)?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
