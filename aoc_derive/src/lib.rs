use std::{
    collections::HashMap,
    fs::{self, DirEntry},
};

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, DeriveInput, File, Item, ItemExternCrate, ItemFn, Token, VisPublic,
    Visibility,
};

extern crate proc_macro;

fn make_public(itemfn: &mut ItemFn) {
    itemfn.vis = Visibility::Public(VisPublic {
        pub_token: Token![pub](Span::call_site()),
    });
}

struct BinMod {
    file: File,
    has_part_2: bool,
    externs: Vec<ItemExternCrate>,
}

fn load_bin_as_mod(path: &str) -> BinMod {
    let contents = fs::read_to_string(path).unwrap();
    let mut file = syn::parse_file(&contents).unwrap();

    let mut has_part_2 = false;
    let mut externs: Vec<ItemExternCrate> = Vec::new();
    for item in &mut file.items {
        match item {
            Item::Fn(ref mut itemfn) => match itemfn.sig.ident.to_string().as_str() {
                "part1" => {
                    make_public(itemfn);
                }
                "part2" => {
                    make_public(itemfn);
                    has_part_2 = true;
                }
                _ => {}
            },
            Item::ExternCrate(ref crate_) => {
                externs.push(crate_.clone());
                let stream = TokenStream2::new();
                *item = Item::Verbatim(stream);
            }
            _ => {}
        }
    }

    return BinMod {
        file,
        has_part_2,
        externs,
    };
}

#[proc_macro_derive(RunnableListProvider)]
pub fn part_finder_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let mut uses: Vec<TokenStream2> = Vec::new();
    let mut externs: HashMap<String, ItemExternCrate> = HashMap::new();
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

        let binmod = load_bin_as_mod(entry.path().to_str().unwrap());
        let modfile = binmod.file;

        uses.push(quote! {
            mod #modident {
                #modfile
            }
        });

        for ex in binmod.externs {
            externs.insert(ex.ident.to_string(), ex);
        }

        let part1ident = quote! { |i| #modident::part1(i).to_string() };
        let part2ident = if binmod.has_part_2 {
            quote! { |i| #modident::part2(i).to_string() }
        } else {
            quote! { missing::<String>  }
        };
        runnables.push(quote! { (#modname, #part1ident, #part2ident) });
    }

    let externs = externs.values();
    let output = quote! {
        use aoc::runner::{missing, RunnableList};
        #(#uses)*
        #(#externs)*
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
