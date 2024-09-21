mod ir;
mod error;
mod component;

use std::{io::{self, stdout}, path::{Path, PathBuf}, sync::{Arc, Mutex}, time::Duration};

use ir::Project;
use ratatui::{crossterm::{event::{self, Event, KeyCode, KeyEventKind, KeyModifiers}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand}, prelude::CrosstermBackend, Terminal};
use component::{editable_text::EditableText, project_display::ProjectDisplay, Renderable};

fn main() -> io::Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;

    loop {
        if event::poll(Duration::from_millis(16))? {
            let event = event::read()?;

            if let Event::Key(k) = event {
                if k.kind == KeyEventKind::Release {
                    continue
                }

                match (k.modifiers, k.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('p')) => {
                    },
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => break,
                    _ => {},
                };
            }
        }

        term.draw(|frame| {
            
            
        })?;
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
