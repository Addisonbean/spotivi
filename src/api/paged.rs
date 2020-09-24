use rspotify::model::page::Page;

use crate::{
    app::{Action, NetworkRequest},
    keybindings::KeyBinding,
    api::InteractiveList,
    api::spotify_api::PAGE_SIZE,
};

#[derive(Debug)]
pub struct NextPage {
    pub index: u32,
    pub uri: String,
}

impl NextPage {
    fn add_offset(mut self, offset: u32) -> NextPage {
        self.index += offset;
        self
    }
}


#[derive(Debug)]
pub struct Paged<T> {
    items: InteractiveList<T>,
    next_page: Option<NextPage>,
}

impl<T> Paged<T> {
    pub fn new() -> Paged<T> {
        Paged {
            items: InteractiveList::new(),
            next_page: None,
        }
    }

    pub fn add_page(&mut self, page: Paged<T>) {
        self.items.extend(page.items.into_iter());
        let old_index = self.next_page().map(|p| p.index).unwrap_or(0);
        self.next_page = page.next_page.map(|p| p.add_offset(old_index));
    }

    pub fn items(&self) -> &InteractiveList<T> {
        &self.items
    }

    // TODO: THIS METHOD SHOULDN'T EXIST...
    // (have some way to just directly call self.items.handle_input)
    pub fn items_mut(&mut self) -> &mut InteractiveList<T> {
        &mut self.items
    }

    pub fn next_page(&self) -> Option<&NextPage> {
        self.next_page.as_ref()
    }

    pub fn receive_input(&mut self, input: KeyBinding) -> Option<Action> {
        match input {
            KeyBinding::Up => {
                self.items.select_prev();
                Some(Action::Redraw)
            }
            KeyBinding::Down => {
                self.items.select_next();
                match self.next_page {
                    // TODO: don't hard code this page...
                    // Make a identifier for pages???
                    // Also TODO: don't keep trying to load more pages when loading one already...
                    // Another TODO: still redraw, but also load a page...
                    Some(ref p) if self.needs_next_page() => Some(Action::NetworkRequest(NetworkRequest::LoadPlaylistsPage(p.index))),
                    _ => Some(Action::Redraw),
                }
            }
            _ => return None,
        }
    }

    fn needs_next_page(&self) -> bool {
        (self.items.len() - self.items.index()) <= PAGE_SIZE as usize / 4
    }
}

impl<T, U: Into<T>> From<Page<U>> for Paged<T> {
    fn from(page: Page<U>) -> Paged<T> {
        Paged {
            items: InteractiveList::from(page.items.into_iter().map(Into::into).collect()),
            next_page: page.next.map(|uri| NextPage { uri, index: 1 }),
        }
    }
}
