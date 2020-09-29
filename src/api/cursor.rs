use std::marker::PhantomData;

use crate::api::spotify_api::PAGE_SIZE;

use crate::{
    api::Paged,
    app::{Action, NetworkRequest},
    keybindings::KeyBinding,
};

#[derive(Debug)]
pub struct Cursor<T> {
    index: usize,
    _phantom: PhantomData<T>,
}

impl<T> Cursor<T> {
    pub fn new() -> Cursor<T> {
        Cursor { index: 0, _phantom: PhantomData }
    }

    pub fn get<'a>(&self, index: usize, items: &'a [T]) -> Option<&'a T> {
        items.get(index)
    }

    pub fn selected_item<'a>(&self, items: &'a [T]) -> Option<&'a T> {
        items.get(self.index)
    }

    pub fn select_next(&mut self, len: usize) {
        if len != 0 {
            self.index = std::cmp::min(len - 1, self.index + 1);
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

    pub fn receive_input(&mut self, input: KeyBinding, paged: &Paged<T>) -> Option<Action> {
        match input {
            KeyBinding::Up => {
                self.select_prev();
                Some(Action::Redraw)
            }
            KeyBinding::Down => {
                self.select_next(paged.len());
                if self.needs_next_page(paged.len()) {
                    // TODO: don't hard code this page...
                    // Make a identifier for pages???
                    // Also TODO: don't keep trying to load more pages when loading one already...
                    paged.load_next(NetworkRequest::LoadNextPlaylistPage);
                }
                Some(Action::Redraw)
            }
            _ => return None,
        }
    }

    fn needs_next_page(&self, len: usize) -> bool {
        (len - self.index) <= PAGE_SIZE as usize / 4
    }
}
