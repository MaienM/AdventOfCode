//! Helpers for mathematical matrices.

use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

/// A mathemathical matrix.
pub struct Matrix<T, const R: usize, const C: usize>([[T; C]; R]);

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    /// Create a matrix from a slice of slices.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc::utils::matrix::Matrix;
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

// TODO: change to impl Display instead.
impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: Display,
{
    /// Print the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc::utils::matrix::Matrix;
    /// let matrix = Matrix::new([
    ///     [1, 0, 0],
    ///     [0, 1, 0],
    ///     [0, 0, 1],
    /// ]);
    /// matrix.print();
    /// // Output:
    /// // ⎡      1       0       0  ⎤
    /// // ⎢      0       1       0  ⎥
    /// // ⎣      0       0       1  ⎦
    /// ```
    pub fn print(&self) {
        for (idx, row) in self.iter().enumerate() {
            if idx == 0 {
                print!("⎡ ");
            } else if idx == R - 1 {
                print!("⎣ ");
            } else {
                print!("⎢ ");
            }

            // TODO: adjust width based on the maximum needed width/parameters specified to this object?
            for col in row {
                print!("{col:>6.2}  ");
            }

            if idx == 0 {
                println!("⎤");
            } else if idx == R - 1 {
                println!("⎦");
            } else {
                println!("⎥");
            }
        }
    }
}

// TODO: Implement on num instead, create a copy using f64 internally and return that.
impl<const R: usize, const C: usize> Matrix<f64, R, C> {
    /// Perform Gauss-Jordan elimination.
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
    /// # use aoc::utils::matrix::Matrix;
    /// let mut matrix = Matrix::new([
    ///     [2.0, 1.0, -1.0, 8.0],
    ///     [-3.0, -1.0, 2.0, -11.0],
    ///     [-2.0, 1.0, 2.0, -3.0],
    /// ]);
    /// matrix.gauss_jordan_elimination();
    /// // matrix is now (approximately)
    /// // [1, 0, 0, 2]
    /// // [0, 1, 0, 3]
    /// // [0, 0, 1, -1]
    /// ```
    ///
    /// Which translates to the following equations/solutions:
    ///
    /// ```math
    /// x = 2
    /// y = 3
    /// z = -1
    /// ```
    pub fn gauss_jordan_elimination(&mut self) {
        let mut pivot_row = 0;
        let mut pivot_col = 0;

        while pivot_row < R && pivot_col < C {
            let (max_idx, max) = self
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
                self.swap(max_idx, pivot_row);
            }

            for r in (pivot_row + 1)..R {
                let f = self[r][pivot_col] / self[pivot_row][pivot_col];
                self[r][pivot_col] = 0.0;
                for c in (pivot_col + 1)..C {
                    self[r][c] -= self[pivot_row][c] * f;
                }
            }
            pivot_row += 1;
            pivot_col += 1;
        }

        for pivot_row in (0..R).rev() {
            let Some((pivot_col, _)) = self[pivot_row]
                .iter()
                .enumerate()
                .find(|(_, v)| v.abs() > f64::EPSILON)
            else {
                continue;
            };

            let f = self[pivot_row][pivot_col].recip();
            for c in pivot_col..C {
                self[pivot_row][c] *= f;
            }

            for r in 0..pivot_row {
                let f = self[r][pivot_col];
                for c in pivot_col..C {
                    self[r][c] -= self[pivot_row][c] * f;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
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
        let mut matrix = super::Matrix([
            [2.0, 1.0, -1.0, 8.0],
            [-3.0, -1.0, 2.0, -11.0],
            [-2.0, 1.0, 2.0, -3.0],
        ]);
        matrix.gauss_jordan_elimination();
        assert_eq_approx!(matrix[0][0], 1.0);
        assert_eq_approx!(matrix[0][1], 0.0);
        assert_eq_approx!(matrix[0][2], 0.0);
        assert_eq_approx!(matrix[0][3], 2.0);
        assert_eq_approx!(matrix[1][0], 0.0);
        assert_eq_approx!(matrix[1][1], 1.0);
        assert_eq_approx!(matrix[1][2], 0.0);
        assert_eq_approx!(matrix[1][3], 3.0);
        assert_eq_approx!(matrix[2][0], 0.0);
        assert_eq_approx!(matrix[2][1], 0.0);
        assert_eq_approx!(matrix[2][2], 1.0);
        assert_eq_approx!(matrix[2][3], -1.0);
    }
}
