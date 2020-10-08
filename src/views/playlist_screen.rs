use std::io::{stdout, Write};

use anyhow::Result;
use crossterm::{cursor, queue, style};

use crate::{
    api::{Cursor, PlaylistTrack},
    app::Action,
    data::PLAYLISTS,
    keybindings::KeyBinding,
    views::{BoundingBox, Screen},
};

#[derive(Debug)]
pub struct PlaylistScreen {
    pub playlist_id: String,
    pub cursor: Cursor<PlaylistTrack>,
}

impl PlaylistScreen {
    pub fn new(playlist_id: String) -> PlaylistScreen {
        PlaylistScreen { playlist_id, cursor: Cursor::new() }
    }
}

impl Screen for PlaylistScreen {
    fn display(&self, bounds: BoundingBox) -> Result<()> {
        let playlists = PLAYLISTS.lock().unwrap();
        let playlist = match playlists.get(&self.playlist_id) {
            Some(p) => p,
            None => return Ok(()),
        };

        queue!(
            stdout(),
            cursor::MoveTo(bounds.x, bounds.y),
            style::Print(playlist.name()),
            cursor::MoveToNextLine(1),
        )?;

        for (i, t) in playlist.items().iter().enumerate() {
            if self.cursor.is_highlighted(i) {
                queue!(
                    stdout(),
                    style::SetAttribute(style::Attribute::Reverse),
                )?;
            }
            let name = if let Some(ref track) = t.track {
                &track.full_track.name
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
        match input {
            KeyBinding::InfoPopup => {
                let playlists = PLAYLISTS.lock().unwrap();
                let tracks = playlists.get(&self.playlist_id)?.tracks();
                let track = self.cursor.selected_item(&tracks.items()[..])?;

                track.track
                    .as_ref()
                    .and_then(|p| p.info_popup().ok())
                    .map(Action::Popup)
            }
            _ => {
                let playlists = PLAYLISTS.lock().unwrap();
                self.cursor.receive_input(input, playlists.get(&self.playlist_id)?.tracks())
            }
        }
    }

    fn notify(&mut self, _action: Action) -> Option<Action> {
        None
    }
}
