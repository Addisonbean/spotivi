use rspotify::model::playlist::{FullPlaylist, PlaylistTrack};

use crate::api::{InteractiveList, Paged};

#[derive(Debug)]
pub struct Playlist {
    id: String,
    name: String,
    tracks: Paged<PlaylistTrack>,
}

impl Playlist {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn items(&self) -> &InteractiveList<PlaylistTrack> {
        self.tracks.items()
    }

    // TODO: NOPE
    pub fn items_mut(&mut self) -> &mut InteractiveList<PlaylistTrack> {
        self.tracks.items_mut()
    }
}

impl From<FullPlaylist> for Playlist {
    fn from(p: FullPlaylist) -> Playlist {
        Playlist {
            id: p.id,
            name: p.name,
            tracks: p.tracks.into(),
        }
    }
}
