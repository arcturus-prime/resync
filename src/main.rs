mod ir;
mod error;
mod renderable;

use std::{io::{self, stdout}, path::{Path, PathBuf}, sync::{Arc, Mutex}, time::Duration};

use ir::Project;
use ratatui::{crossterm::{event::{self, Event, KeyCode, KeyEventKind, KeyModifiers}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand}, prelude::CrosstermBackend, Terminal};
use renderable::{editable_text::EditableText, project::ProjectMenu, Renderable};

fn main() -> io::Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;

    let mut file_open = EditableText::new();
    let mut focus_open = true;
    let mut menus: Vec<ProjectMenu> = Vec::new();
    let mut current = 0;

    loop {
        term.draw(|frame| {
            if focus_open {
                file_open.render(frame, frame.area());
                return
            }

            if menus.len() == 0 {
                return
            }
            
            menus[current].render(frame, frame.area())
        })?;

        if event::poll(Duration::from_millis(16))? {
            let event = event::read()?;

            if let Event::Key(k) = event {
                if k.kind == KeyEventKind::Release {
                    continue
                }

                match (k.modifiers, k.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('p')) => {
                        focus_open = true;
                    },
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => break,
                    _ => {},
                }

                if focus_open {
                    if k.code == KeyCode::Enter {
                        let Ok(project) = Project::open(&PathBuf::from(file_open.get())) else {
                            continue
                        };
                        file_open.clear();

                        menus.push(ProjectMenu::new(project));
                        current = menus.len() - 1;

                        focus_open = false;
                        continue
                    }

                    file_open.update(k)
                } else {
                    menus[current].update(event)
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
