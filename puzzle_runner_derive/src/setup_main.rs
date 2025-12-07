use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

use crate::{
    include_chapters::include_chapters,
    utils::{return_err, source_crate},
};

pub fn main(input: TokenStream) -> TokenStream {
    let args_parser = syn::meta::parser(|meta| Err(meta.error("unsupported property")));
    parse_macro_input!(input with args_parser);

    let chapters = include_chapters(true);
    let crateident = format_ident!("{}", return_err!(source_crate()));

    quote! {
        #chapters

        pub fn main() {
            let series = ::puzzle_runner::derived::Series {
                chapters: CHAPTERS.clone(),
                ..*::#crateident::SERIES
            };
            ::puzzle_runner::multi::main(&series);
        }
    }
    .into()
}
