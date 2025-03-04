#![feature(proc_macro_span)]
// The macro's & docs will be re-exported in `puzzle_runner`, and the links will work there.
#![allow(rustdoc::broken_intra_doc_links)]

mod examples;
mod scanner;
mod visual;

use proc_macro::TokenStream;

#[doc = include_str!("../docs/register_crate.md")]
#[proc_macro]
pub fn register_crate(input: TokenStream) -> TokenStream {
    scanner::register_crate(input)
}

#[doc = include_str!("../docs/register.md")]
#[proc_macro]
pub fn register(input: TokenStream) -> TokenStream {
    scanner::register(input)
}

#[doc = include_str!("../docs/example_input.md")]
#[proc_macro_attribute]
pub fn example_input(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    examples::example_input(input, annotated_item)
}

#[doc = include_str!("../docs/visual.md")]
#[proc_macro_attribute]
pub fn visual(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    visual::visual(input, annotated_item)
}

#[doc = include_str!("../docs/derive_to_renderable.md")]
#[proc_macro_derive(ToRenderable)]
pub fn derive_to_renderable(item: TokenStream) -> TokenStream {
    visual::derive_to_renderable(item)
}
