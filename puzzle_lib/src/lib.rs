//! Various utilities to help with things that appear commonly in puzzles.

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
        use puzzle_lib::prelude::*;

        // Register binary.
        puzzle_runner::register!($($name = $value),*);
    };
}
