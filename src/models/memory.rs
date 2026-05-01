use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::models::{image::Media, location::Location, music::Music};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Memory {
    pub front_image: Media,
    pub back_image: Media,
    date: String,
    bereal_moment: String,
    is_late: bool,
    location: Option<Location>,
    music: Option<Music>,
    taken_time: String,
    bts_media: Option<Media>,
}

impl Memory {
    pub fn get_date(&self) -> &str {
        let date_part = self.date.split('T').next().unwrap_or("");
        date_part
    }
}
