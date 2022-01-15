use proptest::strategy::Strategy;

use super::{Point, Vec3};

const RANGE: std::ops::Range<f64> = -100_000f64..100_000f64;

pub fn arbitrary_vec3() -> impl Strategy<Value = Vec3> {
    (RANGE, RANGE, RANGE).prop_map(|(x, y, z)| Vec3::new(x, y, z))
}

pub fn arbitrary_point() -> impl Strategy<Value = Point> {
    (RANGE, RANGE, RANGE).prop_map(|(x, y, z)| Point::new(x, y, z))
}
