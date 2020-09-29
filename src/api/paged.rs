use rspotify::model::page::Page;

use crate::send_request;
use crate::app::NetworkRequest;

#[derive(Clone, Debug)]
pub enum PageId {
    Playlists,
    Playlist(String),
}

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
    items: Vec<T>,
    next_page: Option<NextPage>,
    page_id: PageId,
}

impl<T> Paged<T> {
    pub fn new(page_id: PageId) -> Paged<T> {
        Paged {
            items: Vec::new(),
            next_page: None,
            page_id,
        }
    }

    pub fn add_page(&mut self, page: Paged<T>) {
        self.items.extend(page.items.into_iter());
        let old_index = self.next_page().map(|p| p.index).unwrap_or(0);
        self.next_page = page.next_page.map(|p| p.add_offset(old_index));
    }

    pub fn items(&self) -> &Vec<T> {
        &self.items
    }

    pub fn next_page(&self) -> Option<&NextPage> {
        self.next_page.as_ref()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn load_next(&self) {
        if let Some(_) = self.next_page {
            send_request(NetworkRequest::LoadNextPage(self.page_id.clone()));
        }
    }
}

impl<T> Paged<T> {
    pub fn from<U: Into<T>>(page: Page<U>, page_id: PageId) -> Paged<T> {
        Paged {
            items: page.items.into_iter().map(Into::into).collect(),
            next_page: page.next.map(|uri| NextPage { uri, index: 1 }),
            page_id,
        }
    }
}
