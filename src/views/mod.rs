use std::fmt::Debug;

use anyhow::Result;

use crate::{
    app::Action,
    keybindings::KeyBinding,
};

pub mod playlist;
pub mod playlists;

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

#[derive(Debug)]
pub struct InteractiveList<T> {
    index: usize,
    items: Vec<T>,
}

impl<T> InteractiveList<T> {
    pub fn new() -> InteractiveList<T> {
        InteractiveList { index: 0, items: Vec::new() }
    }

    pub fn from(items: Vec<T>) -> InteractiveList<T> {
        InteractiveList { index: 0, items }
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.items.iter()
    }

    pub fn extend(&mut self, items: impl Iterator<Item=T>) {
        self.items.extend(items);
    }

    pub fn selected_item(&self) -> Option<&T> {
        self.items.get(self.index)
    }

    pub fn select_next(&mut self) {
        if self.items.len() != 0 {
            self.index = std::cmp::min(self.items.len() - 1, self.index + 1);
        } else {
            self.index = 0;
        }
    }

    pub fn select_prev(&mut self) {
        if self.index != 0 {
            self.index -= 1;
        }
    }

    pub fn is_highlighted(&self, i: usize) -> bool {
        self.index == i
    }

    fn receive_input(&mut self, input: KeyBinding) -> Option<Action> {
        match input {
            KeyBinding::Up => {
                self.select_prev();
                Some(Action::Redraw)
            }
            KeyBinding::Down => {
                self.select_next();
                Some(Action::Redraw)
            }
            _ => return None,
        }
    }
}
