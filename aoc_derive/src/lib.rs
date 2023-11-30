use std::fs::{self, DirEntry};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Item};

extern crate proc_macro;

fn has_part_2(path: &str) -> bool {
    let contents = fs::read_to_string(path).unwrap();
    let file = syn::parse_file(&contents).unwrap();
    for item in &file.items {
        match item {
            Item::Fn(itemfn) => match itemfn.sig.ident.to_string().as_str() {
                "part2" => {
                    return true;
                }
                _ => {}
            },
            _ => {}
        }
    }
    return false;
}

#[proc_macro_derive(RunnableListProvider)]
pub fn part_finder_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let mut uses: Vec<TokenStream2> = Vec::new();
    let mut runnables: Vec<TokenStream2> = Vec::new();

    let mut entries: Vec<DirEntry> = fs::read_dir("./src/bin")
        .unwrap()
        .into_iter()
        .map(Result::unwrap)
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let fname = entry.file_name().into_string().unwrap();
        if !fname.starts_with("day") || !fname.ends_with(".rs") {
            continue; // Skip files that don't look like bins for days.
        }

        let modname = fname.replace(".rs", "");
        let modident = format_ident!("{}", modname);
        let has_part_2 = has_part_2(entry.path().to_str().unwrap());

        uses.push(quote! {
            pub mod #modident;
        });

        let part1ident = quote! { Some(|i| crate::bin::#modident::part1(&i).to_string()) };
        let part2ident = if has_part_2 {
            quote! { Some(|i| crate::bin::#modident::part2(&i).to_string()) }
        } else {
            quote! { None  }
        };
        runnables.push(quote! { (#modname, #part1ident, #part2ident) });
    }

    let output = quote! {
        mod bin {
            #![allow(dead_code)]
            #(#uses)*
        }
        impl RunnableListProvider for #ident {
            fn get() -> RunnableList {
                return vec![
                    #(#runnables),*
                ];
            }
        }
    };
    return output.into();
}
