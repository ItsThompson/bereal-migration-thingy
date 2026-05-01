use bereal_local::models::memory::Memory;
use bereal_local::services::memory_service::generate_memory_image;
use bereal_local::storage::paths::base_memory_file;
use serde_json::Result;
use std::fs::File;

fn read_memory_file() -> Result<()> {
    let file = File::open(base_memory_file()).expect("file should open read only");

    let memories: Vec<Memory> = serde_json::from_reader(file).expect("file should be proper JSON");

    for memory in &memories {
        println!(
            "Front Image Path: {}, Back Image Path: {}",
            memory.front_image.get_local_path(),
            memory.back_image.get_local_path()
        );
        generate_memory_image(
            &memory.front_image,
            &memory.back_image,
            format!("memory_{}.png", memory.get_date()).as_str(),
        )
        .expect("Failed to generate memory image");
        break;
    }

    Ok(())
}

fn main() {
    let _ = read_memory_file();
}
