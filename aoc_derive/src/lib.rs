mod examples;
mod scanner;

use proc_macro::TokenStream;

/// Define a static that will hold a list of all [`aoc::derived::Day`]s for all days.
#[proc_macro]
pub fn scan_days(input: TokenStream) -> TokenStream {
    scanner::scan_days(input)
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
