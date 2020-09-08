use std::sync::{Arc, Mutex};

#[macro_use]
extern crate anyhow;

use anyhow::Result;
use crossterm::event;
use tokio;

mod app;
mod config;
mod keybindings;
mod spotify;
mod views;
use app::App;
use config::Config;
use keybindings::KeyBindings;
use spotify::SpotifyApi;
use views::{BoundingBox, Screen};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new()?;

    let mut client = SpotifyApi::new(&config).await.expect("auth failed...");

    let mut app = App {
        playlists: views::playlist::PlaylistScreen::new(),
        keybindings: KeyBindings::new(),
    };
    app.start()?;

    let app = Arc::new(Mutex::new(app));

    // TODO: use message passing instead?
    let api_app = Arc::clone(&app);
    tokio::spawn(async move {
        let ps = client.get_playlists().await.unwrap();

        let mut app = api_app.lock().unwrap();
        app.playlists.add_playlists(ps);
        // TODO: move this... (and maybe the line above???)
        app.playlists.display(
            BoundingBox { x: 0, y: 0, width: 100, height: 25 }
        ).unwrap();
    });

    {
        let app = app.lock().unwrap();
        app.playlists.display(
            BoundingBox { x: 0, y: 0, width: 100, height: 25 }
        )?;
    }

    loop {
        if let Ok(event::Event::Key(e)) = event::read() {
            let mut app = app.lock().unwrap();
            if let Some(&key) = app.keybindings.get(&e) {
                if !app.handle_key(key)? {
                    return Ok(());
                }
            }
        }
    }
}
