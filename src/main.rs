mod ir;
mod ui;
mod net;

use std::{env, io::{stdout, Stdout}, net::{Ipv4Addr, SocketAddr}, path::PathBuf, time::Duration};

use net::Client;
use ir::{ObjectKind, Project};
use ratatui::{crossterm::{event::{self, Event, KeyCode, KeyEventKind, KeyModifiers}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand}, prelude::CrosstermBackend, Terminal};

use ui::ProjectView;

fn main() -> Result<(), ir::Error> {
    let Some(path_string) = env::args().next() else {
        println!("Expected a path as an argument!");
        return Ok(())
    };
    let path = PathBuf::from(path_string);

    let mut project = match Project::open(&path) {
        Ok(o) => o,
        Err(e) => {
            println!("Could not open project, creating new one. Reason: {}", e);
            Project::new()
        },
    };

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;

    enter_main_screen(&mut term, &mut project)?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn enter_main_screen(term: &mut Terminal<CrosstermBackend<Stdout>>, project: &mut Project) -> Result<(), ir::Error> {
    let mut conflicts = Project::new(); 
    let mut client = Client::connect(SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 30012))?;
    let mut menu = ProjectView::new();

    loop {
        client.update_project(project, &mut conflicts);

        if !conflicts.is_empty() {
            enter_merge_screen(term, project, &mut conflicts)?;
        }

        term.draw(|frame| {
            menu.render(frame, frame.area(), &project);
        })?;

        if !event::poll(Duration::from_millis(16))? {
            continue
        }

        let event = event::read()?;

        let k = match event {
            Event::Key(key_event) => key_event,
            _ => continue,
        };

        if k.kind == KeyEventKind::Release {
            continue
        }

        match (k.modifiers, k.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c')) => break,
            (KeyModifiers::NONE, KeyCode::Up) => menu.move_cursor_up(project),
            (KeyModifiers::NONE, KeyCode::Down) => menu.move_cursor_down(project),
            (KeyModifiers::NONE, KeyCode::Char('1')) => menu.tab = ObjectKind::Types,
            (KeyModifiers::NONE, KeyCode::Char('2')) => menu.tab = ObjectKind::Functions,
            (KeyModifiers::NONE, KeyCode::Char('3')) => menu.tab = ObjectKind::Globals,
            _ => {},
        }
    }

    Ok(())
}

fn enter_merge_screen(term: &mut Terminal<CrosstermBackend<Stdout>>, project: &mut Project, conflicts: &mut Project) -> Result<(), ir::Error> {
    loop {
        term.draw(|frame| {
        })?;

        if !event::poll(Duration::from_millis(16))? {
            continue
        }

        let event = event::read()?;

        let k = match event {
            Event::Key(key_event) => key_event,
            _ => continue,
        };

        if k.kind == KeyEventKind::Release {
            continue
        }

        match (k.modifiers, k.code) {
            (KeyModifiers::NONE, KeyCode::Esc) => break, 
            _ => {},
        }
    }
    Ok(())
}