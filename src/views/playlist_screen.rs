use std::io::{stdout, Write};

use crossterm::{cursor, queue, style};

use anyhow::Result;

use crate::{
    api::Playlist,
    keybindings::KeyBinding,
};
use super::{BoundingBox, Screen, Action};

#[derive(Debug)]
pub struct PlaylistScreen {
    pub playlist: Playlist,
}

impl PlaylistScreen {
    pub fn new(playlist: Playlist) -> PlaylistScreen {
        PlaylistScreen { playlist }
    }
}

impl Screen for PlaylistScreen {
    fn display(&self, bounds: BoundingBox) -> Result<()> {
        queue!(
            stdout(),
            cursor::MoveTo(bounds.x, bounds.y),
            style::Print(self.playlist.name()),
            cursor::MoveToNextLine(1),
        )?;

        for (i, p) in self.playlist.items().iter().enumerate() {
            if self.playlist.items().is_highlighted(i) {
                queue!(
                    stdout(),
                    style::SetAttribute(style::Attribute::Reverse),
                )?;
            }
            queue!(
                stdout(),
                style::Print(p),
                cursor::MoveToNextLine(1),
                style::SetAttribute(style::Attribute::Reset),
            )?;
        }

        stdout().flush()?;

        Ok(())
    }

    fn receive_input(&mut self, input: KeyBinding) -> Option<Action> {
        self.playlist.items_mut().receive_input(input)
    }

    fn handle_action(&mut self, _action: Action) -> Result<()> {
        Ok(())
    }
}
