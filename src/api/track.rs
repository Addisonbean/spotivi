use anyhow::Result;

use itertools::Itertools;

use rspotify::model::playlist::PlaylistTrack as PT;
use rspotify::model::track::FullTrack;

use crate::views::Popup;

#[derive(Debug)]
pub struct Track {
    pub full_track: FullTrack,
}

impl Track {
    pub fn new(full_track: FullTrack) -> Track {
        Track { full_track }
    }

    pub fn info_popup(&self) -> Result<Popup> {
        let artist = self.full_track.artists.iter().map(|a| &a.name).join(", ");
        Popup::new(vec![
           format!("Title: {}", self.full_track.name),
           format!("Artist: {}", artist),
        ])
    }
}

#[derive(Debug)]
pub struct PlaylistTrack {
    is_local: bool,
    pub track: Option<Track>,
}

impl PlaylistTrack {
    pub fn new(track: PT) -> PlaylistTrack {
        PlaylistTrack {
            is_local: track.is_local,
            track: track.track.map(Track::new),
        }
    }
}

impl From<PT> for PlaylistTrack {
    fn from(pt: PT) -> PlaylistTrack {
        PlaylistTrack::new(pt)
    }
}
