use std::ops::Sub;

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
}
