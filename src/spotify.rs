use anyhow::Result;
use rspotify::client::Spotify;
use rspotify::oauth2::SpotifyClientCredentials;
use rspotify::oauth2::SpotifyOAuth;

use crate::config::Config;

pub struct SpotifyApi {
    client: Spotify,
}

impl SpotifyApi {
    pub async fn new(config: &Config) -> Option<SpotifyApi> {
        Some(SpotifyApi {
            client: make_spotify_client(config).await?,
        })
    }

    pub async fn get_playlists(&mut self) -> Result<impl Iterator<Item=String>> {
        let page = self.client.current_user_playlists(8, 0).await.map_err(|e| anyhow!(e))?;
        Ok(page
            .items
            .into_iter()
            .map(|p| p.name)
        )
    }
}

async fn make_spotify_client(config: &Config) -> Option<Spotify> {
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