//! Structures used for the data that is injected by the macros in `aoc_runner_derive`.

use std::ops::Deref;

/// The main components of the implementation of a single day.
#[derive(Clone)]
pub struct Bin {
    /// The name of the binary.
    pub name: &'static str,

    /// The title of the puzzle.
    pub title: Option<&'static str>,

    /// The year that the binary is for (last 2 digits only).
    pub year: u8,

    /// The day that the binary is for.
    pub day: u8,

    /// The runnable for part 1, with the result cast to a string.
    pub part1: Solver<String>,

    /// The runnable for part 2, with the result cast to a string.
    pub part2: Solver<String>,

    /// The visualizer for part 1.
    #[cfg(feature = "visual")]
    pub visual1: Solver<Box<dyn crate::visual::Renderable>>,

    /// The visualizer for part 2.
    #[cfg(feature = "visual")]
    pub visual2: Solver<Box<dyn crate::visual::Renderable>>,

    /// The examples.
    pub examples: Vec<Example>,
}

/// The implementation (or lack thereof) for a single puzzle within a [`Bin`].
#[derive(Clone)]
pub enum Solver<T> {
    Implemented(fn(&str) -> T),
    NotImplemented,
}
impl<T> Solver<T> {
    pub fn is_implemented(&self) -> bool {
        match self {
            Solver::Implemented(_) => true,
            Solver::NotImplemented => false,
        }
    }
}

/// An example input.
#[derive(Clone)]
pub struct Example {
    /// The name of the example.
    pub name: &'static str,

    /// The example input.
    pub input: &'static str,

    /// The expected result of part 1, cast to a string.
    pub part1: Option<&'static str>,

    /// The expected result of part 2, cast to a string.
    pub part2: Option<&'static str>,
}
impl Deref for Example {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        &self.input
    }
}
