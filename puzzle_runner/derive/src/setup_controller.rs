use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

use crate::utils::{find_crate_root, return_err, source_call_site, source_crate};

pub fn main(input: TokenStream) -> TokenStream {
    let args_parser = syn::meta::parser(|meta| Err(meta.error("unsupported property")));
    parse_macro_input!(input with args_parser);

    if let Some(path) = source_call_site().local_file()
        && let Ok(path) = path.canonicalize()
        && let Ok(root) = find_crate_root(&path)
        && path != root.join("src/bin/controller.rs")
    {
        return_err!(Err("must be used in src/bin/controller.rs"));
    }

    if let Ok(name) = source_crate() {
        let crateident = format_ident!("{name}");
        quote! {
            pub fn main() {
                ::puzzle_runner::__internal::controller::main(&#crateident::SERIES);
            }
        }
        .into()
    } else {
        quote!().into()
    }
}
