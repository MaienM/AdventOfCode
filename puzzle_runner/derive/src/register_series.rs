use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, Ident, Type, parse_macro_input};

use crate::{
    include_chapters::include_chapters,
    utils::{ParseNestedMetaExt as _, args_struct, return_err, source_crate},
};

args_struct! {
    struct Args {
        /// The controller for the series.
        controller: Type,
        /// Metadata to pass directly to the builder.
        metadata: Map<Ident, Expr>,
    }
}

pub fn main(input: TokenStream) -> TokenStream {
    let mut builder = Args::build();
    let args_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("controller") {
            meta.map_err(builder.controller(meta.value()?.parse()?))?;
        } else if let Some(key) = meta.path.get_ident() {
            meta.map_err(builder.metadata_insert(key.clone(), meta.value()?.parse()?))?;
        } else {
            return Err(meta.error("unsupported property"));
        }
        Ok(())
    });
    parse_macro_input!(input with args_parser);
    let Args {
        controller,
        metadata,
    } = return_err!(builder.finalize());

    let mut prefix = include_chapters(false);

    let name = if let Ok(name) = source_crate() {
        let crateident = format_ident!("{name}");
        prefix = quote! {
            #prefix

            // Make the current crate available under its external name (which is the name that must be
            // used when referring to it from the bins normally).
            #[cfg(not(any(test, doctest)))]
            extern crate self as #crateident;
        };
        name
    } else {
        String::new()
    };

    let metadata_expressions = metadata.into_iter().map(|(k, v)| quote!(builder.#k(#v);));

    quote! {
        #prefix

        pub static CONTROLLER: ::std::sync::LazyLock<::std::sync::Arc<::std::boxed::Box<dyn ::puzzle_runner::controller::Controller>>> = ::std::sync::LazyLock::new(|| {
            ::std::sync::Arc::new(::std::boxed::Box::new(
                <#controller as ::puzzle_runner::controller::Controller>::new().unwrap()
            ))
        });

        pub static SERIES: ::std::sync::LazyLock<::puzzle_runner::derived::Series> = ::std::sync::LazyLock::new(|| {
            let mut builder = ::puzzle_runner::derived::SeriesBuilder::default();
            builder.name(#name);
            #(#metadata_expressions)*
            builder.chapters(CHAPTERS.clone());
            builder.controller(CONTROLLER.clone());
            CONTROLLER.process_series(&mut builder).unwrap();
            builder.build().unwrap()
        });
    }
    .into()
}
