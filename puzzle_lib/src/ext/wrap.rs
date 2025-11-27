use std::ops::{AddAssign, Range, RangeInclusive};

use num::PrimInt;

/// Wrap a number to fit within a range.
pub trait WrapRange<T> {
    /// Wrap the value to fit within the range, with values outside the bounds wrapping to the
    /// other side.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::prelude::*;
    /// assert_eq!((2..5).wrap(6), 3);
    /// assert_eq!((2..=5).wrap(0), 4);
    /// ```
    fn wrap(&self, value: T) -> T;
}

#[inline]
fn wrap<T>(mut value: T, start: T, end: T) -> T
where
    T: PrimInt + AddAssign,
{
    let size = end - start;
    if value < start {
        value += ((start - value) / size + T::one()) * size;
    }
    start + (value - start) % size
}

impl<T> WrapRange<T> for Range<T>
where
    T: PrimInt + AddAssign,
{
    fn wrap(&self, value: T) -> T {
        wrap(value, self.start, self.end)
    }
}

impl<T> WrapRange<T> for RangeInclusive<T>
where
    T: PrimInt + AddAssign,
{
    fn wrap(&self, value: T) -> T {
        wrap(value, *self.start(), *self.end() + T::one())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn wrap_unsigned() {
        assert_eq!((3..8).wrap(10), 5);
        assert_eq!((3..8).wrap(50), 5);
        assert_eq!((3..=8).wrap(10), 4);
        assert_eq!((3..=8).wrap(0), 6);
    }

    #[test]
    fn wrap_unsigned_bounds() {
        // A naive implementation might attempt to add both numbers at some point and overflow in
        // the process, so check near the bounds.
        assert_eq!((250..254).wrap(255), 251);
        assert_eq!((250..254).wrap(248), 252);
    }

    #[test]
    fn wrap_signed() {
        assert_eq!((-3..4).wrap(8), 1);
        assert_eq!((-3..4).wrap(-8), -1);
        assert_eq!((-3..=4).wrap(8), 0);
    }

    #[test]
    fn wrap_signed_bounds() {
        // Similar to the unsigned bound test, but in addition a naive implementation might try to
        // calculate the difference between the numbers, which for signed numbers can also overfow.
        assert_eq!((124..126).wrap(127), 125);
        assert_eq!((-120..-110).wrap(120), -120);
    }
}
