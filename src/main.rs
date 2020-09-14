use std::{
    process::exit,
    sync::{Arc, Mutex},
};

#[macro_use]
extern crate anyhow;

use anyhow::Result;
use crossterm::event;
use tokio::{self, sync::mpsc};

mod api;
mod app;
mod config;
mod keybindings;
mod spotify;
mod views;
use app::{App, Action};
use config::Config;
use spotify::SpotifyApi;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new()?;

    let mut client = SpotifyApi::new(&config).await.expect("auth failed...");

    let mut app = App::new();
    app.start()?;

    let app = Arc::new(Mutex::new(app));

    // TODO: why 20? Is that enough? Too much???
    // tx.send() docs should help
    let (mut tx, mut rx) = mpsc::channel(20);
    let mut api_tx = tx.clone();
    tokio::spawn(async move {
        let ps = client.get_playlists().await.unwrap();
        let action = Action::AddPlaylists(ps.collect());

        api_tx.send(action).await.unwrap();
    });

    let app_keyhandler = Arc::clone(&app);
    tokio::spawn(async move {
        loop {
            if let Ok(event::Event::Key(e)) = event::read() {
                if let Some(&key) = config.keybindings.get(&e) {
                    // TODO: don't unwrap here...
                    let action = app_keyhandler.lock().unwrap().handle_key(key).unwrap();
                    if let Some(a) = action {
                        tx.send(a).await.unwrap();
                    }
                }
            }
        }
    });

    while let Some(action) = rx.recv().await {
        if !app.lock().unwrap().handle_action(action)? {
            exit(0);
        }
    }

    Ok(())
}
