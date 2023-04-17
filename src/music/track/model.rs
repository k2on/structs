use chrono::Duration;
use serde::{Deserialize, Serialize};

use crate::{
    music::{
        sample::model::Sample,
        traits::{HasLocation, Location},
    },
    util::serde_duration,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub position: u8,
    pub artist_id: String,
    pub album_id: String,
    #[serde(with = "serde_duration")]
    pub duration: Duration,
    pub artists: Vec<TrackArtist>,
    pub locations: Vec<Location>,
    pub samples: Vec<Sample>,
    #[serde(skip)]
    pub wave: Option<Wave>,
}

impl HasLocation for Track {
    fn locations(&self) -> Vec<Location> {
        self.locations.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackArtist {
    pub id: String,
    pub r#for: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Wave {
    pub length: i32,
    pub points: Vec<u8>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Location {
//     pub url: String,
//     #[serde(with = "serde_duration_opt", skip_serializing_if = "Option::is_none")]
//     pub at: Option<Duration>,
// }
