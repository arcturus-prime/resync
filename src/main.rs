mod ir;
mod error;
mod component;
mod menu;

use std::{env, io::{self, stdout}, path::{Path, PathBuf}, process::exit, sync::{Arc, Mutex}, time::Duration};

use ir::Project;
use ratatui::{crossterm::{event::{self, Event, KeyCode, KeyEventKind, KeyModifiers}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand}, prelude::CrosstermBackend, Terminal};

use component::{editable_text::EditableText, Renderable};
use menu::Menu;


fn exit_screen() -> io::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn main() -> io::Result<()> {
    let path = PathBuf::from(env::args().next().unwrap());
    let Ok(project) = Project::open(&path) else {
        println!("Could not open project!");
        return Ok(())
    };
    let mut menu: Menu = Menu::new(project);

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;

    loop {
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
            _ => {},
        }

        menu.update(event);
    }

    exit_screen()?;
    Ok(())
}
