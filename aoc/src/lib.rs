//! My solutions to the [Advent of Code](https://adventofcode.com) challenges.
aoc_runner::register_crate!();

#[doc(hidden)]
extern crate self as aoc;

mod ext;
pub mod matrix;
pub mod parser;
pub mod point;
pub mod prelude;

/// Basic scaffold for a solution.
#[macro_export]
macro_rules! setup {
    ($($name:ident = $value:literal),* $(,)?) => {
        // Include prelude.
        #[allow(unused_imports)]
        use aoc::prelude::*;

        // Register binary.
        aoc_runner::register!($($name = $value),*);
    };
}
