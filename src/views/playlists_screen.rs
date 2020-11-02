use std::io::{stdout, Write};

use anyhow::Result;
use crossterm::{cursor, queue, style};

use crate::{
    send_request,
    api::{Cursor, PlaylistSummary},
    app::{Action, NetworkRequest},
    data::PLAYLIST_SUMMARIES,
    keybindings::KeyBinding,
    views::{BoundingBox, Screen},
};

#[derive(Debug)]
pub struct PlaylistsScreen {
    cursor: Cursor<PlaylistSummary>,
}

impl PlaylistsScreen {
    pub fn new() -> PlaylistsScreen {
        PlaylistsScreen { cursor: Cursor::new() }
    }
}

impl Screen for PlaylistsScreen {
    fn display(&self, bounds: BoundingBox) -> Result<()> {
        let playlists = PLAYLIST_SUMMARIES.lock().unwrap();

        queue!(
            stdout(),
            cursor::MoveTo(bounds.x, bounds.y),
            style::Print("Playlists:"),
            cursor::MoveToNextLine(1),
        )?;

        for (i, p) in playlists.items().iter().enumerate() {
            if self.cursor.is_highlighted(i) {
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
                let playlists = PLAYLIST_SUMMARIES.lock().unwrap();
                let id = self.cursor.selected_item(playlists.items())?.id().to_owned();
                send_request(NetworkRequest::LoadPlaylist(id));
                None
            }
            KeyBinding::InfoPopup => {
                let playlists = PLAYLIST_SUMMARIES.lock().unwrap();

                self.cursor
                    .selected_item(playlists.items())
                    .and_then(|p| p.info_popup().ok())
                    .map(Action::Popup)
            }
            KeyBinding::TogglePlayback => {
                send_request(NetworkRequest::TogglePlayback);
                None
            }
            _ => self.cursor.receive_input(input, &*PLAYLIST_SUMMARIES.lock().unwrap()),
        }
    }

    fn notify(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::PlaylistsUpdated => Some(Action::Redraw),
            _ => None,
        }
    }
}
