use proptest::strategy::Strategy;
use std::ops::Range;

use super::{Matrix, Point, Vec3};

pub fn arbitrary_vec3() -> impl Strategy<Value = Vec3> {
    const RANGE: Range<f64> = -100_000f64..100_000f64;
    [RANGE; 3].prop_map(|[x, y, z]| Vec3::new(x, y, z))
}

pub fn arbitrary_point() -> impl Strategy<Value = Point> {
    const RANGE: Range<f64> = -100_000f64..100_000f64;
    [RANGE; 3].prop_map(|[x, y, z]| Point::new(x, y, z))
}

pub fn arbitrary_matrix3() -> impl Strategy<Value = Matrix<3, 3>> {
    const RANGE: Range<f64> = -1e3f64..1e3f64;
    [[RANGE; 3], [RANGE; 3], [RANGE; 3]].prop_map(|rows| Matrix::new(rows))
}
