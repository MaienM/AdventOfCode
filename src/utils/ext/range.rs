//! Extension methods for [`std::ops::RangeBounds`].

use std::ops::{Add, Bound, Div, RangeBounds, Sub};

/// Extension methods for [`std::ops::RangeBounds`].
pub trait RangeExt<T> {
    /// Returns the index of the partition point according to the given predicate (the index of the first element of the second partition).
    fn partition_point<F>(&self, f: F) -> Option<T>
    where
        F: Fn(T) -> bool;
}
impl<R, T> RangeExt<T> for R
where
    R: RangeBounds<T>,
    T: Add<usize, Output = T>
        + Add<T, Output = T>
        + Sub<usize, Output = T>
        + Sub<T, Output = T>
        + Div<usize, Output = T>
        + Copy
        + PartialEq,
{
    fn partition_point<F>(&self, f: F) -> Option<T>
    where
        F: Fn(T) -> bool,
    {
        let mut min = match self.start_bound() {
            Bound::Included(n) => *n,
            Bound::Excluded(n) => *n + 1,
            Bound::Unbounded => panic!("Cannot do binary search on an unbounded range."),
        };
        let mut max = match self.end_bound() {
            Bound::Included(n) => *n,
            Bound::Excluded(n) => *n - 1,
            Bound::Unbounded => panic!("Cannot do binary search on an unbounded range."),
        };
        while min != max {
            let midpoint = min + (max - min) / 2;
            if f(midpoint) {
                max = midpoint;
            } else {
                min = midpoint + 1;
            }
        }
        if f(min) {
            Some(min)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn partition_point() {
        assert_eq!((1..10).partition_point(|v| v > 6), Some(7));
        assert_eq!(
            (1..1_000_000_000).partition_point(|v| v > 628_162_832),
            Some(628_162_833)
        );
        assert_eq!((1..10).partition_point(|v| v > 0), Some(1));
        assert_eq!((1..10).partition_point(|v| v > 8), Some(9));
        assert_eq!((1..10).partition_point(|v| v > 9), None);

        assert_eq!((1..=10).partition_point(|v| v > 6), Some(7));
        assert_eq!(
            (1..1_000_000_000).partition_point(|v| v > 628_162_832),
            Some(628_162_833)
        );
        assert_eq!((1..=10).partition_point(|v| v > 0), Some(1));
        assert_eq!((1..=10).partition_point(|v| v > 9), Some(10));
        assert_eq!((1..=10).partition_point(|v| v > 10), None);
    }
}
