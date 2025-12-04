#![feature(proc_macro_span)]
// The macros & docs will be re-exported in `puzzle_runner`, and the links will work there.
#![allow(rustdoc::broken_intra_doc_links)]

mod chapters;
mod examples;
mod series;

use proc_macro::TokenStream;

/// Register the crate as a puzzle series.
///
/// Must be applied on the crate root (`lib.rs`).
///
/// This will collect all [`puzzle_runner::derived::Chapter`]s in the crate + other metadata &
/// expose it as a static (`SERIES`).
#[proc_macro]
pub fn register_series(input: TokenStream) -> TokenStream {
    series::register_series(input)
}

/// Register the binary crate as a puzzle chapter.
///
/// Must be applied to a binary (in the `bin` folder).
///
/// This will collect all parts/examples/metadata from that file into a
/// [`puzzle_runner::derived::Chapter`], expose it as a static (`CHAPTER`), and generate a main
/// entrypoint.
#[proc_macro]
pub fn register_chapter(input: TokenStream) -> TokenStream {
    chapters::register_chapter(input)
}

/// Mark an attribute as an example input.
///
/// The leading and trailing newline + a static amount of indentation for each line will be
/// stripped to make the input match the original. The result will be stored in an
/// [`Example`](puzzle_runner::derived::Example) along with the expected outputs (if provided).
///
/// A test will be generated for each part that has an expected output defined.
#[proc_macro_attribute]
pub fn example_input(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    examples::example_input(input, annotated_item)
}
