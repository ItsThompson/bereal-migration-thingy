use std::path::{Path, PathBuf};

const PHOTOS_FILE_PATH: &str = "data/profile_activity_data/Photos/";
const MEMORY_FILE_PATH: &str = "data/profile_activity_data/memories.json";

pub fn base_memory_file() -> &'static Path {
    Path::new(MEMORY_FILE_PATH)
}

pub fn base_photos_dir() -> &'static Path {
    Path::new(PHOTOS_FILE_PATH)
}

pub fn posts_dir() -> PathBuf {
    base_photos_dir().join("post")
}
