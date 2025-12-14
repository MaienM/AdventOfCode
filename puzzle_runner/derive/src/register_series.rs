use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, parse_macro_input};

use crate::{
    include_chapters::include_chapters,
    utils::{ParseNestedMetaExt as _, args_struct, return_err, source_crate},
};

args_struct! {
    struct Args {
        /// The title of the series.
        title: String,
        /// The controller for the series.
        controller: Expr = default ::syn::parse_quote!(::puzzle_runner::controller::DefaultController),
    }
}

pub fn main(input: TokenStream) -> TokenStream {
    let mut builder = Args::build();
    let args_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("title") {
            meta.set_empty_option(&mut builder.title, meta.parse_stringify_nonempty()?)?;
        } else if meta.path.is_ident("controller") {
            meta.set_empty_option(&mut builder.controller, meta.value()?.parse()?)?;
        } else {
            return Err(meta.error("unsupported property"));
        }
        Ok(())
    });
    parse_macro_input!(input with args_parser);
    let Args { title, controller } = return_err!(builder.finalize());

    let chapters = include_chapters(false);
    let (name, alias) = if let Ok(name) = source_crate() {
        let crateident = format_ident!("{name}");
        (
            name,
            quote! {
                #[cfg(not(any(test, doctest)))]
                extern crate self as #crateident;
            },
        )
    } else {
        (String::new(), quote!())
    };

    quote! {
        #chapters

        pub static SERIES: ::std::sync::LazyLock<::puzzle_runner::derived::Series> = ::std::sync::LazyLock::new(|| {
            ::puzzle_runner::derived::Series {
                name: #name,
                title: #title,
                chapters: CHAPTERS.clone(),
                controller: ::std::sync::Arc::new(::std::boxed::Box::new(
                    <#controller as ::puzzle_runner::controller::Controller>::new().unwrap()
                )),
            }
        });

        // Make the current crate available under its external name (which is the name that must be
        // used when referring to it from the bins normally).
        #alias
    }
    .into()
}
