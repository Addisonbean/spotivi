use rspotify::model::page::Page;

use crate::views::InteractiveList;

mod playlist_summary;
mod requests;
pub use playlist_summary::PlaylistSummary;
pub use requests::ApiRequest;

#[derive(Debug)]
pub struct Paged<T> {
    items: InteractiveList<T>,
    next_page: Option<String>,
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
        self.next_page = page.next_page;
    }

    pub fn items(&self) -> &InteractiveList<T> {
        &self.items
    }

    // TODO: THIS METHOD SHOULDN'T EXIST...
    pub fn items_mut(&mut self) -> &mut InteractiveList<T> {
        &mut self.items
    }

    // pub fn add_page<U: Into<T>>(&mut self, page: Page<U>) {
    //     self.items.extend(page.items.into_iter().map(Into::into));
    //     self.next_page = page.next;
    // }
}

impl<T, U: Into<T>> From<Page<U>> for Paged<T> {
    fn from(page: Page<U>) -> Paged<T> {
        Paged {
            items: InteractiveList::from(page.items.into_iter().map(Into::into).collect()),
            next_page: page.next,
        }
    }
}
