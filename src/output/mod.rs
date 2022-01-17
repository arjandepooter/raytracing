use crate::core::{Canvas, Color};
use anyhow::{Context, Result};
use image::{Rgb, RgbImage};

impl From<Color> for Rgb<u8> {
    fn from(color: Color) -> Self {
        Rgb([
            (color.r().clamp(0.0, 1.0) * 255.0) as u8,
            (color.g().clamp(0.0, 1.0) * 255.0) as u8,
            (color.b().clamp(0.0, 1.0) * 255.0) as u8,
        ])
    }
}

pub fn save_canvas(canvas: &Canvas, filename: &str) -> Result<()> {
    let pixels: Vec<u8> = canvas
        .iter_pixels()
        .flat_map(|&color| Rgb::from(color).0)
        .collect::<Vec<_>>();
    let image = RgbImage::from_vec(canvas.width as u32, canvas.height as u32, pixels)
        .context("Error while reading pixels")?;

    image.save(filename)?;

    Ok(())
}
