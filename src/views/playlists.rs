use std::io::{stdout, Write};

use crossterm::{cursor, queue, style};

use anyhow::Result;

use crate::{
    api::Playlist,
    app::Action,
    keybindings::KeyBinding,
    views::playlist::PlaylistScreen,
};

use super::{BoundingBox, Screen, InteractiveList};

#[derive(Debug)]
pub struct PlaylistsScreen {
    playlists: InteractiveList<String>,
}

impl PlaylistsScreen {
    pub fn new() -> PlaylistsScreen {
        PlaylistsScreen {
            playlists: InteractiveList::new(),
        }
    }
}

impl Screen for PlaylistsScreen {
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

    fn receive_input(&mut self, input: KeyBinding) -> Option<Action> {
        Some(match input {
            KeyBinding::Enter => {
                let p = Playlist::new(
                    self.playlists.selected_item()?.clone(),
                    vec!["heyo".to_owned(), "there".to_owned()],
                );
                Action::PushScreen(Box::new(PlaylistScreen::new(p)))
            }
            _ => return self.playlists.receive_input(input),
        })
    }

    fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::AddPlaylists(ps) => {
                self.playlists.extend(ps.into_iter());
                self.display(
                    BoundingBox { x: 0, y: 0, width: 100, height: 25 }
                )?;
            }
            _ => {},
        }
        Ok(())
    }
}
