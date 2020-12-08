use std::io::{stdout, Write};

use anyhow::Result;
use crossterm::{cursor, queue, style, terminal};

use crate::{
    send_request,
    api::Cursor,
    app::{Action, NetworkRequest},
    data::PLAYLIST_SUMMARIES,
    keybindings::KeyBinding,
    views::{BoundingBox, Screen},
};

#[derive(Debug)]
pub struct PlaylistsScreen {
    cursor: Cursor,
}

impl PlaylistsScreen {
    pub fn new() -> PlaylistsScreen {
        PlaylistsScreen { cursor: Cursor::new() }
    }
}

impl Screen for PlaylistsScreen {
    fn display(&self, bounds: BoundingBox) -> Result<()> {
        let height = terminal::size()?.1 as usize;

        let playlists = PLAYLIST_SUMMARIES.lock().unwrap();

        queue!(
            stdout(),
            cursor::MoveTo(bounds.x, bounds.y),
            style::Print("Playlists:"),
            cursor::MoveToNextLine(1),
        )?;

        let lines_drawn = 1;
        self.cursor.queue_draw(
            playlists.items().iter(),
            height - lines_drawn,
            |p| {
                queue!(
                    stdout(),
                    style::Print(p.name()),
                ).map_err(|e| anyhow!(e))
            },
        )?;

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
            // TODO: don't just always subtract 1 here...
            _ => self.cursor.receive_input(input, &*PLAYLIST_SUMMARIES.lock().unwrap(), terminal::size().ok()?.1 - 1),
        }
    }

    fn notify(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::PlaylistsUpdated => Some(Action::Redraw),
            _ => None,
        }
    }
}
