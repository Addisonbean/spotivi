use std::marker::PhantomData;

use crate::api::spotify_api::PAGE_SIZE;

use crate::{
    api::Paged,
    app::Action,
    keybindings::KeyBinding,
};

#[derive(Debug)]
pub struct Cursor<T> {
    index: usize,
    scroll_offset: usize,
    _phantom: PhantomData<T>,
}

impl<T> Cursor<T> {
    pub fn new() -> Cursor<T> {
        Cursor {
            index: 0,
            scroll_offset: 0,
            _phantom: PhantomData
        }
    }

    pub fn offset(&self) -> usize {
        self.scroll_offset
    }

    pub fn get<'a>(&self, index: usize, items: &'a [T]) -> Option<&'a T> {
        items.get(index)
    }

    pub fn selected_item<'a>(&self, items: &'a [T]) -> Option<&'a T> {
        items.get(self.index)
    }

    fn select_next(&mut self, len: usize, height: u16) {
        if len != 0 {
            self.index = std::cmp::min(len - 1, self.index + 1);
            if self.index - self.scroll_offset >= height as usize {
                self.scroll_offset += 1;
            }
        } else {
            // TODO: scroll_offset???
            self.index = 0;
        }
    }

    fn select_prev(&mut self) {
        if self.index != 0 {
            self.index -= 1;
            if self.index < self.scroll_offset {
                self.scroll_offset -= 1;
            }
        }
    }

    pub fn is_highlighted(&self, i: usize) -> bool {
        self.index == i
    }

    pub fn receive_input(&mut self, input: KeyBinding, paged: &Paged<T>, height: u16) -> Option<Action> {
        match input {
            KeyBinding::Up => {
                self.select_prev();
                Some(Action::Redraw)
            }
            KeyBinding::Down => {
                self.select_next(paged.len(), height);
                if self.needs_next_page(paged.len()) {
                    paged.load_next();
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
