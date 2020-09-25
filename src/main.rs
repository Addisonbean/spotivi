use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, Mutex as TokioMutex};

#[macro_use]
extern crate anyhow;

use anyhow::Result;
use crossterm::event;
use lazy_static::lazy_static;

mod api;
mod app;
mod config;
mod keybindings;
mod views;

use api::SpotifyApi;
use app::{App, Action, NetworkRequest};
use config::Config;
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
    let api = Arc::new(SpotifyApi::new(Arc::clone(&config)).await.expect("auth failed..."));

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
                NetworkRequest::LoadPlaylistsPage(index) => {
                    let api = Arc::clone(&api);
                    let app = Arc::clone(&app_handler);
                    tokio::spawn(async move {
                        let p = api.get_playlists(index).await.unwrap();
                        // TODO: instead add playlists to the app object and
                        // like just tell the screen playlists have changed
                        let mut app = app.lock().unwrap();
                        app.handle_action(Action::AddPlaylists(p)).unwrap();
                    });
                }
                NetworkRequest::LoadPlaylist(id) => {
                    let api = Arc::clone(&api);
                    let app = Arc::clone(&app_handler);
                    tokio::spawn(async move {
                        let p = api.get_playlist(&id).await.unwrap();
                        let screen = Box::new(PlaylistScreen::new(p));

                        // TODO: just notifiy don't add a screen
                        app.lock().unwrap().add_screen(screen).unwrap();
                    });
                }
            }
        }
    });

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

pub async fn init(app: Arc<Mutex<App>>, api: Arc<SpotifyApi>) -> Result<()> {
    let p = api.get_playlists(0).await.unwrap();
    app.lock().unwrap().handle_action(Action::AddPlaylists(p))?;
    Ok(())
}
