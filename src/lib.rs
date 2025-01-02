//! My solutions to the [Advent of Code](https://adventofcode.com) challenges.

#[doc(hidden)]
extern crate self as aoc;

pub mod bins;
mod ext;
pub mod matrix;
pub mod parser;
pub mod point;
pub mod prelude;

use std::ops::Sub;

/// Basic scaffold for a solution.
#[macro_export]
macro_rules! setup {
    () => {
        // Include prelude.
        use aoc::prelude::*;

        // Generate entrypoint.
        aoc_runner::single::generate_main!();
    };
}

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
