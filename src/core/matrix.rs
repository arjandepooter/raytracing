use approx::AbsDiff;
use std::{
    convert::TryInto,
    iter::{once, repeat},
    ops::Mul,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix<const R: usize, const C: usize> {
    pub rows: [[f64; C]; R],
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn new(rows: [[f64; C]; R]) -> Self {
        Matrix { rows }
    }

    pub fn identity<const T: usize>() -> Matrix<T, T> {
        let rows = (0..T)
            .map(|r| {
                repeat(0.0)
                    .take(r)
                    .chain(once(1.0))
                    .chain(repeat(0.0).take(T - 1 - r))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Matrix { rows }
    }

    pub fn transpose(&self) -> Matrix<C, R> {
        let rows = self.cols().collect::<Vec<_>>().try_into().unwrap();

        Matrix { rows }
    }

    pub fn rows(&self) -> impl Iterator<Item = [f64; C]> + '_ {
        self.rows.iter().map(|row| *row)
    }

    pub fn cols(&self) -> impl Iterator<Item = [f64; R]> + '_ {
        (0..C).map(move |c| {
            self.rows()
                .map(|row| row[c])
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
    }

    pub fn elements(&self) -> impl Iterator<Item = f64> + '_ {
        self.rows()
            .flat_map(|row| row.clone().iter().map(|el| *el).collect::<Vec<_>>())
    }
}

impl<const T: usize> Matrix<T, T> {
    pub fn inverse(&self) -> Self {
        let mut inv_rows = self.rows.clone();

        for p in 0..T {
            let pivot = inv_rows[p][p];

            for j in 0..T {
                if j != p {
                    inv_rows[j][p] = -inv_rows[j][p] / pivot;
                }
            }

            for i in 0..T {
                for j in 0..T {
                    if i != p && j != p {
                        inv_rows[i][j] += inv_rows[p][j] * inv_rows[i][p];
                    }
                }
            }

            for j in 0..T {
                if j != p {
                    inv_rows[p][j] = inv_rows[p][j] / pivot;
                }
            }

            inv_rows[p][p] = 1.0 / pivot;
        }

        Matrix::new(inv_rows)
    }
}

impl<const R: usize, const N: usize, const C: usize> Mul<Matrix<N, C>> for Matrix<R, N> {
    type Output = Matrix<R, C>;

    fn mul(self, rhs: Matrix<N, C>) -> Self::Output {
        let rows: Vec<_> = self.rows().collect();
        let cols: Vec<_> = rhs.cols().collect();

        let elements = (0..R)
            .map(move |r| {
                (0..C)
                    .map(|c| {
                        let row = &rows[r];
                        let col = &cols[c];

                        (0..N).map(|n| row[n] * col[n]).sum()
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self::Output::new(elements)
    }
}

impl<const R: usize, const C: usize> From<f64> for Matrix<R, C> {
    fn from(value: f64) -> Self {
        Matrix::new([[value; C]; R])
    }
}

impl<const R: usize, const C: usize> approx::AbsDiffEq for Matrix<R, C> {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        1e-4
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.elements()
            .zip(other.elements())
            .all(|(a, b)| AbsDiff::default().epsilon(epsilon).eq(&a, &b))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::test_utils::arbitrary_matrix3;

    use super::*;
    use approx::{abs_diff_eq, assert_abs_diff_eq};
    use proptest::prelude::*;

    #[test]
    fn test_matrix_rows() {
        let m = Matrix::new([[1.0, 2.0], [4.0, 3.0]]);
        let v: Vec<_> = m.rows().collect();

        assert_eq!(v, vec![[1.0, 2.0], [4.0, 3.0]]);
    }

    #[test]
    fn test_matrix_cols() {
        let m = Matrix::new([[1.0, 2.0], [4.0, 3.0]]);
        let v: Vec<_> = m.cols().collect();

        assert_eq!(v, vec![[1.0, 4.0], [2.0, 3.0]]);
    }

    #[test]
    fn test_matrix_from_float() {
        let m: Matrix<3, 3> = 4.0.into();

        assert_eq!(m, Matrix::new([[4.0; 3]; 3]))
    }

    #[test]
    fn test_matrix_mul() {
        let m = Matrix::new([[1.0, 2.0], [4.0, 3.0]]);
        let n = Matrix::new([[1.0, 2.0, 3.0], [3.0, -4.0, 7.0]]);

        let expected = Matrix::new([[7.0, -6.0, 17.0], [13.0, -4.0, 33.0]]);

        assert_eq!(m * n, expected);
    }

    #[test]
    fn test_matrix_transpose() {
        let m = Matrix::new([[1.0, 2.0, 3.0], [3.0, -4.0, 7.0]]);

        let expected = Matrix::new([[1.0, 3.0], [2.0, -4.0], [3.0, 7.0]]);

        assert_eq!(m.transpose(), expected);
    }

    #[test]
    fn test_matrix_identity() {
        let m = Matrix::<4, 4>::identity();

        assert_eq!(
            m,
            Matrix::new([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ])
        );
    }

    #[test]
    fn test_matrix_inverse() {
        let m = Matrix::new([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let expected = Matrix::new([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        assert_abs_diff_eq!(m.inverse(), expected);
    }

    proptest! {
        #[test]
        fn test_matrix_mul_identity(m in arbitrary_matrix3()) {
            prop_assert!(abs_diff_eq!(m * Matrix::<3, 3>::identity(), m));
        }

        #[test]
        fn test_matrix_mul_associative(
            m1 in arbitrary_matrix3(),
            m2 in arbitrary_matrix3(),
            m3 in arbitrary_matrix3()
        ) {
            prop_assert!(abs_diff_eq!(m1 * (m2 * m3), (m1 * m2) * m3));
        }
    }
}
