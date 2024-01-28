use std::ops::{Index, IndexMut};

use crate::core::Color;

/// A canvas of pixels.
///
/// # Examples
///
/// ```
/// use raytracing::core::{Canvas, Color};
///
/// let mut canvas = Canvas::new(10, 20);
///
/// assert_eq!(canvas.width, 10);
/// assert_eq!(canvas.height, 20);
/// assert_eq!(canvas[(2, 3)], Color::default());
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    /// Creates a new canvas of the given width and height.
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut pixels = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            pixels.push(Color::default());
        }
        Canvas {
            width,
            height,
            pixels,
        }
    }

    /// Returns the pixel at the given coordinates.
    pub fn pixel_at(&self, x: usize, y: usize) -> &Color {
        &self.pixels[y * self.width + x]
    }

    /// Returns a mutable reference to the pixel at the given coordinates.
    pub fn pixel_at_mut(&mut self, x: usize, y: usize) -> &mut Color {
        &mut self.pixels[y * self.width + x]
    }

    /// Returns an iterator over the pixels of the canvas.
    pub fn iter_pixels(&self) -> impl Iterator<Item = &Color> {
        self.pixels.iter()
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = Color;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.pixel_at(index.0, index.1)
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.pixel_at_mut(index.0, index.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_index() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas[(2, 3)], Color::default());
    }

    #[test]
    fn test_canvas_index_mut() {
        let mut canvas = Canvas::new(10, 20);
        canvas[(2, 3)] = Color::new(0.5, 0.5, 0.5);
        assert_eq!(canvas[(2, 3)], Color::new(0.5, 0.5, 0.5));
    }
}
