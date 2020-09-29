use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::api::{
    Paged,
    PageId,
    Playlist,
    PlaylistSummary,
};

lazy_static! {
    pub static ref PLAYLIST_SUMMARIES: Arc<Mutex<Paged<PlaylistSummary>>> = Arc::new(Mutex::new(Paged::new(PageId::Playlists)));
    pub static ref PLAYLISTS: Arc<Mutex<HashMap<String, Playlist>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn add_playlist(p: Playlist) {
    let mut playlists = PLAYLISTS.lock().unwrap();
    playlists.insert(p.id().to_owned(), p);
}

pub fn add_playlist_summaries(p: Paged<PlaylistSummary>) {
    let mut playlists = PLAYLIST_SUMMARIES.lock().unwrap();
    playlists.add_page(p);
}
