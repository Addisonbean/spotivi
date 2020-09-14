use std::fmt::Debug;

use anyhow::Result;

use crate::{
    app::Action,
    keybindings::KeyBinding,
};

// pub mod playlist_screen;
mod playlists_screen;
pub use playlists_screen::PlaylistsScreen;

pub struct BoundingBox {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

pub trait Screen: Debug {
    fn display(&self, bounds: BoundingBox) -> Result<()>;
    fn receive_input(&mut self, input: KeyBinding) -> Option<Action>;
    fn handle_action(&mut self, action: Action) -> Result<()>;
}
