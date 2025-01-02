//! My solutions to the [Advent of Code](https://adventofcode.com) challenges.

#[doc(hidden)]
extern crate self as aoc;

pub mod bins;
mod ext;
pub mod matrix;
pub mod parser;
pub mod point;
pub mod prelude;

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
