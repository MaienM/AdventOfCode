//! Various utilities to help with things that appear commonly in puzzles.
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod ext;
pub mod grid;
mod macros;
pub mod matrix;
pub mod parser;
pub mod point;
pub mod prelude;

/// Mark a point in the program that should never be reached.
#[macro_export]
macro_rules! never {
    () => {
        panic!("This should never happen.");
    };
}
