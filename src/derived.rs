//! Structures used for the data that is injected by the macros in `aoc_derive`.

use std::ops::Deref;

type Processor<T> = Option<fn(&str) -> T>;

/// The main components of the implementation of a single day.
#[derive(Clone)]
pub struct Day {
    /// The name of the day.
    pub name: &'static str,

    /// The numer of the day.
    pub num: u8,

    /// The runnable for part 1, with the result cast to a string.
    pub part1: Processor<String>,

    /// The runnable for part 2, with the result cast to a string.
    pub part2: Processor<String>,

    /// The visualizer for part 1.
    #[cfg(feature = "visual")]
    pub visual1: Processor<Box<dyn crate::visual::Renderable>>,

    /// The visualizer for part 2.
    #[cfg(feature = "visual")]
    pub visual2: Processor<Box<dyn crate::visual::Renderable>>,

    /// The examples.
    pub examples: Vec<Example>,
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
