use std::ops::{Index, IndexMut};

use crate::core::Color;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
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

    pub fn pixel_at(&self, x: usize, y: usize) -> &Color {
        &self.pixels[y * self.width + x]
    }

    pub fn pixel_at_mut(&mut self, x: usize, y: usize) -> &mut Color {
        &mut self.pixels[y * self.width + x]
    }

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
