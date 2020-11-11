use std::sync::Arc;

use anyhow::Result;
use rspotify::client::Spotify;
use rspotify::model::device::Device;
use rspotify::model::context::CurrentlyPlaybackContext;
use rspotify::oauth2::SpotifyClientCredentials;
use rspotify::oauth2::SpotifyOAuth;

use crate::config::Config;
use crate::api::{Paged, PageId, Playlist, PlaylistSummary};

pub const PAGE_SIZE: u32 = 8;

pub struct SpotifyApi {
    client: Spotify,
    device_id: Option<String>,
    playing: bool,
}

impl SpotifyApi {
    pub async fn new(config: Arc<Config>) -> Option<SpotifyApi> {
        Some(SpotifyApi {
            client: make_spotify_client(config).await?,
            device_id: None,
            playing: false,
        })
    }

    pub async fn get_playlists(&self, page_num: u32) -> Result<Paged<PlaylistSummary>> {
        self.client
            .current_user_playlists(PAGE_SIZE, PAGE_SIZE * page_num)
            .await
            .map_err(|e| anyhow!(e))
            .map(|p| Paged::from(p, PageId::Playlists))
    }

    pub async fn get_playlist(&self, id: &str) -> Result<Playlist> {
        self.client
            .playlist(id, None, None)
            .await
            .map_err(|e| anyhow!(e))
            .map(Into::into)
    }

    pub async fn get_devices(&self) -> Result<Vec<Device>> {
        self.client
            .device()
            .await
            .map_err(|e| anyhow!(e))
            .map(|p| p.devices)
    }

    pub fn set_device_id(&mut self, id: Option<String>) {
        self.device_id = id;
    }

    pub async fn toggle_playback(&mut self) -> Result<()> {
        if self.playing {
            self.pause_playback().await
        } else {
            self.resume_playback().await
        }
    }

    pub async fn resume_playback(&mut self) -> Result<()> {
        self.client
            .start_playback(
                self.device_id.clone(),
                None,
                None,
                None,
                None,
            )
            .await
            .map_err(|e| anyhow!(e))?;

        self.playing = true;
        Ok(())
    }

    pub async fn pause_playback(&mut self) -> Result<()> {
        self.client
            .pause_playback(self.device_id.clone())
            .await
            .map_err(|e| anyhow!(e))?;

        self.playing = false;
        Ok(())
    }

    pub async fn playback_status(&self) -> Result<Option<CurrentlyPlaybackContext>> {
        self.client
            .current_playback(None, None)
            .await
            .map_err(|e| anyhow!(e))
    }

    pub fn set_playing(&mut self, playing: bool) {
        self.playing = playing;
    }

    pub async fn play_from_uri(&mut self, uri: String) -> Result<()> {
        self.client
            .start_playback(
                self.device_id.clone(),
                None,
                Some(vec![uri]),
                None,
                None,
            )
            .await
            .map_err(|e| anyhow!(e))?;

        self.playing = true;
        Ok(())
    }
}

async fn make_spotify_client(config: Arc<Config>) -> Option<Spotify> {
    let mut oauth = SpotifyOAuth::default()
        .scope("streaming user-modify-playback-state user-read-playback-state playlist-read-private")
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
