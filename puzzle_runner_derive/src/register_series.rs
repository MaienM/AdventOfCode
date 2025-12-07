use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

use crate::utils::{ParseNestedMetaExt as _, args_struct, return_err, source_crate};

args_struct! {
    struct Args {
        /// The title of the series.
        title: String,
    }
}

pub fn main(input: TokenStream) -> TokenStream {
    let mut builder = Args::build();
    let args_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("title") {
            meta.set_empty_option(&mut builder.title, meta.parse_nonempty_string()?)?;
        } else {
            return Err(meta.error("unsupported property"));
        }
        Ok(())
    });
    parse_macro_input!(input with args_parser);
    let Args { title } = return_err!(builder.finalize());

    #[cfg(feature = "include-chapters")]
    let (pre, chapters) = (
        quote!(::puzzle_runner::include_chapters!();),
        quote!(CHAPTERS.clone()),
    );
    #[cfg(not(feature = "include-chapters"))]
    let (pre, chapters) = (quote!(), quote!(Vec::new()));

    let name = return_err!(source_crate());
    let crateident = format_ident!("{}", name);

    quote! {
        #pre

        pub static SERIES: ::std::sync::LazyLock<::puzzle_runner::derived::Series> = ::std::sync::LazyLock::new(|| {
            ::puzzle_runner::derived::Series {
                name: #name,
                title: #title,
                chapters: #chapters,
            }
        });

        // Make the current crate available under its external name (which is the name that must be
        // used when referring to it from the bins normally).
        extern crate self as #crateident;
    }
    .into()
}
