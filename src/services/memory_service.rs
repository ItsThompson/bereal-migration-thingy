use crate::models::image::Image;
use image::{ImageBuffer, Rgba};
use std::error::Error;

// -----------------------------
// Rounded corner mask (FAST)
// -----------------------------
fn build_round_mask(w: u32, h: u32, radius: u32) -> Vec<u8> {
    let mut mask = vec![255u8; (w * h) as usize];
    let r2 = (radius * radius) as i32;

    for y in 0..h as i32 {
        for x in 0..w as i32 {
            let mut dx = 0;
            let mut dy = 0;

            if x < radius as i32 {
                dx = radius as i32 - x - 1;
            } else if x >= (w as i32 - radius as i32) {
                dx = x - (w as i32 - radius as i32);
            }

            if y < radius as i32 {
                dy = radius as i32 - y - 1;
            } else if y >= (h as i32 - radius as i32) {
                dy = y - (h as i32 - radius as i32);
            }

            if dx * dx + dy * dy > r2 {
                mask[(y * w as i32 + x) as usize] = 0;
            }
        }
    }

    mask
}

// Apply mask to image alpha channel (FAST linear pass)
fn apply_mask(img: &mut [u8], mask: &[u8]) {
    for (px, m) in img.chunks_exact_mut(4).zip(mask.iter()) {
        px[3] = ((px[3] as u16 * *m as u16) / 255) as u8;
    }
}

// -----------------------------
// Fast overlay (no rayon, cache-friendly)
// -----------------------------
fn overlay_top_left(
    background: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    foreground: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    margin: u32,
) {
    let (bg_w, bg_h) = background.dimensions();
    let (fg_w, fg_h) = foreground.dimensions();

    let start_x = margin as usize;
    let start_y = margin as usize;

    let bg = background.as_mut();
    let fg = foreground.as_raw();

    let bg_stride = (bg_w * 4) as usize;
    let fg_stride = (fg_w * 4) as usize;

    for y in 0..fg_h as usize {
        let bg_y = y + start_y;
        if bg_y >= bg_h as usize {
            break;
        }

        let bg_row = &mut bg[bg_y * bg_stride..(bg_y + 1) * bg_stride];
        let fg_row = &fg[y * fg_stride..(y + 1) * fg_stride];

        for x in 0..fg_w as usize {
            let bg_x = x + start_x;
            if bg_x >= bg_w as usize {
                continue;
            }

            let bg_i = bg_x * 4;
            let fg_i = x * 4;

            let f = &fg_row[fg_i..fg_i + 4];

            let a = f[3] as u32;
            if a == 0 {
                continue;
            }

            let inv = 255 - a;
            let b = &mut bg_row[bg_i..bg_i + 4];

            b[0] = ((f[0] as u32 * a + b[0] as u32 * inv) / 255) as u8;
            b[1] = ((f[1] as u32 * a + b[1] as u32 * inv) / 255) as u8;
            b[2] = ((f[2] as u32 * a + b[2] as u32 * inv) / 255) as u8;
            b[3] = 255;
        }
    }
}

// -----------------------------
// Main pipeline
// -----------------------------
pub fn generate_memory_image(
    front_image: &Image,
    back_image: &Image,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    println!(
        "Generating memory image with front: {} and back: {}",
        front_image.get_local_path(),
        back_image.get_local_path()
    );

    // Decode
    let mut back = image::open(back_image.get_local_path())?.to_rgba8();
    let front = image::open(front_image.get_local_path())?.to_rgba8();

    // -----------------------------
    // Resize front
    // -----------------------------
    let target_width = back.width() / 4;
    let aspect = front.height() as f32 / front.width() as f32;
    let target_height = (target_width as f32 * aspect) as u32;

    let mut front_resized = image::imageops::resize(
        &front,
        target_width,
        target_height,
        image::imageops::FilterType::Triangle,
    );

    // -----------------------------
    // Rounded corners (FAST MASK)
    // -----------------------------
    let mask = build_round_mask(front_resized.width(), front_resized.height(), 20);

    apply_mask(front_resized.as_mut(), &mask);

    // -----------------------------
    // Overlay
    // -----------------------------
    overlay_top_left(&mut back, &front_resized, 20);

    // Save
    back.save(output_path)?;

    Ok(())
}
