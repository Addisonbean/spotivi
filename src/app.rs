use std::io::{stdout, Write};
use std::process::exit;
use std::sync::Arc;

use anyhow::Result;
use crossterm::{
    execute,
    terminal as term,
    cursor,
    queue,
};

use crate::views::{
    BoundingBox,
    PlaylistsScreen,
    Screen,
};
use crate::api::{Paged, PlaylistSummary};
use crate::keybindings::KeyBinding;
use crate::config::Config;

pub struct App {
    screens: Vec<Box<dyn Screen + Send>>,
    config: Arc<Config>,
}

impl App {
    pub fn new(config: Arc<Config>) -> App {
        let screens = vec![Box::new(PlaylistsScreen::new()) as Box<dyn Screen + Send>];
        App {
            screens,
            config,
        }
    }

    fn current_screen(&self) -> &dyn Screen {
        self.screens.last().unwrap().as_ref()
    }

    fn current_screen_mut(&mut self) -> &mut dyn Screen {
        self.screens.last_mut().unwrap().as_mut()
    }

    pub fn start(&mut self) -> Result<()> {
        term::enable_raw_mode()?;
        execute!(
            stdout(),
            term::EnterAlternateScreen,
            cursor::Hide,
        )?;

        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        term::disable_raw_mode()?;
        execute!(
            stdout(),
            term::LeaveAlternateScreen,
            cursor::Show,
        )?;
        Ok(())
    }

    pub fn handle_key(&mut self, key: KeyBinding) -> Result<()> {
        match key {
            KeyBinding::Quit => {
                self.stop()?;
                exit(0);
            }
            _ => {
                if let Some(a) = self.current_screen_mut().receive_input(key) {
                    self.handle_action(a).unwrap();
                }
            }
        }
        Ok(())
    }

    pub fn redraw(&mut self) -> Result<()> {
        queue!(stdout(), term::Clear(term::ClearType::All))?;

        self.current_screen().display(
            BoundingBox { x: 0, y: 0, width: 100, height: 25 }
        )?;

        Ok(())
    }

    pub fn handle_action(&mut self, action: Action) -> Result<bool> {
        match action {
            Action::Redraw => {
                self.redraw()?;
            }
            Action::Quit => {
                self.stop()?;
                return Ok(false);
            },
            _ => self.notify(action)?,
        }
        Ok(true)
    }

    pub fn notify(&mut self, action: Action) -> Result<()> {
        self.current_screen_mut().notify(action)
    }

    pub fn add_screen(&mut self, s: Box<dyn Screen + Send + Sync>) -> Result<()> {
        self.screens.push(s);
        self.redraw()
    }
}

#[derive(Debug)]
pub enum Action {
    AddPlaylists(Paged<PlaylistSummary>),
    Redraw,
    Quit,
    LoadScreen(ScreenId),
    PushScreen(Box<dyn Screen + Send + Sync>),
    Key(KeyBinding),
}

#[derive(Debug)]
pub enum NetworkRequest {
    LoadPlaylistsPage(u32),
    LoadPlaylist(String),
}

#[derive(Debug)]
pub enum ScreenId {
    Playlist(String),
}
