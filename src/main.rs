use bereal_local::models::memory::Memory;
use bereal_local::services::memory_service::generate_memory_image;
use bereal_local::storage::paths::base_memory_file;
use serde_json::Result;
use std::fs::File;

fn read_memory_file() -> Result<()> {
    let file = File::open(base_memory_file()).expect("file should open read only");

    let memories: Vec<Memory> = serde_json::from_reader(file).expect("file should be proper JSON");

    let test = &memories[0];

    let _ = generate_memory_image(&test.front_image, &test.back_image);

    Ok(())
}

fn main() {
    let _ = read_memory_file();
}
