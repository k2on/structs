use serde::{Deserialize, Serialize};

use super::r#const::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Location {
    Url { url: String },
    UrlAt { url: String, at: u32 },
}

impl Location {
    pub fn url(&self) -> String {
        match self {
            Location::Url { url } => url,
            Location::UrlAt { url, at: _ } => url,
        }
        .to_owned()
    }
}

pub trait HasLocation {
    fn locations(&self) -> Vec<Location>;

    fn url(&self, url_key: &str) -> Option<Location> {
        self.locations()
            .iter()
            .find(|location| location.url().contains(url_key))
            .cloned()
    }

    fn url_bandcamp(&self) -> Option<Location> {
        self.url(URL_BANDCAMP)
    }
    fn url_soundcloud(&self) -> Option<Location> {
        self.url(URL_SOUNDCLOUD)
    }
    fn url_youtube(&self) -> Option<Location> {
        self.url(URL_YOUTUBE)
    }
    fn url_spotify(&self) -> Option<Location> {
        self.url(URL_SPOTIFY)
    }
    fn url_apple(&self) -> Option<Location> {
        self.url(URL_APPLE)
    }
}
