use core::time;
use std::io;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, style::Stylize, widgets::Paragraph, Terminal};

fn main() -> io::Result<()> {
    // enter an alternate screen to avoid interference from other terminal apps.
    io::stdout().execute(EnterAlternateScreen)?;

    // enable raw mode - turns of input/output processing by the terminal.
    enable_raw_mode()?;

    // create a backend and terminal
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    // main loop

    loop {
        // draw
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello, Ratatui! (Press 'q' to quit.)")
                    .white()
                    .on_blue(),
                area,
            );
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
    // before exiting, make sure to exit the alternate scree and disable raw mode
    io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
