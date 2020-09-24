use std::sync::Arc;

use anyhow::Result;
use rspotify::client::Spotify;
use rspotify::oauth2::SpotifyClientCredentials;
use rspotify::oauth2::SpotifyOAuth;

use crate::config::Config;
use crate::api::{Paged, Playlist, PlaylistSummary};

pub const PAGE_SIZE: u32 = 8;

pub struct SpotifyApi {
    client: Spotify,
}

impl SpotifyApi {
    pub async fn new(config: Arc<Config>) -> Option<SpotifyApi> {
        Some(SpotifyApi {
            client: make_spotify_client(config).await?,
        })
    }

    pub async fn get_playlists(&self, page_num: u32) -> Result<Paged<PlaylistSummary>> {
        self.client
            .current_user_playlists(PAGE_SIZE, PAGE_SIZE * page_num)
            .await
            .map_err(|e| anyhow!(e))
            .map(Into::into)
    }

    pub async fn get_playlist(&self, id: &str) -> Result<Playlist> {
        self.client
            .playlist(id, None, None)
            .await
            .map_err(|e| anyhow!(e))
            .map(Into::into)
    }
}

async fn make_spotify_client(config: Arc<Config>) -> Option<Spotify> {
    let mut oauth = SpotifyOAuth::default()
        .client_id(&config.api_client_id)
        .client_secret(&config.api_client_secret)
        .redirect_uri(&config.redirect_uri)
        .cache_path(config.cache_path.clone())
        .build();
    let token = rspotify::util::get_token(&mut oauth).await?;
    let credentials = SpotifyClientCredentials::default()
        .token_info(token)
        .build();
    Some(Spotify::default()
        .client_credentials_manager(credentials)
        .build())
}
