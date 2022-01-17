use std::ops::{Add, Sub};

use approx::AbsDiffEq;

use super::Vec3;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Point(f64, f64, f64);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }
}

impl From<(f64, f64, f64)> for Point {
    fn from(t: (f64, f64, f64)) -> Self {
        Point::new(t.0, t.1, t.2)
    }
}

impl Add<Vec3> for Point {
    type Output = Point;

    fn add(self, rhs: Vec3) -> Point {
        Point::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub<Point> for Point {
    type Output = Vec3;

    fn sub(self, rhs: Point) -> Vec3 {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Sub<Vec3> for Point {
    type Output = Point;

    fn sub(self, rhs: Vec3) -> Point {
        Point::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl AbsDiffEq for Point {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        1e-10
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.x().abs_diff_eq(&other.x(), epsilon)
            && self.y().abs_diff_eq(&other.y(), epsilon)
            && self.z().abs_diff_eq(&other.z(), epsilon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_utils::{arbitrary_point, arbitrary_vec3};
    use approx::abs_diff_eq;
    use proptest::prelude::*;

    #[test]
    fn test_point_add_vec3() {
        let p = Point::new(1.0, 2.0, 3.0);
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(p + v, Point::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_point_sub_point() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(1.0, 2.0, 3.0);
        assert_eq!(p1 - p2, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_point_sub_vec3() {
        let p = Point::new(1.0, 2.0, 3.0);
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(p - v, Point::new(0.0, 0.0, 0.0));
    }

    proptest! {
        #[test]
        fn test_point_add_sub_vec3_identity(p in arbitrary_point(), v in arbitrary_vec3()) {
            prop_assert!(abs_diff_eq!(p + v - v, p, epsilon = 1e-6));
        }
    }
}
