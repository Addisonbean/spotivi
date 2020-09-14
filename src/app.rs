use std::io::{stdout, Write};

use anyhow::Result;
use crossterm::{
    execute,
    terminal as term,
    cursor,
};

use crate::views::{
    BoundingBox,
    playlists::PlaylistsScreen,
    Screen,
};
use crate::keybindings::KeyBinding;

pub struct App {
    screens: Vec<Box<dyn Screen + Send>>,
}

impl App {
    pub fn new() -> App {
        App {
            screens: vec![Box::new(PlaylistsScreen::new())],
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

    pub fn handle_key(&mut self, key: KeyBinding) -> Result<Option<Action>> {
        Ok(match key {
            KeyBinding::Quit => {
                Some(Action::Quit)
            }
            _ => {
                self.current_screen_mut().receive_input(key)
            }
        })
    }

    pub fn redraw(&mut self) -> Result<()> {
        execute!(stdout(), term::Clear(term::ClearType::All))?;

        self.current_screen().display(
            BoundingBox { x: 0, y: 0, width: 100, height: 25 }
        )?;

        Ok(())
    }

    pub fn handle_action(&mut self, action: Action) -> Result<bool> {
        match action {
            Action::PushScreen(s) => {
                self.screens.push(s);
                self.redraw()?;
            }
            Action::Redraw => {
                self.redraw()?;
            }
            Action::Quit => {
                self.stop()?;
                return Ok(false);
            },
            _ => self.current_screen_mut().handle_action(action)?,
        }
        Ok(true)
    }
}

#[derive(Debug)]
pub enum Action {
    AddPlaylists(Vec<String>),
    Redraw,
    Quit,
    PushScreen(Box<dyn Screen + Send>),
}
