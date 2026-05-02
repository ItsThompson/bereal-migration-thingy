use bereal_local::models::memory::Memory;
use bereal_local::services::memory_service::{generate_memory_image, generate_memory_video};
use bereal_local::storage::paths::base_memory_file;
use serde_json::Result;
use std::fs;
use std::fs::File;

fn main() -> Result<()> {
    let file = File::open(base_memory_file()).expect("file should open read only");

    let memories: Vec<Memory> = serde_json::from_reader(file).expect("file should be proper JSON");

    fs::create_dir_all("output/").expect("Failed to create output directory");

    for (i, memory) in memories.iter().enumerate() {
        println!(
            "Memory taken on: {} ({}/{})",
            memory.get_date(),
            i + 1,
            memories.len()
        );
        if let Some(bts) = memory.bts_media() {
            generate_memory_video(
                &memory.front_image,
                &memory.back_image,
                bts,
                format!("output/memory_{}.mp4", memory.get_date()).as_str(),
                memory.taken_time(),
            )
            .expect("Failed to generate memory video");
        } else {
            generate_memory_image(
                &memory.front_image,
                &memory.back_image,
                format!("output/memory_{}.png", memory.get_date()).as_str(),
                memory.taken_time(),
            )
            .expect("Failed to generate memory image");
        }
    }

    Ok(())
}
