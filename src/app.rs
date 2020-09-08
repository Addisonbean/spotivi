use std::io::{stdout, Write};

use anyhow::Result;
use crossterm::{
    execute,
    terminal as term,
    cursor,
};

use crate::views::{
    AcceptsInput,
    BoundingBox,
    playlist::PlaylistScreen,
    Screen,
};
use crate::keybindings::KeyBindings;
use crate::keybindings::{self, KeyBinding};

pub struct App {
    pub playlists: PlaylistScreen,
    pub keybindings: KeyBindings,
}

impl App {
    pub fn new() -> App {
        App {
            playlists: PlaylistScreen::new(),
            keybindings: KeyBindings::new(),
        }
    }

    pub fn start(&mut self) -> Result<()> {
        keybindings::default_keybindings(&mut self.keybindings);

        term::enable_raw_mode()?;
        execute!(
            stdout(),
            term::EnterAlternateScreen,
            cursor::Hide,
        )?;

        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        term::disable_raw_mode()?;
        execute!(
            stdout(),
            term::LeaveAlternateScreen,
            cursor::Show,
        )?;
        Ok(())
    }

    pub fn handle_key(&mut self, key: KeyBinding) -> Result<bool> {
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

    pub fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::Redraw => self.playlists.display(
                BoundingBox { x: 0, y: 0, width: 100, height: 25 }
            )?,
        }
        Ok(())
    }
}

pub enum Action {
    Redraw,
}
