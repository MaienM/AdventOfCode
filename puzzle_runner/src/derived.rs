//! Structures used for the data that is injected by the macros in `puzzle_runner_derive`.

use std::{collections::HashMap, ops::Deref};

/// A single puzzle series. See [`puzzle_runner#Chapter`].
#[derive(Clone)]
pub struct Series {
    /// The name of crate containing the series.
    pub name: &'static str,

    /// The title of the series.
    pub title: &'static str,

    /// The chapters in the series.
    pub chapters: Vec<Chapter>,
}

/// A single chapter in a [`Series`]. See [`puzzle_runner#Series`].
#[derive(Clone)]
pub struct Chapter {
    /// The name of the binary containing the chapter.
    pub name: &'static str,

    /// The book this chapter is part of.
    pub book: Option<&'static str>,

    /// The title of the chapter.
    pub title: Option<&'static str>,

    /// The path of the source file, relative to the root of the repository.
    pub source_path: &'static str,

    /// The parts for this chapter.
    pub parts: Vec<Part>,

    /// The examples.
    pub examples: Vec<Example>,
}

/// A single part in a [`Chapter`]. See [`puzzle_runner#Part`].
#[derive(Clone)]
pub struct Part {
    /// The number of the part.
    pub num: u8,

    /// The implementation for this part, with the result converted to a string.
    pub implementation: fn(&str) -> String,
}

/// An example input for a chapter.
#[derive(Clone)]
pub struct Example {
    /// The name of the example.
    pub name: &'static str,

    /// The example input.
    pub input: &'static str,

    /// The expected results for the parts, cast to strings.
    pub parts: HashMap<u8, &'static str>,
}
impl Deref for Example {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        &self.input
    }
}
