use std::io::{stdout, Write};

use crossterm::{
    execute,
    terminal as term,
    Result,
    event::{self, Event},
    queue,
};

mod views;

use views::BoundingBox;

fn main() -> Result<()> {
    let mut stdout = stdout();
    queue!(stdout, term::EnterAlternateScreen)?;
    term::enable_raw_mode()?;

    views::playlist::display(
        &mut stdout,
        BoundingBox { x: 0, y: 0, width: 100, height: 25 }
    )?;

    loop {
        if let Ok(Event::Key(_)) = event::read() {
            term::disable_raw_mode()?;
            execute!(stdout, term::LeaveAlternateScreen)?;
            return Ok(());
        }
    }
}
