use crate::storage::paths::posts_dir;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    bucket: String,
    height: i64,
    media_type: String,
    mime_type: String,
    path: String,
    width: i64,
}

impl Media {
    pub fn get_local_path(&self) -> String {
        let file_name = Path::new(&self.path)
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("");

        posts_dir().join(file_name).to_string_lossy().into_owned()
    }
}
