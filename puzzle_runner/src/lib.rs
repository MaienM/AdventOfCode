//! Scaffolding to run/test/benchmark solutions for programming puzzles/challenges.
//!
//! This uses the following hierarchy:
//!
//! ## Series
//!
//! This represents a single source of puzzles (e.g., Advent of Code). This series will be in its
//! own crate with the [`register_series`] macro. This is represented by an (automatically
//! generated) [Series](`puzzle_runner::derived::Series`).
//!
//! ## Book
//!
//! This represents a logical grouping of puzzles within a series (e.g., a single year for Advent
//! of Code). This is part of the metadata of the (Chapter)[`puzzle_runner#Chapter`]s in the
//! series.
//!
//! ## Chapter
//!
//! This represents a (possibly multi-part) puzzle in within the series. This will be in its own
//! binary (in the `bin` folder of the series) with the [`register_chapter`] macro. This is
//! represented by an (automatically generated) [Chapter](`puzzle_runner::derived::Chapter`).
//!
//! ## Part
//!
//! This represents a part of a puzzle. This will be a public function in the chapter binary with a
//! name in the format `partN`. This is represented by an (automatically generated)
//! [Part](`puzzle_runner::derived::Part`).

#[doc(hidden)]
extern crate self as puzzle_runner;

pub mod bench;
pub mod derived;
pub mod multi;
pub mod runner;
pub mod single;
mod source;

pub use puzzle_runner_derive::{example_input, register_chapter, register_series};

#[cfg(not(target_arch = "wasm32"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
