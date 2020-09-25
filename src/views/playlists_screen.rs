use std::io::{stdout, Write};

use crossterm::{cursor, queue, style};

use anyhow::Result;

use crate::{
    send_request,
    api::{Paged, PlaylistSummary},
    app::{Action, NetworkRequest},
    keybindings::KeyBinding,
    views::Popup,
};

use super::{BoundingBox, Screen};

#[derive(Debug)]
pub struct PlaylistsScreen {
    playlists: Paged<PlaylistSummary>,
}

impl PlaylistsScreen {
    pub fn new() -> PlaylistsScreen {
        PlaylistsScreen {
            playlists: Paged::new(),
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

        for (i, p) in self.playlists.items().iter().enumerate() {
            if self.playlists.items().is_highlighted(i) {
                queue!(
                    stdout(),
                    style::SetAttribute(style::Attribute::Reverse),
                )?;
            }
            queue!(
                stdout(),
                style::Print(p.name()),
                cursor::MoveToNextLine(1),
                style::SetAttribute(style::Attribute::Reset),
            )?;
        }

        stdout().flush()?;

        Ok(())
    }

    fn receive_input(&mut self, input: KeyBinding) -> Option<Action> {
        match input {
            KeyBinding::Enter => {
                let id = self.playlists.items().selected_item()?.id().to_owned();
                send_request(NetworkRequest::LoadPlaylist(id));
                None
            }
            KeyBinding::InfoPopup => {
                if let Some(playlist) = self.playlists.items().selected_item() {
                    let p = Popup::new(vec![
                        format!("Name: {}", playlist.name()),
                        "TODO: add more info...".to_owned(),
                    ]).unwrap();
                    Some(Action::Popup(p))
                } else {
                    None
                }
            }
            _ => self.playlists.receive_input(input),
        }
    }

    fn notify(&mut self, action: Action) -> Result<()> {
        match action {
            Action::AddPlaylists(p) => {
                self.playlists.add_page(p);
                self.display(
                    BoundingBox { x: 0, y: 0, width: 100, height: 25 }
                )?;
            }
            _ => {},
        }
        Ok(())
    }
}
