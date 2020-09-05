use std::io::{stdout, Write};

use crossterm::{
    execute,
    terminal as term,
    Result,
    event,
    queue,
    cursor,
};

pub enum Action {
    Redraw,
}

mod views;
mod keybindings;
use views::{BoundingBox, Screen, AcceptsInput, playlist::PlaylistScreen};
use keybindings::{KeyBinding, KeyBindings};

struct App {
    playlists: PlaylistScreen,
    keybindings: KeyBindings,
}

impl App {
    fn start(&mut self) -> Result<()> {
        keybindings::default_keybindings(&mut self.keybindings);

        term::enable_raw_mode()?;
        queue!(
            stdout(),
            term::EnterAlternateScreen,
            cursor::Hide,
        )?;
        Ok(())
    }

    fn stop(&self) -> Result<()> {
        term::disable_raw_mode()?;
        execute!(
            stdout(),
            term::LeaveAlternateScreen,
            cursor::Show,
        )?;
        Ok(())
    }

    fn handle_key(&mut self, key: KeyBinding) -> Result<bool> {
        match key {
            KeyBinding::Quit => {
                self.stop()?;
                return Ok(false);
            }
            _ => {
                if let Some(res) = self.playlists.receive_input(key) {
                    self.handle_action(res)?;
                }
            }
        }
        Ok(true)
    }

    fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::Redraw => self.playlists.display(
                BoundingBox { x: 0, y: 0, width: 100, height: 25 }
            )?,
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut app = App {
        playlists: views::playlist::PlaylistScreen::new(),
        keybindings: KeyBindings::new(),
    };

    app.start()?;

    app.playlists.display(
        BoundingBox { x: 0, y: 0, width: 100, height: 25 }
    )?;

    loop {
        if let Ok(event::Event::Key(e)) = event::read() {
            if let Some(&key) = app.keybindings.get(&e) {
                if !app.handle_key(key)? {
                    return Ok(());
                }
            }
        }
    }
}
