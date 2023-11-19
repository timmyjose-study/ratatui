use core::time;
use std::io;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, style::Stylize, widgets::Paragraph, Terminal};

fn main() -> io::Result<()> {
    // enter alternate screen
    io::stdout().execute(EnterAlternateScreen)?;

    // enable raw mode
    enable_raw_mode()?;

    // create a new terminal with backend
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    loop {
        // draw
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello, ratatui! (Press 'q' to quit).")
                    .black()
                    .on_yellow(),
                area,
            )
        })?;

        // handle events
        if event::poll(time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && (key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q'))
                {
                    break;
                }
            }
        }
    }

    // disable alternate screen
    io::stdout().execute(LeaveAlternateScreen)?;

    // disable raw mode
    disable_raw_mode()?;

    Ok(())
}
