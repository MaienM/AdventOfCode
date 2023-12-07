use std::ops::Deref;

use crate::runner::Runnable;

/// An example input.
pub struct Example {
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

/// The main components of the implementation of a single day.
pub struct Day {
    /// The name of the day.
    pub name: &'static str,

    /// The runnable for part 1, with the result cast to a string.
    pub part1: Runnable<String>,

    /// The runnable for part 2, with the result cast to a string.
    pub part2: Runnable<String>,

    /// The examples.
    pub examples: Vec<Example>,
}
