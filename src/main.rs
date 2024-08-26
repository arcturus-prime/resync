mod ir;
mod error;
mod menus;
mod app;

use std::{env, io::{self, stdout}, path::PathBuf, time::Duration};

use app::{App, Renderable};
use ratatui::{crossterm::{event, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand}, prelude::CrosstermBackend, Terminal};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Expected a file path to the project!")
    }

    let path = PathBuf::from(&args[1]);
    let mut app = App::create(path).expect("Initializing app failed!");

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
