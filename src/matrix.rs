//! Helpers for mathematical matrices.

use std::{
    fmt::{self, Debug},
    ops::{Deref, DerefMut},
};

use itertools::Itertools;
use num::{NumCast, ToPrimitive};

/// A mathemathical matrix.
#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Matrix<T, const R: usize, const C: usize>([[T; C]; R]);

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    /// Create a matrix from a slice of slices.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc::matrix::Matrix;
    /// let matrix = Matrix::new([
    ///     [1, 0, 0],
    ///     [0, 1, 0],
    ///     [0, 0, 1],
    /// ]);
    /// ```
    pub fn new(data: [[T; C]; R]) -> Self {
        Self(data)
    }
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: ToPrimitive,
{
    /// Try to convert all items in the matrix to another type.
    ///
    /// # Examples.
    ///
    /// ```
    /// # use aoc::matrix::Matrix;
    /// let matrix = Matrix::new([
    ///     [1, 0, 0],
    ///     [0, 1, 0],
    ///     [0, 0, 1000],
    /// ]);
    /// assert_eq!(matrix.cast::<f64>(), Some(Matrix::new([
    ///     [1.0, 0.0, 0.0],
    ///     [0.0, 1.0, 0.0],
    ///     [0.0, 0.0, 1000.0],
    /// ])));
    /// assert_eq!(matrix.cast::<u8>(), None);
    /// ```
    pub fn cast<TT>(self) -> Option<Matrix<TT, R, C>>
    where
        TT: NumCast + Debug,
    {
        let mut new_rows = Vec::new();
        for row in self.0 {
            let mut new_row = Vec::new();
            for cell in row {
                new_row.push(<TT as NumCast>::from(cell)?);
            }
            new_rows.push(new_row.try_into().unwrap());
        }
        Some(Matrix::new(new_rows.try_into().unwrap()))
    }
}

impl<T, const R: usize, const C: usize> Deref for Matrix<T, R, C> {
    type Target = [[T; C]; R];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T, const R: usize, const C: usize> DerefMut for Matrix<T, R, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Debug, const R: usize, const C: usize> Debug for Matrix<T, R, C>
where
    [[T; C]; R]: Debug,
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !f.alternate() {
            return (self.0).fmt(f);
        }

