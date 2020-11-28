use std::io::{stdout, Write};

use anyhow::Result;
use crossterm::{cursor, queue, style, terminal};

use crate::{
    send_request,
    api::{Cursor, PlaylistTrack},
    app::{Action, NetworkRequest},
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
        let height = terminal::size()?.1 as usize;

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

        let offset = self.cursor.offset();
        let lines_drawn = 1;

        for (i, t) in playlist.items().iter().enumerate().skip(offset).take(height - lines_drawn) {
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
            KeyBinding::Enter => {
                let playlists = PLAYLISTS.lock().unwrap();
                let tracks = playlists.get(&self.playlist_id)?.tracks();
                let track = self.cursor.selected_item(&tracks.items()[..])?;
                let uri = track.track.as_ref().unwrap().full_track.uri.clone();

                send_request(NetworkRequest::PlayUri(uri));
                None
            }
            _ => {
                let playlists = PLAYLISTS.lock().unwrap();
                self.cursor.receive_input(input, playlists.get(&self.playlist_id)?.tracks(), terminal::size().ok()?.1 - 1)
            }
        }
    }

    fn notify(&mut self, _action: Action) -> Option<Action> {
        None
    }
}
