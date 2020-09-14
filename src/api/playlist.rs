use crate::views::InteractiveList;

#[derive(Debug)]
pub struct Playlist {
    name: String,
    items: InteractiveList<String>,
}

impl Playlist {
    pub fn new(name: String, items: Vec<String>) -> Playlist {
        Playlist {
            name,
            items: InteractiveList::from(items),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn items(&self) -> &InteractiveList<String> {
       &self.items
    }

    pub fn items_mut(&mut self) -> &mut InteractiveList<String> {
       &mut self.items
    }

    pub fn load_tracks(&mut self, items: impl Iterator<Item=String>) {
        self.items.extend(items);
    }
}
