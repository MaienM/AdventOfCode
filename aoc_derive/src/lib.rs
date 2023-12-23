#![feature(proc_macro_span)]

mod examples;
mod scanner;
mod visual;

use proc_macro::TokenStream;

/// Define a static that will hold a list of all [`aoc::derived::Day`]s for all days.
///
/// This will also include all days as modules.
#[proc_macro_attribute]
pub fn inject_days(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    scanner::inject_days(input, annotated_item)
}

/// Define a static that will hold the [`aoc::derived::Day`] for the current file.
#[proc_macro_attribute]
pub fn inject_day(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    scanner::inject_day(input, annotated_item)
}

/// Mark an attribute as an example input.
///
/// The leading and trailing newline + a static amount of indentation for each line will be stripped to make the input match the original. The result will be stored in an [`aoc::derived::Example`] along with the expected outputs (if provided).
///
/// A test will be generated for each part that has an expected output defined.
#[proc_macro_attribute]
pub fn example_input(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    examples::example_input(input, annotated_item)
}

/// Mark a module as the one used for visualizations.
///
/// Must be used exactly once in days where the [`inform_visual!`] macro is used.
#[proc_macro_attribute]
pub fn visual(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    visual::visual(input, annotated_item)
}

/// Derive macro for [`aoc::visual::ToRenderable`].
///
/// Requires annotated struct to implement [`aoc::visual::Visual`] and to be located in a module annotated with [`aoc_derive::visual`].
#[proc_macro_derive(ToRenderable)]
pub fn derive_renderable(item: TokenStream) -> TokenStream {
    visual::derive_renderable(item)
}
