// use std::sync::{Arc, Mutex, RwLock};
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, Mutex as TokioMutex, RwLock};

#[macro_use]
extern crate anyhow;

use anyhow::Result;
use crossterm::event;
use lazy_static::lazy_static;

mod api;
mod app;
mod config;
pub mod data;
mod keybindings;
mod views;

use api::{SpotifyApi, PageId};
use app::{App, Action, NetworkRequest};
use config::Config;
use data::{add_playlist, add_playlist_summaries, PLAYLIST_SUMMARIES};
use views::PlaylistScreen;

lazy_static! {
    static ref CHANNEL: (mpsc::Sender<NetworkRequest>, TokioMutex<mpsc::Receiver<NetworkRequest>>) = {
        // TODO: why 10???
        let (tx, rx) = mpsc::channel(10);
        (tx, TokioMutex::new(rx))
    };
}

pub fn send_request(r: NetworkRequest) {
    tokio::spawn(async move {
        CHANNEL.0.clone().send(r).await.unwrap();
    });
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Arc::new(Config::new()?);
    let api = Arc::new(RwLock::new(SpotifyApi::new(Arc::clone(&config)).await.expect("auth failed...")));

    let app = Arc::new(Mutex::new(App::new(Arc::clone(&config))));
    app.lock().unwrap().start()?;
    let app_init = Arc::clone(&app);
    let api_init = api.clone();
    tokio::spawn(async move {
        init(app_init, api_init).await.unwrap();
    });

    let app_handler = Arc::clone(&app);
    tokio::spawn(async move {
        let mut rx = CHANNEL.1.lock().await;
        while let Some(r) = rx.recv().await {
            match r {
                NetworkRequest::LoadNextPage(page_id) => {
                    let api = Arc::clone(&api);
                    let app = Arc::clone(&app_handler);
                    tokio::spawn(async move {
                        match page_id {
                            PageId::Playlists => {
                                let index = {
                                    let ps = PLAYLIST_SUMMARIES.lock().unwrap();
                                    ps.next_page().map(|np| np.index)
                                };
                                if let Some(index) = index {
                                    let p = api.read().await.get_playlists(index).await.unwrap();
                                    add_playlist_summaries(p);

                                    let mut app = app.lock().unwrap();
                                    app.handle_action(Action::PlaylistsUpdated).unwrap();
                                }
                            }
                            PageId::Playlist(_) => {}
                        }
                    });
                }
                NetworkRequest::LoadPlaylist(id) => {
                    let api = Arc::clone(&api);
                    let app = Arc::clone(&app_handler);
                    tokio::spawn(async move {
                        let p = api.read().await.get_playlist(&id).await.unwrap();
                        let id = p.id().to_owned();
                        add_playlist(p);

                        let screen = Box::new(PlaylistScreen::new(id));

                        // TODO: just notifiy don't add a screen
                        app.lock().unwrap().add_screen(screen).unwrap();
                    });
                }
                NetworkRequest::GetDevices => {
                    let api1 = Arc::clone(&api);
                    let api2 = Arc::clone(&api);
                    tokio::spawn(async move {
                        let device_id = api1.read().await
                            .get_devices()
                            .await
                            .unwrap()
                            .into_iter()
                            .next()
                            .map(|d| d.id);

                        api1.write().await.set_device_id(device_id);
                    });
                    tokio::spawn(async move {
                        let is_playing = api2.read().await
                            .playback_status()
                            .await
                            .unwrap()
                            .map(|ctx| ctx.is_playing)
                            .unwrap_or(false);

                        api2.write().await.set_playing(is_playing);
                    });
                }
                NetworkRequest::TogglePlayback => {
                    use rspotify::client::ApiError;
                    let api = Arc::clone(&api);
                    tokio::spawn(async move {
                        match api.write().await.toggle_playback().await {
                            _ => {}
                        };
                    });
                }
            }
        }
    });

    send_request(NetworkRequest::GetDevices);

    loop {
        if let Ok(event::Event::Key(e)) = event::read() {
            // TODO: What to do when in a text box or something???
            // Probably send not only the KeyBinding but also the raw key entered, ya that...
            if let Some(&key) = config.keybindings.get(&e) {
                app.lock().unwrap().handle_key(key)?;
            }
        }
    }
}

pub async fn init(app: Arc<Mutex<App>>, api: Arc<RwLock<SpotifyApi>>) -> Result<()> {
    let p = api.read().await.get_playlists(0).await.unwrap();
    add_playlist_summaries(p);
    app.lock().unwrap().handle_action(Action::PlaylistsUpdated)?;
    Ok(())
}
