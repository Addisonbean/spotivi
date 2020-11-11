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

use crate::api::PageId;
use crate::views::{
    BoundingBox,
    PlaylistsScreen,
    Screen,
    Popup,
};
use crate::keybindings::KeyBinding;
use crate::config::Config;

pub struct App {
    screens: Vec<Box<dyn Screen + Send>>,
    config: Arc<Config>,
    popup: Option<Popup>,
}

impl App {
    pub fn new(config: Arc<Config>) -> App {
        let screens = vec![Box::new(PlaylistsScreen::new()) as Box<dyn Screen + Send>];
        App {
            screens,
            config,
            popup: None,
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
        if self.popup.is_some() {
            self.popup = None;
            self.redraw()?;
            return Ok(());
        }
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

        let (w, h) = term::size()?;
        self.current_screen().display(
            BoundingBox { x: 0, y: 0, width: w, height: h }
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
            Action::Popup(popup) => {
                self.display_popup(popup)?;
            },
            _ => {
                if let Some(action) = self.current_screen_mut().notify(action) {
                    return self.handle_action(action);
                }
            }
        }
        Ok(true)
    }

    pub fn add_screen(&mut self, s: Box<dyn Screen + Send + Sync>) -> Result<()> {
        self.screens.push(s);
        self.redraw()
    }

    pub fn display_popup(&mut self, popup: Popup) -> Result<()> {
        popup.display()?;
        self.popup = Some(popup);
        Ok(())
    }
}

#[derive(Debug)]
pub enum Action {
    PlaylistsUpdated,
    Redraw,
    Quit,
    Key(KeyBinding),
    Popup(Popup),
}

#[derive(Debug)]
pub enum NetworkRequest {
    LoadNextPage(PageId),
    LoadPlaylist(String),
    GetDevices,
    TogglePlayback,
    PlayUri(String),
}
