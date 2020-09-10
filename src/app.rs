use std::io::{stdout, Write};

use anyhow::{Context, Result};
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
use crate::keybindings::KeyBinding;

pub struct App {
    playlists: PlaylistScreen,
}

impl App {
    pub fn new() -> App {
        App {
            playlists: PlaylistScreen::new(),
        }
    }

    pub fn start(&mut self) -> Result<()> {
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

    pub fn handle_key(&mut self, key: KeyBinding) -> Result<Action> {
        Ok(match key {
            KeyBinding::Quit => {
                Action::Quit
            }
            _ => {
                self.playlists.receive_input(key).context("TODO")?
            }
        })
    }

    pub fn handle_action(&mut self, action: Action) -> Result<bool> {
        match action {
            Action::AddPlaylists(ps) => {
                self.playlists.add_playlists(ps.into_iter());
                // TODO: only display if visible...
                self.playlists.display(
                    BoundingBox { x: 0, y: 0, width: 100, height: 25 }
                )?;
            }
            Action::Redraw =>
                // TODO: redraw current screen only...
                self.playlists.display(
                    BoundingBox { x: 0, y: 0, width: 100, height: 25 }
                )?,

            Action::Quit => {
                self.stop()?;
                return Ok(false);
            },
        }
        Ok(true)
    }
}

#[derive(Debug)]
pub enum Action {
    AddPlaylists(Vec<String>),
    Redraw,
    Quit,
}
