//! Helpers for mathematical matrices.

use std::{
    fmt::{self, Debug},
    ops::{Deref, DerefMut},
};

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
    /// # use puzzle_lib::matrix::Matrix;
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
    /// # use puzzle_lib::matrix::Matrix;
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
    /// Perform [Gauss-Jordan elimination](https://en.wikipedia.org/wiki/Gaussian_elimination), and
    /// return the last column.
    ///
    /// This is a method to solve a system of linear equations. Generally speaking you will need as
    /// many equations as you have unknowns, though there might be cases where you need more (e.g
    /// if two equations are equivalent (can be transformed to one another), such as $`x + y = 1`$
    /// and $`2x + 2y = 2`$).
    ///
    /// Beware of floating point math, if you're going to cast the result to an integer you will
    /// want to round it first.
    ///
    /// # Examples
    ///
    /// Given the following set of equations:
    ///
    /// ```math
    ///  2x + y -  z = 8 \\
    /// -3x - y + 2z = -11 \\
    /// -2x + y + 2z = -3 \\
    /// ```
    ///
    /// We can express this as a matrix and solve it:
    ///
    /// ```
    /// # use puzzle_lib::matrix::Matrix;
    /// let matrix = Matrix::new([
    ///     [ 2,  1, -1,   8],
    ///     [-3, -1,  2, -11],
    ///     [-2,  1,  2,  -3],
    /// ]);
    /// let result = matrix.gauss_jordan_elimination().unwrap();
    /// assert_eq!(result[0].round(), 2.0);  // x
    /// assert_eq!(result[1].round(), 3.0);  // y
    /// assert_eq!(result[2].round(), -1.0); // z
    /// ```
    ///
    /// # Explanation
    ///
    /// The first phase of this is to convert it to echelon row form, which has the effect of
    /// eliminating some of the variables from some of the equations. This is done by taking each
    /// equation (working from top to bottom) and adding it to the equations below it (multiplying
    /// it by some factor) to eliminate one of the variables (working from left to right) from the
    /// second equation.
    ///
    /// During this phase we also take the step to reorder the equations to have the largest
    /// (absolute) number for the column we're looking at at the top. This is not strictly
    /// required, but this helps get a more precise result due to floating point behaviour.
    ///
    /// ```math
    ///  2x + y -  z = 8 \\
    /// -3x - y + 2z = -11 \\
    /// -2x + y + 2z = -3 \\
    /// ```
    ///
    /// is reordered to
    ///
    /// ```math
    /// -3x - y + 2z = -11 \\
    ///  2x + y -  z = 8 \\
    /// -2x + y + 2z = -3 \\
    /// ```
    ///
    /// and then the second and third equation have (a multiple of) the first equation
    /// added/removed to eliminate `x`
    ///
    /// ```math
    /// \Big(2x + y - z\Big) - \Big(\frac{-2}3 * (-3x - y + 2z)\Big) = 8 - \Big(\frac{-2}3 * -11\Big) \\
    /// \Big(2x + y - z\Big) - \Big(2x + \frac23y - 1\frac13z\Big) = 8 - 7\frac13 \\
    /// \frac13y - \frac13z = \frac23 \\
    /// ```
    ///
    /// ```math
    /// \Big(-2z + y + 2z\Big) - \Big(\frac23 * (-3x - y + 2z)\Big) = 8 - \Big(\frac23 * -11\Big) \\
    /// \Big(-2z + y + 2z\Big) - \Big(-2x - \frac23y + 1\frac13z\Big) = 8 - -7\frac13 \\
    /// 1\frac23y + \frac23z = 4\frac13 \\
    /// ```
    ///
    /// resulting in the following set of equations
    ///
    /// ```math
    /// -3x - y + 2z = -11 \\
    /// \frac13y + \frac13z = \frac23 \\
    /// 1\frac23y + \frac23z = 4\frac13 \\
    /// ```
    ///
    /// This same process is then repeated starting at the second column and row, first reordering
    ///
    /// ```math
    /// -3x - y + 2z = -11 \\
    /// 1\frac23y + \frac23z = 4\frac13 \\
    /// \frac13y + \frac13z = \frac23 \\
    /// ```
    ///
    /// and then using the second equation to eliminate `y` from the third equation
    ///
    /// ```math
    /// \Big(\frac13y + \frac13z\Big) - \frac15 * \Big(1\frac23y + \frac23z\Big) = \frac23 - \Big(\frac15 * 4\frac13\Big) \\
    /// \Big(\frac13y + \frac13z\Big) - \Big(\frac13y + \frac2{15}z\Big) = \frac23 - \frac{13}{15} \\
    /// \frac15z = \frac{-1}5 \\
    /// ```
    ///
    /// resulting in the following set of equations
    ///
    /// ```math
    /// -3x - y + 2z = -11 \\
    /// \frac13y + \frac13z = \frac23 \\
    /// \frac15z = \frac{-1}5 \\
    /// ```
    ///
    /// At this point we can follow this process no further, which means we've finished the first
    /// phase and have arrived at row echelon format. The second phase is to get to reduced row
    /// echelon format, which will solve for our variables (if possible given the input equations).
    ///
    /// This is done by taking each equation (working from bottom to top), multiplying it so that
    /// its first (and hopefully only, else there will be no solution for this set of equations)
    /// variable has a multiplier of `1`, and then using this equation to eliminate this variable
    /// from the equations above it.
    ///
    /// ```math
    /// 5 * \frac15z = 5 * \frac{-1}5 \\
    /// z = -1 \\
    /// ```
    ///
    /// this can then be used to eliminate `z` from the equations above it
    ///
    /// ```math
    /// -3x - y + 2z - 2 * z = -11 - 2 * (-1) \\
    /// -3x - y = -9 \\
    /// ```
    ///
    /// ```math
    /// \frac13y + \frac13z - \frac13 * z = \frac23 - \frac13 * (-1) \\
    /// \frac13y = 1 \\
    /// ```
    ///
    /// resulting in the following set of equations
    ///
    /// ```math
    /// -3x - y = -9 \\
    /// \frac13y = 1 \\
    /// z = -1 \\
    /// ```
    ///
    /// This leaves the second equation with only `y` as variable, so we can repeat this process,
    /// first multiplying it to get `1y`
    ///
    /// ```math
    /// \frac13y = 1 \\
    /// y = 3 \\
    /// ```
    ///
    /// and then using it to eliminate `y` from the first equation
    ///
    /// ```math
    /// -3x - y + y = -9 + 3\\
    /// -3x = -6 \\
    /// ```
    ///
    /// resulting in the following set of equations
    ///
    /// ```math
    /// -3x = -6 \\
    /// y = 3 \\
    /// z = -1 \\
    /// ```
    ///
    /// Finally, this leaves the first equation with only `x` as variable, so we can once again
    /// multiply it to get `1x`
    ///
    /// ```math
    /// -3x = -6 \\
    /// x = 2 \\
    /// ```
    ///
    /// resulting in the following set of equations/answers
    ///
    /// ```math
    /// x = 2 \\
    /// y = 3 \\
    /// z = -1 \\
    /// ```
    #[must_use]
    pub fn gauss_jordan_elimination(&self) -> Option<[f64; C - 1]> {
        let mut matrix = self.cast::<f64>().unwrap();

        // We need at least as many expressions as there are variables.
        if R < C - 1 {
            return None;
        }

        // Transform the matrix into row echelon form using Gaussian elimination.
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

        // Transform the matrix into reduced row echelon form using back substitution.
        for pivot_row in (0..R).rev() {
            let Some((pivot_col, _)) = matrix[pivot_row]
                .iter()
                .enumerate()
                .find(|(_, v)| v.abs() > f64::EPSILON)
            else {
                // Empty row, which means this equation was an exact multiple of another and has
                // been fully eliminated at this point.
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

        // At this point each row should have at most two non-zero values; one arbitrary value in
        // the last column (the value of the variable, could be zero) and one in one of the other
        // columns with value `1` which indicates which variable that answer is for. Verify this is
        // the case & that the copy the anwser to the results.
        let mut results = [0.0; C - 1];
        for r in 0..R {
            for c in 0..(C - 1) {
                if (matrix[r][c] - (if r == c { 1.0 } else { 0.0 })) > 0.000_000_001 {
                    return None;
                }
            }
            if r < C - 1 {
                results[r] = matrix[r][C - 1];
            }
        }
        Some(results)
    }
}

#[cfg(test)]
mod tests {
    use core::f64;

    use super::*;

    macro_rules! assert_eq_approx {
        ($actual:expr, $expected:expr $(,)?) => {{
            let actual = $actual;
            let expected = $expected;
            assert!(
                (actual - expected).abs() < 0.000_000_001,
                "expected {:?} to approximately equal {:?}",
                actual,
                expected,
            );
        }};
    }

    #[test]
    fn gauss_jordan_elimination() {
        #[rustfmt::skip]
        let matrix = Matrix([
            [ 2,  1, -1,   8],
            [-3, -1,  2, -11],
            [-2,  1,  2,  -3],
        ]);
        let result = matrix.gauss_jordan_elimination();
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq_approx!(result[0], 2.0);
        assert_eq_approx!(result[1], 3.0);
        assert_eq_approx!(result[2], -1.0);
    }

    /// Validate that one of the answers being 0 does not trip up the check for whether the matrix
    /// is solved at the end.
    #[test]
    fn gauss_jordan_elimination_zero_answer() {
        #[rustfmt::skip]
        let matrix = Matrix([
            [ 2,  1, -1,  5],
            [-3, -1,  2, -8],
            [-2,  1,  2, -6],
        ]);
        let result = matrix.gauss_jordan_elimination();
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq_approx!(result[0], 2.0);
        assert_eq_approx!(result[1], 0.0);
        assert_eq_approx!(result[2], -1.0);
    }

    /// Validate that having more equations than necessary doesn't cause issues.
    #[test]
    fn gauss_jordan_elimination_extra_equations() {
        #[rustfmt::skip]
        let matrix = Matrix([
            [ 2,  1, -1,   8],
            [ 4,  2, -2,  16],
            [-3, -1,  2, -11],
            [-2,  1,  2,  -3],
            [ 1, -2,  3,  -7],
            [ 2,  3, -1,  14],
        ]);
        let result = matrix.gauss_jordan_elimination();
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq_approx!(result[0], 2.0);
        assert_eq_approx!(result[1], 3.0);
        assert_eq_approx!(result[2], -1.0);
    }

    /// Validate that having less equations than variables results in no answer.
    #[test]
    fn gauss_jordan_elimination_insufficient_equations() {
        #[rustfmt::skip]
        let matrix = Matrix([
            [ 2,  1, -1,  8],
            [-2,  1,  2, -3],
        ]);
        let result = matrix.gauss_jordan_elimination();
        assert!(result.is_none());
    }

    /// Validate that having equations that don't provide enough information results in no answer.
    #[test]
    fn gauss_jordan_elimination_insufficent_unique_equations() {
        #[rustfmt::skip]
        let matrix = Matrix([
            [ 2,  1, -1,  8],
            [ 4,  2, -2, 16], // this row is 2x the previous row
            [-2,  1,  2, -3],
        ]);
        let result = matrix.gauss_jordan_elimination();
        assert!(result.is_none());
    }

    /// Validate that having conflicting equations results in no answer.
    #[test]
    fn gauss_jordan_elimination_conflicting_equations() {
        #[rustfmt::skip]
        let matrix = Matrix([
            [ 2,  1, -1,  8],
            [ 4,  2, -2, 6],
            [-2,  1,  2, -3],
        ]);
        let result = matrix.gauss_jordan_elimination();
        assert!(result.is_none());
    }

    #[test]
    fn format() {
        let matrix = Matrix([
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
