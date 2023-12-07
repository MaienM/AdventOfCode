use std::fs::{self, DirEntry};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Item, Path};

extern crate proc_macro;

fn has_part_2(path: &str) -> bool {
    let contents = fs::read_to_string(path).unwrap();
    let file = syn::parse_file(&contents).unwrap();
    for item in &file.items {
        if let Item::Fn(itemfn) = item {
            if itemfn.sig.ident.to_string().as_str() == "part2" {
                return true;
            }
        }
    }
    false
}

pub fn get_runnables(input: TokenStream) -> TokenStream {
    let ident: Path = parse_macro_input!(input);

    let mut binmods: Vec<TokenStream2> = Vec::new();
    let mut runnables: Vec<TokenStream2> = Vec::new();

    let mut entries: Vec<DirEntry> = fs::read_dir("./src/bin")
        .unwrap()
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

        binmods.push(quote! {
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

    quote! {
        #[allow(dead_code)]
        mod bin {
            #(#binmods)*
        }

        static #ident: once_cell::sync::Lazy<Vec<(
            &'static str,
            aoc::runner::Runnable<String>,
            aoc::runner::Runnable<String>,
        )>> = once_cell::sync::Lazy::new(|| vec![ #(#runnables),* ]);
    }
    .into()
}
