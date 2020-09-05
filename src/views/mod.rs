use crossterm::Result;

use crate::{Action, keybindings::KeyBinding};

pub mod playlist;

pub struct BoundingBox {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

pub trait AcceptsInput {
    fn receive_input(&mut self, input: KeyBinding) -> Option<Action>;
}

pub trait Screen {
    fn display(&self, bounds: BoundingBox) -> Result<()>;
}

pub struct InteractiveList<T> {
    index: usize,
    items: Vec<T>,
}

impl<T> InteractiveList<T> {
    pub fn new(items: Vec<T>) -> InteractiveList<T> {
        InteractiveList { index: 0, items }
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
}

impl<T> AcceptsInput for InteractiveList<T> {
    fn receive_input(&mut self, input: KeyBinding) -> Option<Action> {
        match input {
            KeyBinding::Up => self.select_prev(),
            KeyBinding::Down => self.select_next(),
            _ => return None,
        }
        Some(Action::Redraw)
    }
}
