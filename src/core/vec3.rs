use std::ops::{Add, Div, Mul, Neg, Sub};

use approx::AbsDiffEq;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3(x, y, z)
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

    pub fn magnitude(&self) -> f64 {
        self.x().abs() + self.y().abs() + self.z().abs()
    }

    pub fn normalize(&self) -> Vec3 {
        let mag = self.magnitude();

        if mag == 0.0 {
            Vec3::default()
        } else {
            Vec3::new(self.x() / mag, self.y() / mag, self.z() / mag)
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        self.cross(&rhs)
    }
}

impl AbsDiffEq for Vec3 {
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
    use crate::core::test_utils::arbitrary_vec3;
    use approx::abs_diff_eq;
    use proptest::prelude::*;

    #[test]
    fn test_vec3_add_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1 + v2, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_vec3_sub_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1 - v2, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_vec3_neg() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(-v, Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_vec3_mul_f64() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v * 2.0, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_f64_mul_vec3() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(2.0 * v, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_vec3_div_f64() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v / 2.0, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_vec3_magnitude() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).magnitude(), 6.0);
        assert_eq!(Vec3::new(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(Vec3::new(-1.0, -5.0, 8.0).magnitude(), 14.0);
    }

    #[test]
    fn test_vec3_normalize() {
        let v = Vec3::new(20.0, 60.0, 120.0);
        assert_eq!(v.normalize(), Vec3::new(0.1, 0.3, 0.6));
    }

    #[test]
    fn test_vec3_normalize_zero() {
        let v = Vec3::default();
        assert_eq!(v.normalize(), Vec3::default());
    }

    #[test]
    fn test_vec3_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.dot(&v2), 14.0);
    }

    #[test]
    fn test_vec3_cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3 = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(v1 * v2, Vec3::new(-1.0, 2.0, -1.0));
        assert_eq!(v2 * v1, Vec3::new(1.0, -2.0, 1.0));
    }

    proptest! {
        #[test]
        fn test_vec3_mul_f64_commutative(v in arbitrary_vec3(), i in -1000.0..1000.0) {
            prop_assert_eq!(v * i, i * v);
        }

        #[test]
        fn test_vec3_add_commutative(v1 in arbitrary_vec3(), v2 in arbitrary_vec3()) {
            prop_assert_eq!(v1 + v2, v2 + v1);
        }

        #[test]
        fn test_vec3_sub_add_identity(v in arbitrary_vec3()) {
            prop_assert_eq!(v - v + v, v);
            prop_assert_eq!(v + v - v, v);
        }

        #[test]
        fn test_vec3_sub_sub_identity(v in arbitrary_vec3()) {
            prop_assert_eq!(v - v - v, -v);
        }

        #[test]
        fn test_vec3_mul_div_identity(v in arbitrary_vec3(), i in 1.0f64..110.0) {
            prop_assert!(abs_diff_eq!((v * i) / i, v));
        }

        #[test]
        fn test_vec3_sub_self_zero(v in arbitrary_vec3()) {
            prop_assert_eq!(v - v, Vec3::default());
        }

        #[test]
        fn test_vec3_negate_twice_identity(v in arbitrary_vec3()) {
            prop_assert_eq!(-(-v), v);
        }

        #[test]
        fn test_normalize_magnitude(v in arbitrary_vec3()) {
            prop_assert!(abs_diff_eq!(v.normalize().magnitude(), 1.0, epsilon = 1e-6));
        }
    }
}
