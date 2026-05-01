use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Music {
    artist: String,
    artwork: String,
    audio_type: String,
    isrc: String,
    open_url: String,
    provider: String,
    provider_id: String,
    track: String,
    visibility: String,
}
