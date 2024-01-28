use std::ops::{Add, Mul, Sub};

use approx::AbsDiffEq;

/// A color with RGB values.
///
/// # Notes
///
/// Values out of the range `[0, 1]` are valid but should be clamped when
/// converting to a pixel value.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Color(f64, f64, f64);

impl Color {
    /// Creates a new color.
    ///
    /// # Examples
    ///
    /// ```
    /// use raytracing::core::Color;
    ///
    /// let c = Color::new(1.0, 2.0, 3.0);
    /// ```
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(r, g, b)
    }

    pub fn r(&self) -> f64 {
        self.0
    }

    pub fn g(&self) -> f64 {
        self.1
    }

    pub fn b(&self) -> f64 {
        self.2
    }

    /// Returns the color with RGB values normalized.
    ///
    /// # Examples
    ///
    /// ```
    /// use raytracing::core::Color;
    ///
    /// let c = Color::new(1.5, 0.5, -20.0);
    ///
    /// assert_eq!(c.clamp(), Color::new(1.0, 0.5, 0.0));
    /// ````
    pub fn clamp(&self) -> Color {
        Color::new(
            self.r().clamp(0.0, 1.0),
            self.g().clamp(0.0, 1.0),
            self.b().clamp(0.0, 1.0),
        )
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from(t: (f64, f64, f64)) -> Self {
        Color::new(t.0, t.1, t.2)
    }
}

impl Into<(f64, f64, f64)> for Color {
    fn into(self) -> (f64, f64, f64) {
        (self.r(), self.g(), self.b())
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color::new(self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b())
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color::new(self.r() - rhs.r(), self.g() - rhs.g(), self.b() - rhs.b())
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color::new(self.r() * rhs, self.g() * rhs, self.b() * rhs)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        return rhs * self;
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        return Color::new(self.r() * rhs.r(), self.g() * rhs.g(), self.b() * rhs.b());
    }
}

impl AbsDiffEq for Color {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        1e-10
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.r().abs_diff_eq(&other.r(), epsilon)
            && self.g().abs_diff_eq(&other.g(), epsilon)
            && self.b().abs_diff_eq(&other.b(), epsilon)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn test_color_add() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_color_sub() {
        let c1 = Color::new(0.8, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_abs_diff_eq!(c1 - c2, Color::new(0.1, 0.5, 0.5));
    }

    #[test]
    fn test_color_mul_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_abs_diff_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
        assert_abs_diff_eq!(4.5 * c, c * 4.5);
    }

    #[test]
    fn test_color_mul() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_abs_diff_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}
