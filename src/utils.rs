use std::ops::{Add, Bound, Div, RangeBounds, Sub};

/// Calculate the absolute difference between two (possibly unsigned) integers.
pub fn abs_diff<T>(a: T, b: T) -> T
where
    T: PartialOrd + Sub<Output = T>,
{
    if a > b {
        a - b
    } else {
        b - a
    }
}

/// Binary search on a range.
pub fn range_binary_search<R, T, F>(range: R, f: F) -> Option<T>
where
    R: RangeBounds<T>,
    T: Add<usize, Output = T>
        + Add<T, Output = T>
        + Sub<usize, Output = T>
        + Sub<T, Output = T>
        + Div<usize, Output = T>
        + Copy
        + PartialEq,
    F: Fn(T) -> bool,
{
    let mut min = match range.start_bound() {
        Bound::Included(n) => *n,
        Bound::Excluded(n) => *n + 1,
        Bound::Unbounded => panic!("Cannot do binary search on an unbounded range."),
    };
    let mut max = match range.end_bound() {
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_abs_diff() {
        assert_eq!(abs_diff(1, 10), 9);
        assert_eq!(abs_diff(10, 1), 9);
        assert_eq!(abs_diff(10, -1), 11);
    }

    #[test]
    fn test_range_binary_search() {
        assert_eq!(range_binary_search(1..10, |v| v > 6), Some(7));
        assert_eq!(
            range_binary_search(1..1_000_000_000, |v| v > 628_162_832),
            Some(628_162_833)
        );
        assert_eq!(range_binary_search(1..10, |v| v > 0), Some(1));
        assert_eq!(range_binary_search(1..10, |v| v > 8), Some(9));
        assert_eq!(range_binary_search(1..10, |v| v > 9), None);

        assert_eq!(range_binary_search(1..=10, |v| v > 6), Some(7));
        assert_eq!(
            range_binary_search(1..1_000_000_000, |v| v > 628_162_832),
            Some(628_162_833)
        );
        assert_eq!(range_binary_search(1..=10, |v| v > 0), Some(1));
        assert_eq!(range_binary_search(1..=10, |v| v > 9), Some(10));
        assert_eq!(range_binary_search(1..=10, |v| v > 10), None);
    }
}
