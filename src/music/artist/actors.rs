use crate::music::traits::{HasLocation, Location};

use super::model::Artist;

impl HasLocation for Artist {
    fn locations(&self) -> Vec<Location> {
        self.urls
            .iter()
            .map(|url| Location::Url {
                url: url.to_owned(),
            })
            .collect()
    }
}
