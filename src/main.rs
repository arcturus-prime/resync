mod ir;
mod error;
mod component;
mod app;
mod context;

use std::{io::{self, stdout}, time::Duration};

use app::App;
use ratatui::{crossterm::{event, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand}, prelude::CrosstermBackend, Terminal};

fn main() -> io::Result<()> {
    let mut app = App::new();

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
