pub mod ext;
pub mod parser;
pub mod point;

use std::ops::Sub;

pub use parser::parse;

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

    #[test]
    fn abs_diff() {
        assert_eq!(super::abs_diff(1, 10), 9);
        assert_eq!(super::abs_diff(10, 1), 9);
        assert_eq!(super::abs_diff(10, -1), 11);
    }
}
