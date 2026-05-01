use crate::models::image::Media;
use image::{ImageBuffer, Rgba};
use std::error::Error;
use std::process::Command;

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

pub fn generate_memory_image(
    front_image: &Media,
    back_image: &Media,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    println!(
        "Generating memory image with front: {} and back: {}",
        front_image.get_local_path(),
        back_image.get_local_path()
    );

    let mut back = image::open(back_image.get_local_path())?.to_rgba8();
    let front = image::open(front_image.get_local_path())?.to_rgba8();

    let target_width = back.width() / 4;
    let aspect = front.height() as f32 / front.width() as f32;
    let target_height = (target_width as f32 * aspect) as u32;

    let mut front_resized = image::imageops::resize(
        &front,
        target_width,
        target_height,
        image::imageops::FilterType::Triangle,
    );

    let mask = build_round_mask(front_resized.width(), front_resized.height(), 20);

    apply_mask(front_resized.as_mut(), &mask);

    overlay_top_left(&mut back, &front_resized, 20);

    back.save(output_path)?;

    Ok(())
}

pub fn generate_memory_video(
    front_image: &Media,
    back_image: &Media,
    bts_media: &Media,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let frame_png = format!("{}.frame.png", output_path);

    generate_memory_image(front_image, back_image, &frame_png)?;
    append_frame_to_video(bts_media.get_local_path().as_str(), &frame_png, output_path)?;

    let _ = std::fs::remove_file(&frame_png);

    Ok(())
}

pub fn append_frame_to_video(
    input_video: &str,
    frame_png: &str,
    output_video: &str,
) -> Result<(), Box<dyn Error>> {
    let img = image::open(frame_png)?;
    let w = img.width();
    let h = img.height();

    let filter = format!(
        "[0:v]fps=30,scale={w}:{h}:force_original_aspect_ratio=decrease,\
        pad={w}:{h}:(ow-iw)/2:(oh-ih)/2,format=yuv420p[img];\
        [1:v]fps=30,scale={w}:{h}:force_original_aspect_ratio=decrease,\
        pad={w}:{h}:(ow-iw)/2:(oh-ih)/2,format=yuv420p[vid];\
        [vid][img]concat=n=2:v=1:a=0[out]"
    );

    let status = Command::new("ffmpeg")
        .args([
            "-y",
            "-loop", "1", "-t", "1", "-i", frame_png,
            "-i", input_video,
            "-filter_complex", &filter,
            "-map", "[out]",
            "-c:v", "libx264",
            "-pix_fmt", "yuv420p",
            output_video,
        ])
        .status()?;

    if !status.success() {
        return Err(format!("ffmpeg exited with status: {}", status).into());
    }

    Ok(())
}
