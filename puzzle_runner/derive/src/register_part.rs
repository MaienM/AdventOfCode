use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, ItemFn, Visibility, parse_macro_input};

use crate::utils::return_err;

pub fn main(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let args_parser = syn::meta::parser(|meta| Err(meta.error("unsupported property")));
    parse_macro_input!(input with args_parser);

    let part = parse_macro_input!(annotated_item as ItemFn);
    let ident = part.sig.ident.clone();
    let num = return_err!(
        ident
            .to_string()
            .strip_prefix("part")
            .ok_or_else(
                || "invalid name, must be in format partN, where N is an integer > 0".to_owned()
            )
            .and_then(|v| v.parse::<u8>().map_err(|e| e.to_string()))
    );
    if part.vis != Visibility::Inherited {
        return_err!(Err("should be private"));
    }

    let const_ident = format_ident!("PART{num}");

    quote! {
        static #const_ident: ::puzzle_runner::derived::Part = ::puzzle_runner::derived::Part {
            num: #num,
            implementation: |input| (#ident(input)).to_string(),
        };

        #part
    }
    .into()
}
