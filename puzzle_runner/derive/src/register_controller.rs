use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemStruct, parse_macro_input};

use crate::utils::source_crate;

pub fn main(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let args_parser = syn::meta::parser(|meta| Err(meta.error("unsupported property")));
    parse_macro_input!(input with args_parser);

    let item = parse_macro_input!(annotated_item as ItemStruct);
    let ident = &item.ident;

    let main = if let Ok(name) = source_crate() {
        let crateident = format_ident!("{name}");
        quote! {
            pub fn main() {
                let series = ::puzzle_runner::derived::Series {
                    controller: ::std::sync::Arc::new(::std::boxed::Box::new(#ident::new().unwrap())),
                    ..#crateident::SERIES.clone()
                };
                ::puzzle_runner::__internal::controller::main(&series);
            }
        }
    } else {
        quote!()
    };

    quote! {
        #item
        #main
    }
    .into()
}
