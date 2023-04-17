use serde::{Deserialize, Serialize};

use crate::music::track::model::Track;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    media: SampleMedia,
    r#type: SampleType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SampleType {
    Vocals,
    Beat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum SampleMedia {
    Track(Track),
    // Movie(Movie)
}
