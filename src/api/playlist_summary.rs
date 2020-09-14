use rspotify::model::playlist::SimplifiedPlaylist;

#[derive(Debug)]
pub struct PlaylistSummary {
    name: String,
    id: String,
}

impl PlaylistSummary {
    pub fn new(name: String, id: String) -> PlaylistSummary {
        PlaylistSummary { name, id }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl From<SimplifiedPlaylist> for PlaylistSummary {
    fn from(p: SimplifiedPlaylist) -> PlaylistSummary {
        PlaylistSummary {
            name: p.name,
            id: p.id,
        }
    }
}
