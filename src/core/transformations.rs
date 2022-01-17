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

pub fn translate(x: f64, y: f64, z: f64) -> Matrix<4, 4> {
    let mut m = Matrix::<4, 4>::identity();
    m[(0, 3)] = x;
    m[(1, 3)] = y;
    m[(2, 3)] = z;

    m
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{test_utils::arbitrary_vec3, Point};
    use proptest::prelude::*;

    #[test]
    fn test_translate_point() {
        let point = Point::default();
        let transformation = translate(1.0, 2.0, 3.0);

        assert_eq!(point.transform(&transformation), Point::new(1.0, 2.0, 3.0));
    }

    proptest! {
        #[test]
        fn test_translate_vec3_noop(v in arbitrary_vec3()) {
            let translation = translate(6.0, -1.3, 2.0);
            prop_assert_eq!(v.transform(&translation), v);
        }
    }
}
