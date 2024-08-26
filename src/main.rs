mod ir;
mod error;
mod menus;
mod app;

use std::{env, io::{self, stdout}, path::PathBuf, time::Duration};

use app::{App, Component};
use ratatui::{crossterm::{event, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand}, prelude::CrosstermBackend, Terminal};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut app = App::new();

    if args.len() > 1 {
        if let Err(e) = app.init_with_project(PathBuf::from(&args[1])) {
            panic!("Error while initializing project: {}", e);
        }
    }

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;

    loop {
        term.draw(|frame| {
            app.render(frame, frame.area()); 
        })?;

        if event::poll(Duration::from_millis(16))? {
            app.update(event::read()?);
        }

        if app.exit {
            break
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
