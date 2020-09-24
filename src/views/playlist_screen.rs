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

        for (i, t) in self.playlist.items().iter().enumerate() {
            if self.playlist.items().is_highlighted(i) {
                queue!(
                    stdout(),
                    style::SetAttribute(style::Attribute::Reverse),
                )?;
            }
            let name = if let Some(ref track) = t.track {
                &track.name
            } else {
                "(N/A)"
            };
            queue!(
                stdout(),
                style::Print(name),
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

    fn notify(&mut self, _action: Action) -> Result<()> {
        Ok(())
    }
}
