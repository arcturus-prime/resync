mod ir;
mod ui;
mod menu;
mod client;

use std::{env, io::{self, stdout}, net::{Ipv4Addr, SocketAddr}, path::PathBuf, time::Duration};

use client::Client;
use ir::{ObjectKind, Project};
use ratatui::{crossterm::{event::{self, Event, KeyCode, KeyEventKind, KeyModifiers}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand}, prelude::CrosstermBackend, Terminal};

use ui::Renderable;
use menu::Menu;


fn exit_screen() -> io::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn main() -> Result<(), ir::Error> {
    let Some(path_string) = env::args().next() else {
        println!("Expected a path as an argument!");
        return Ok(())
    };
    let path = PathBuf::from(path_string);
    
    let project = match Project::open(&path) {
        Ok(o) => o,
        Err(e) => {
            println!("Could not open project, creating new one. Reason: {}", e);
            Project::new()
        },
    };

    let mut menu = Menu::new(project);
    let mut client = Client::connect(SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 30012))?;

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;

    loop {
        client.update_project(&mut menu.project);

        term.draw(|frame| {
            menu.render(frame, frame.area());
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
            (KeyModifiers::NONE, KeyCode::Up) => menu.move_cursor_up(),
            (KeyModifiers::NONE, KeyCode::Down) => menu.move_cursor_down(),
            (KeyModifiers::NONE, KeyCode::Char('1')) => menu.tab = ObjectKind::Types,
            (KeyModifiers::NONE, KeyCode::Char('2')) => menu.tab = ObjectKind::Functions,
            (KeyModifiers::NONE, KeyCode::Char('3')) => menu.tab = ObjectKind::Globals,
            _ => {},
        }
    }

    exit_screen()?;
    Ok(())
}