        for (y, row) in self.iter().enumerate() {
            if y == 0 {
                write!(f, "⎡ ")?;
            } else if y == R - 1 {
                write!(f, "⎣ ")?;
            } else {
                write!(f, "⎢ ")?;
            }

            for item in row {
                item.fmt(f)?;
                write!(f, " ")?;
            }

            if y == 0 {
                writeln!(f, "⎤")?;
            } else if y == R - 1 {
                writeln!(f, "⎦")?;
            } else {
                writeln!(f, "⎥")?;
            }
        }
        Ok(())
    }
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: ToPrimitive + Copy,
{
    /// Perform Gauss-Jordan elimination, and return the last column.
    ///
    /// This is a method to solve a system of linear equations. Generally speaking you will need as
    /// many equations as you have unknowns, though there might be cases where you need more (e.g
    /// if two equations are equivalent (can be transformed to one another), such as `x + y = 1`
    /// and `2x + 2y = 2`).
    ///
    /// Beware of floating point math, if you're going to cast the result to an integer you will
    /// want to round it first.
    ///
    /// See <https://en.wikipedia.org/wiki/Gaussian_elimination> for more information.
    ///
    /// # Examples
    ///
    /// Given the following set of equations:
    ///
    /// ```math
    /// 2x + y - z = 8
    /// -3x - y + 2z = -11
    /// -2x + y + 2z = -3
    /// ```
    ///
    /// We can express this as a matrix and solve it:
    ///
    /// ```
    /// # use aoc::matrix::Matrix;
    /// let matrix = Matrix::new([
    ///     [2, 1, -1, 8],
    ///     [-3, -1, 2, -11],
    ///     [-2, 1, 2, -3],
    /// ]);
    /// let result = matrix.gauss_jordan_elimination();
    /// assert_eq!(result[0].round(), 2.0);
    /// assert_eq!(result[1].round(), 3.0);
    /// assert_eq!(result[2].round(), -1.0);
    /// ```
    #[must_use]
    pub fn gauss_jordan_elimination(&self) -> [f64; R] {
        let mut matrix = self.cast::<f64>().unwrap();

        let mut pivot_row = 0;
        let mut pivot_col = 0;

        while pivot_row < R && pivot_col < C {
            let (max_idx, max) = matrix
                .iter()
                .enumerate()
                .skip(pivot_row)
                .map(|(idx, r)| (idx, (r[pivot_col].abs() * 1000.0) as usize))
                .max_by_key(|(_, v)| *v)
                .unwrap();
            if max == 0 {
                // No pivot in this column, pass to next column.
                pivot_col += 1;
                continue;
            }

            if max_idx != pivot_row {
                matrix.swap(max_idx, pivot_row);
            }

            for r in (pivot_row + 1)..R {
                let f = matrix[r][pivot_col] / matrix[pivot_row][pivot_col];
                matrix[r][pivot_col] = 0.0;
                for c in (pivot_col + 1)..C {
                    matrix[r][c] -= matrix[pivot_row][c] * f;
                }
            }
            pivot_row += 1;
            pivot_col += 1;
        }

        for pivot_row in (0..R).rev() {
            let Some((pivot_col, _)) = matrix[pivot_row]
                .iter()
                .enumerate()
                .find(|(_, v)| v.abs() > f64::EPSILON)
            else {
                continue;
            };

            let f = matrix[pivot_row][pivot_col].recip();
            for c in pivot_col..C {
                matrix[pivot_row][c] *= f;
            }

            for r in 0..pivot_row {
                let f = matrix[r][pivot_col];
                for c in pivot_col..C {
                    matrix[r][c] -= matrix[pivot_row][c] * f;
                }
            }
        }

        matrix
            .into_iter()
            .map(|row| row[C - 1])
            .collect_vec()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use core::f64;

    macro_rules! assert_eq_approx {
        ($actual:expr, $expected:expr $(,)?) => {{
            let actual = $actual;
            let expected = $expected;
            assert!(
                (actual - expected).abs() < 0.000_005,
                "expected {:?} to approximately equal {:?}",
                actual,
                expected,
            );
        }};
    }

    #[test]
    fn gauss_jordan_elimination() {
        let matrix = super::Matrix([
            [2.0, 1.0, -1.0, 8.0],
            [-3.0, -1.0, 2.0, -11.0],
            [-2.0, 1.0, 2.0, -3.0],
        ]);
        let result = matrix.gauss_jordan_elimination();
        assert_eq_approx!(result[0], 2.0);
        assert_eq_approx!(result[1], 3.0);
        assert_eq_approx!(result[2], -1.0);
    }

    #[test]
    fn format() {
        let matrix = super::Matrix([
            [1.0, 2.0, 3.0],
            [-1.0, f64::consts::PI, 1.0 / 7.0],
            [1_000.0, 0.0, 2.5],
        ]);
        assert_eq!(
            format!("{matrix:.1?}"),
            "[[1.0, 2.0, 3.0], [-1.0, 3.1, 0.1], [1000.0, 0.0, 2.5]]"
        );
        assert_eq!(
            format!("{matrix:#.2?}"),
            "⎡ 1.00 2.00 3.00 ⎤\n⎢ -1.00 3.14 0.14 ⎥\n⎣ 1000.00 0.00 2.50 ⎦\n",
        );
        assert_eq!(
            format!("{matrix:#6.1?}"),
            "⎡    1.0    2.0    3.0 ⎤\n⎢   -1.0    3.1    0.1 ⎥\n⎣ 1000.0    0.0    2.5 ⎦\n",
        );
        assert_eq!(
            format!("{matrix:<#6.1?}"),
            "⎡ 1.0    2.0    3.0    ⎤\n⎢ -1.0   3.1    0.1    ⎥\n⎣ 1000.0 0.0    2.5    ⎦\n",
        );
    }
}
