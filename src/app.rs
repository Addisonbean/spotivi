use std::io::{stdout, Write};
use std::process::exit;
use std::sync::{
    Arc,
    mpsc::{
        self,
        Sender,
        Receiver,
    },
};

use anyhow::Result;
use crossterm::{
    event,
    execute,
    terminal as term,
    cursor,
};

use crate::views::{
    BoundingBox,
    playlists::PlaylistsScreen,
    Screen,
};
use crate::api::{Paged, PlaylistSummary, ApiRequest};
use crate::keybindings::KeyBinding;

use crate::config::Config;
use crate::spotify::SpotifyApi;

pub struct App {
    screens: Vec<Box<dyn Screen + Send>>,
    api: Arc<SpotifyApi>,
    config: Arc<Config>,
    sender: Sender<Action>,
    receiver: Receiver<Action>,
}

impl App {
    pub fn new(config: Config, api: SpotifyApi) -> App {
        let (sender, receiver) = mpsc::channel();
        let screens = vec![Box::new(PlaylistsScreen::new()) as Box<dyn Screen + Send>];
        App {
            screens,
            api: Arc::new(api),
            config: Arc::new(config),
            sender,
            receiver,
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

    pub fn init_data(&self) {
        let api = Arc::clone(&self.api);
        let tx = self.sender.clone();
        tokio::spawn(async move {
            let ps = api.get_playlists().await.unwrap();
            let action = Action::AddPlaylists(ps);
            tx.send(action).unwrap();
        });
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
            Action::Key(k) => {
                if let Some(a) = self.handle_key(k)? {
                    return self.handle_action(a);
                }
            }
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
            Action::Request(_r) => {
                // tokio::spawn(async move {
                // });
            }
            _ => self.current_screen_mut().handle_action(action)?,
        }
        Ok(true)
    }

    pub fn handle_keys(&self) {
        let config = Arc::clone(&self.config);
        let tx = self.sender.clone();
        tokio::spawn(async move {
            loop {
                if let Ok(event::Event::Key(e)) = event::read() {
                    if let Some(&key) = config.keybindings.get(&e) {
                        tx.send(Action::Key(key)).unwrap();
                    }
                }
            }
        });
    }

    pub async fn main_loop(&mut self) -> Result<()> {
        loop {
            let action = self.receiver.recv()?;
            if !self.handle_action(action)? {
                exit(0);
            }
        }
    }
}

#[derive(Debug)]
pub enum Action {
    AddPlaylists(Paged<PlaylistSummary>),
    Redraw,
    Quit,
    PushScreen(Box<dyn Screen + Send>),
    Request(ApiRequest),
    Key(KeyBinding),
}
