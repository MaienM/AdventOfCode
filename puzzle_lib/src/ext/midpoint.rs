use num::PrimInt;

/// Computes the midpoint between two numbers.
pub trait Midpoint {
    /// Calculates the middle point of `self` and `rhs`.
    ///
    /// midpoint(a, b) is (a + b) >> 1 as if it were performed in a sufficiently-large signed
    /// integral type. This implies that the result is always rounded towards negative infinity and
    /// that no overflow will ever occur.
    ///
    /// (This is pretty much just the built-in experimental [`core::num::NonZero::midpoint`], but
    /// without the experimental flag & with implementations for `num` traits).
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::prelude::*;
    /// assert_eq!(u8::mid_point(3, 8), 5);
    /// assert_eq!(i8::mid_point(-12, 24), 6);
    /// ```
    #[must_use]
    fn mid_point(self, rhs: Self) -> Self;
}

impl<T> Midpoint for T
where
    T: PrimInt,
{
    fn mid_point(self, rhs: Self) -> Self {
        (self >> 1) + (rhs >> 1) + (self & rhs & Self::one())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn midpoint_unsigned() {
        assert_eq!(u8::mid_point(3, 7), 5);
        assert_eq!(u8::mid_point(3, 8), 5);
        assert_eq!(u8::mid_point(7, 3), 5);
        assert_eq!(u8::mid_point(8, 3), 5);
    }

    #[test]
    fn midpoint_unsigned_bounds() {
        // A naive implementation might attempt to add both numbers at some point and overflow in
        // the process, so check near the bounds.
        assert_eq!(u8::mid_point(250, 254), 252);
    }

    #[test]
    fn midpoint_signed() {
        assert_eq!(i8::mid_point(-12, 24), 6);
        assert_eq!(i8::mid_point(-12, 25), 6);
        assert_eq!(i8::mid_point(24, -12), 6);
        assert_eq!(i8::mid_point(24, -11), 6);

        assert_eq!(i8::mid_point(-13, 23), 5);
    }

    #[test]
    fn midpoint_signed_bounds() {
        // Similar to the unsigned bound test, but in addition a naive implementation might try to
        // calculate the difference between the numbers, which for signed numbers can also overfow.
        assert_eq!(i8::mid_point(124, 126), 125);
        assert_eq!(i8::mid_point(-124, 126), 1);
    }
}
