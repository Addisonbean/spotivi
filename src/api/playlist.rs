use rspotify::model::playlist::{FullPlaylist, PlaylistTrack};

use crate::api::Paged;

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

    pub fn items(&self) -> &Vec<PlaylistTrack> {
        self.tracks.items()
    }

    pub fn tracks(&self) -> &Paged<PlaylistTrack> {
        &self.tracks
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
