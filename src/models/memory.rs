use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::models::{image::Image, location::Location, music::Music};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Memory {
    pub back_image: Image,
    bereal_moment: String,
    date: String,
    pub front_image: Image,
    is_late: bool,
    location: Option<Location>,
    music: Option<Music>,
    taken_time: String,
}

impl Memory {
    pub fn get_date(&self) -> &str {
        let date_part = self.date.split('T').next().unwrap_or("");
        date_part
    }
}
