use super::Matrix;

trait Transform {
    fn transform(self, transformation_matrix: &Matrix<4, 4>) -> Self;
}

impl<T> Transform for T
where
    T: Into<Matrix<4, 1>>,
    T: From<Matrix<4, 1>>,
{
    fn transform(self, transformation_matrix: &Matrix<4, 4>) -> Self {
        let m: Matrix<4, 1> = self.into();
        (*transformation_matrix * m).into()
    }
}

/// Creates a 4x4 transformation matrix for translating by the given x, y, and z factors.
///
/// # Arguments
///
/// * `x` - The translation factor along the x-axis.
/// * `y` - The translation factor along the y-axis.
/// * `z` - The translation factor along the z-axis.
///
/// # Returns
///
/// A new 4x4 translation matrix
pub fn translate(x: f64, y: f64, z: f64) -> Matrix<4, 4> {
    let mut m = Matrix::<4, 4>::identity();
    m[(0, 3)] = x;
    m[(1, 3)] = y;
    m[(2, 3)] = z;

    m
}

/// Creates a 4x4 transfomration matrix for scaling by the given x, y, and z factors.
///
/// # Arguments
///
/// * `x` - The scaling factor along the x-axis.
/// * `y` - The scaling factor along the y-axis.
/// * `z` - The scaling factor along the z-axis.
///
/// # Returns
///
/// A new 4x4 scaling matrix
pub fn scale(x: f64, y: f64, z: f64) -> Matrix<4, 4> {
    let mut m = Matrix::<4, 4>::identity();
    m[(0, 0)] = x;
    m[(1, 1)] = y;
    m[(2, 2)] = z;

    m
}

/// Creates a 4x4 rotation matrix for rotating around the x-axis by the given angle in radians.
///
/// # Arguments
///
/// * `radians` - The angle to rotate the x-axis by, in radians.
///
/// # Returns
///
/// A 4x4 rotation matrix
pub fn rotate_x(radians: f64) -> Matrix<4, 4> {
    let mut m = Matrix::<4, 4>::identity();
    m[(1, 1)] = radians.cos();
    m[(1, 2)] = -radians.sin();
    m[(2, 1)] = radians.sin();
    m[(2, 2)] = radians.cos();

    m
}

/// Creates a 4x4 rotation matrix for rotating around the y-axis by the given angle in radians.
///
/// # Arguments
///
/// * `radians` - The angle to rotate y-axis by, in radians.
///
/// # Returns
///
/// A 4x4 rotation matrix
pub fn rotate_y(radians: f64) -> Matrix<4, 4> {
    let mut m = Matrix::<4, 4>::identity();
    m[(0, 0)] = radians.cos();
    m[(0, 2)] = radians.sin();
    m[(2, 0)] = -radians.sin();
    m[(2, 2)] = radians.cos();

    m
}

/// Creates a 4x4 rotation matrix for rotating around the z-axis by the given angle in radians.
///
/// # Arguments
///
/// * `radians` - The angle to rotate z-axis by, in radians.
///
/// # Returns
///
/// A 4x4 rotation matrix
pub fn rotate_z(radians: f64) -> Matrix<4, 4> {
    let mut m = Matrix::<4, 4>::identity();
    m[(0, 0)] = radians.cos();
    m[(0, 1)] = -radians.sin();
    m[(1, 0)] = radians.sin();
    m[(1, 1)] = radians.cos();

    m
}

/// Create a 4x4 rotation matrix around the x, y, and z axes by the given angles in radians.
///
/// # Arguments
///
/// * `radians_x` - The angle to rotate x-axis by, in radians.
/// * `radians_y` - The angle to rotate y-axis by, in radians.
/// * `radians_z` - The angle to rotate z-axis by, in radians.
///
/// # Returns
///
/// A 4x4 rotation matrix
pub fn rotate(radians_x: f64, radians_y: f64, radians_z: f64) -> Matrix<4, 4> {
    rotate_x(radians_x) * rotate_y(radians_y) * rotate_z(radians_z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{test_utils::arbitrary_vec3, Point, Vec3};
    use approx::abs_diff_eq;
    use proptest::prelude::*;

    #[test]
    fn test_translate_point() {
        let point = Point::new(-3.0, 4.0, 5.0);
        let transformation = translate(5.0, -3.0, 2.0);

        assert_eq!(point.transform(&transformation), Point::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn test_scale_point() {
        let point = Point::new(-4.0, 6.0, 8.0);
        let transformation = scale(2.0, 3.0, 4.0);

        assert_eq!(
            point.transform(&transformation),
            Point::new(-8.0, 18.0, 32.0)
        );
    }

    #[test]
    fn test_scale_vec3() {
        let vector = Vec3::new(-4.0, 6.0, 8.0);
        let transformation = scale(2.0, 3.0, 4.0);

        assert_eq!(
            vector.transform(&transformation),
            Vec3::new(-8.0, 18.0, 32.0)
        );
    }

    #[test]
    fn test_reflection() {
        let point = Point::new(2.0, 3.0, 4.0);
        let transformation = scale(-1.0, 1.0, 1.0);

        assert_eq!(point.transform(&transformation), Point::new(-2.0, 3.0, 4.0));
    }

    #[test]
    fn test_rotate_x() {
        let point = Point::new(0.0, 1.0, 0.0);
        let half_quarter = rotate_x(std::f64::consts::PI / 4.0);
        let full_quarter = rotate_x(std::f64::consts::PI / 2.0);

        assert!(abs_diff_eq!(
            point.transform(&half_quarter),
            Point::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        ));
        assert!(abs_diff_eq!(
            point.transform(&full_quarter),
            Point::new(0.0, 0.0, 1.0)
        ));
    }

    proptest! {
        #[test]
        fn test_translate_vec3_noop(v in arbitrary_vec3()) {
            let translation = translate(6.0, -1.3, 2.0);
            prop_assert_eq!(v.transform(&translation), v);
        }

        #[test]
        fn test_translate_with_inverse(v in arbitrary_vec3()) {
            let translation = translate(6.0, -1.3, 2.0);
            let inverse = translation.inverse();
            let transformation = translation * inverse;
            prop_assert_eq!(v.transform(&transformation), v);
        }

        #[test]
        fn test_scale_with_inverse(v in arbitrary_vec3()) {
            let scale = scale(6.0, -1.3, 2.0);
            let inverse = scale.inverse();
            let transformation = scale * inverse;
            prop_assert_eq!(v.transform(&transformation), v);
        }

        #[test]
        fn test_rotate_x_full_circle(v in arbitrary_vec3()) {
            let rotation = rotate_x(std::f64::consts::PI * 2.0);
            prop_assert!(abs_diff_eq!(v.transform(&rotation), v));
        }

        #[test]
        fn test_rotate_y_full_circle(v in arbitrary_vec3()) {
            let rotation = rotate_y(std::f64::consts::PI * 2.0);
            prop_assert!(abs_diff_eq!(v.transform(&rotation), v));
        }

        #[test]
        fn test_rotate_z_full_circle(v in arbitrary_vec3()) {
            let rotation = rotate_z(std::f64::consts::PI * 2.0);
            prop_assert!(abs_diff_eq!(v.transform(&rotation), v));
        }
    }
}
