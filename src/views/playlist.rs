use std::io::{stdout, Write};

use crossterm::{cursor, queue, style};

use anyhow::Result;

use crate::{
    app::Action,
    keybindings::KeyBinding,
};

use super::{AcceptsInput, BoundingBox, Screen, InteractiveList};

pub struct PlaylistScreen {
    playlists: InteractiveList<String>,
}

impl PlaylistScreen {
    pub fn new() -> PlaylistScreen {
        PlaylistScreen {
            playlists: InteractiveList::new(Vec::new()),
        }
    }

    pub fn add_playlists(&mut self, items: impl Iterator<Item=String>) {
        self.playlists.items.extend(items);
    }
}

impl Screen for PlaylistScreen {
    fn display(&self, bounds: BoundingBox) -> Result<()> {
        queue!(
            stdout(),
            cursor::MoveTo(bounds.x, bounds.y),
            style::Print("Playlists:"),
            cursor::MoveToNextLine(1),
        )?;

        for (i, p) in self.playlists.items.iter().enumerate() {
            if self.playlists.is_highlighted(i) {
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
}

impl AcceptsInput for PlaylistScreen {
    fn receive_input(&mut self, input: KeyBinding) -> Option<Action> {
        self.playlists.receive_input(input)
    }
}
