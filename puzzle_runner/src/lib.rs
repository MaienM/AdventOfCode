//! Scaffolding to run/test/benchmark solutions for programming puzzles/challenges.
//!
//! Each group of puzzles is solved in a separate source file in the `bin` folder of the crate. The
//! [`register`] macro generates a [`Bin`](puzzle_runner::derived::Bin) for each of these, and the
//! [`register_crate`] macro generates a static & the entrypoints needed to run multiple at the
//! same time.

#[doc(hidden)]
extern crate self as puzzle_runner;

pub mod bench;
pub mod derived;
pub mod multi;
pub mod runner;
pub mod single;
mod source;

pub use puzzle_runner_derive::{example_input, register, register_crate};

#[cfg(not(target_arch = "wasm32"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
