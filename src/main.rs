use std::{
    sync::Arc,
    io::{stdout, Write},
};

use tokio;
use std::sync::Mutex;

#[macro_use]
extern crate anyhow;

use anyhow::Result;

use crossterm::{
    execute,
    terminal as term,
    event,
    cursor,
};

mod config;
mod keybindings;
mod spotify;
mod views;
use config::Config;
use keybindings::{KeyBinding, KeyBindings};
use spotify::SpotifyApi;
use views::{BoundingBox, Screen, AcceptsInput, playlist::PlaylistScreen};

pub enum Action {
    Redraw,
}

struct App {
    playlists: PlaylistScreen,
    keybindings: KeyBindings,
}

impl App {
    fn start(&mut self) -> Result<()> {
        keybindings::default_keybindings(&mut self.keybindings);

        term::enable_raw_mode()?;
        execute!(
            stdout(),
            term::EnterAlternateScreen,
            cursor::Hide,
        )?;

        Ok(())
    }

    fn stop(&self) -> Result<()> {
        term::disable_raw_mode()?;
        execute!(
            stdout(),
            term::LeaveAlternateScreen,
            cursor::Show,
        )?;
        Ok(())
    }

    fn handle_key(&mut self, key: KeyBinding) -> Result<bool> {
        match key {
            KeyBinding::Quit => {
                self.stop()?;
                return Ok(false);
            }
            _ => {
                if let Some(res) = self.playlists.receive_input(key) {
                    self.handle_action(res)?;
                }
            }
        }
        Ok(true)
    }

    fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::Redraw => self.playlists.display(
                BoundingBox { x: 0, y: 0, width: 100, height: 25 }
            )?,
        }
        Ok(())
    }
}

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
