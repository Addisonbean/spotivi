use rspotify::model::playlist::SimplifiedPlaylist;

#[derive(Debug)]
pub struct PlaylistSummary {
    name: String,
    id: String,
    collaborative: bool,
    owner_name: Option<String>,
    public: Option<bool>,
}

impl PlaylistSummary {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn collaborative(&self) -> bool {
        self.collaborative
    }

    pub fn owner_name(&self) -> Option<&str> {
        self.owner_name.as_ref().map(|s| &s[..])
    }

    pub fn is_public(&self) -> Option<bool> {
        self.public
    }
}

impl From<SimplifiedPlaylist> for PlaylistSummary {
    fn from(p: SimplifiedPlaylist) -> PlaylistSummary {
        PlaylistSummary {
            name: p.name,
            id: p.id,
            collaborative: p.collaborative,
            owner_name: p.owner.display_name,
            public: p.public,
        }
    }
}
