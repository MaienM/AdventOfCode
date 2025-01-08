use std::ops::{Bound, RangeBounds};

use num::PrimInt;

/// Find the partition point of a collection, as [`slice::partition_point`].
pub trait PartitionPoint<T> {
    /// Returns the index of the partition point according to the given predicate (the index of the first element of the second partition).
    ///
    /// See [`slice::partition_point`].
    fn partition_point<F>(&self, f: F) -> Option<T>
    where
        F: Fn(T) -> bool;
}
impl<R, T> PartitionPoint<T> for R
where
    R: RangeBounds<T>,
    T: PrimInt,
{
    fn partition_point<F>(&self, f: F) -> Option<T>
    where
        F: Fn(T) -> bool,
    {
        let mut min = match self.start_bound() {
            Bound::Included(n) => *n,
            Bound::Excluded(n) => *n + T::one(),
            Bound::Unbounded => panic!("Cannot find partition point of an unbounded range."),
        };
        let mut max = match self.end_bound() {
            Bound::Included(n) => *n,
            Bound::Excluded(n) => *n - T::one(),
            Bound::Unbounded => panic!("Cannot find partition point of an unbounded range."),
        };
        while min != max {
            let midpoint = min + ((max - min) >> 1);
            if f(midpoint) {
                max = midpoint;
            } else {
                min = midpoint + T::one();
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
        assert_eq!((1..10).partition_point(|_| true), Some(1));
        assert_eq!((1..10).partition_point(|v| v > 8), Some(9));
        assert_eq!((1..10).partition_point(|v| v > 9), None);

        assert_eq!((1..=10).partition_point(|_| true), Some(1));
        assert_eq!((1..=10).partition_point(|v| v > 9), Some(10));
        assert_eq!((1..=10).partition_point(|v| v > 10), None);

        assert_eq!(
            (1..1_000_000_000).partition_point(|v| v > 628_162_832),
            Some(628_162_833)
        );
    }
}
