use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, ItemFn, Visibility, parse_macro_input};

use crate::utils::{ParseNestedMetaExt as _, args_struct, get_series_and_controller, return_err};

args_struct! {
    struct Args {
        /// The additional argument to pass in when processing the real input.
        arg: Option<Expr> = initial None,
    }
}

pub fn main(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let mut builder = Args::build();
    let args_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("arg") {
            meta.set_empty_option(&mut builder.arg, meta.value()?.parse::<Expr>()?)?;
        } else {
            return Err(meta.error("unsupported property"));
        }
        Ok(())
    });
    parse_macro_input!(input with args_parser);
    let Args { arg } = return_err!(builder.finalize());

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

    let (_, controller) = get_series_and_controller();
    let const_ident = format_ident!("PART{num}");

    let arg = match arg {
        Some(arg) => quote!(, #arg),
        None => quote!(),
    };

    quote! {
        static #const_ident: ::std::sync::LazyLock<::puzzle_runner::derived::Part> = ::std::sync::LazyLock::new(|| {
            let mut builder = ::puzzle_runner::derived::PartBuilder::default();
            builder.num(#num);
            builder.implementation(|input| (#ident(input #arg)).to_string());
            #controller.process_part(&mut builder).unwrap();
            builder.build().unwrap()
        });

        #part
    }
    .into()
}
