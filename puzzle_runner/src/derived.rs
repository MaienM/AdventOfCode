//! Structures used for the data that is injected by the macros in `puzzle_runner_derive`.

use std::{collections::HashMap, ops::Deref, sync::Arc};

use derive_builder::Builder;

use crate::controller::Controller;

/// A single puzzle series. See [`puzzle_runner#Chapter`].
#[derive(Clone, Builder)]
#[cfg_attr(
    feature = "wasm",
    derive(serde::Serialize, tsify::Tsify),
    tsify(into_wasm_abi, missing_as_null)
)]
#[builder(field(public))]
pub struct Series {
    /// The name of crate containing the series.
    pub name: &'static str,

    /// The title of the series.
    #[builder(setter(into))]
    pub title: String,

    /// The chapters in the series.
    pub chapters: Vec<Chapter>,

    /// The controller that handles actions for this series.
    #[cfg_attr(feature = "wasm", serde(skip_serializing))]
    pub controller: Arc<Box<dyn Controller>>,
}

/// A single chapter in a [`Series`]. See [`puzzle_runner#Series`].
#[derive(Clone, Builder)]
#[cfg_attr(
    feature = "wasm",
    derive(serde::Serialize, tsify::Tsify),
    tsify(into_wasm_abi, missing_as_null)
)]
#[builder(field(public))]
pub struct Chapter {
    /// The name of the binary containing the chapter.
    pub name: &'static str,

    /// The book this chapter is part of.
    #[builder(setter(into, strip_option), default)]
    pub book: Option<String>,

    /// The title of the chapter.
    #[builder(setter(into, strip_option), default)]
    pub title: Option<String>,

    /// The path of the source file, relative to the root of the repository.
    pub source_path: &'static str,

    /// The parts for this chapter.
    pub parts: Vec<Part>,

    /// The examples.
    pub examples: Vec<Example>,
}

/// A single part in a [`Chapter`]. See [`puzzle_runner#Part`].
#[derive(Clone, Builder)]
#[cfg_attr(
    feature = "wasm",
    derive(serde::Serialize, tsify::Tsify),
    tsify(into_wasm_abi, missing_as_null)
)]
#[builder(field(public))]
pub struct Part {
    /// The number of the part.
    pub num: u8,

    /// The implementation for this part, with the result converted to a string.
    #[cfg_attr(feature = "wasm", serde(skip_serializing))]
    pub implementation: fn(&str) -> String,
}

/// An example input for a chapter.
#[derive(Clone, Builder)]
#[cfg_attr(
    feature = "wasm",
    derive(serde::Serialize, tsify::Tsify),
    tsify(into_wasm_abi, missing_as_null)
)]
#[builder(field(public))]
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
