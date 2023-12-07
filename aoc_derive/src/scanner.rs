use std::fs::{self, read_to_string, DirEntry};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_file, parse_macro_input, parse_quote,
    punctuated::Punctuated,
    visit::{self, Visit},
    Expr, ExprPath, Ident, ItemFn, ItemStatic, Meta, PathSegment, Token,
};

use crate::examples;

struct ExampleScraper {
    mod_root_path: Punctuated<PathSegment, Token![::]>,
    current_path: Punctuated<PathSegment, Token![::]>,
    pub part1: Expr,
    pub part2: Expr,
    pub examples: Vec<Expr>,
}
impl ExampleScraper {
    pub fn new(path: ExprPath) -> Self {
        Self {
            mod_root_path: path.path.segments.clone(),
            current_path: path.path.segments,
            part1: parse_quote!(None),
            part2: parse_quote!(None),
            examples: Vec::new(),
        }
    }
}
impl<'ast> Visit<'ast> for ExampleScraper {
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        self.current_path.push(node.ident.clone().into());
        visit::visit_item_mod(self, node);
        self.current_path.pop();
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let cp = &self.current_path;
        if cp == &self.mod_root_path {
            match node.sig.ident.to_string().as_str() {
                "part1" => self.part1 = parse_quote!(Some(|i| #cp::part1(i).to_string())),
                "part2" => self.part2 = parse_quote!(Some(|i| #cp::part2(i).to_string())),
                _ => {}
            }
        }

        visit::visit_item_fn(self, node);
    }

    fn visit_item_static(&mut self, node: &'ast ItemStatic) {
        // Check if this item is an unexpanded example. If it is expand it now and feed it back into this parser.
        let example_annotation = node.attrs.iter().find_map(|attr| {
            let Meta::List(ref list) = attr.meta else {
                return None;
            };
            let is_example = list
                .path
                .get_ident()
                .map_or(false, |i| *i == "example_input");
            if is_example {
                Some(list)
            } else {
                None
            }
        });
        if let Some(annotation) = example_annotation {
            let mut node = node.clone();
            node.attrs.clear();
            let example: TokenStream = examples::example_input(
                annotation.tokens.clone().into(),
                node.into_token_stream().into(),
            );
            visit::visit_file(self, &syn::parse(example).unwrap());
            return;
        }

        // Check if this item in an expanded example.
        if node.ty == parse_quote!(aoc::derived::Example) {
            self.examples.push(*node.expr.clone());
        }

        visit::visit_item_static(self, node);
    }
}

fn scan_day(path: &str, name: &str, modpath: ExprPath) -> TokenStream2 {
    let mut scraper = ExampleScraper::new(modpath);
    let contents = read_to_string(path).unwrap();
    let file = parse_file(&contents).unwrap();
    scraper.visit_file(&file);

    let ExampleScraper {
        part1,
        part2,
        examples,
        ..
    } = scraper;

    quote! {
        aoc::derived::Day {
            name: #name,
            part1: #part1,
            part2: #part2,
            examples: vec![ #(#examples),* ],
        }
    }
}

pub fn scan_days(input: TokenStream) -> TokenStream {
    let ident: Ident = parse_macro_input!(input);

    let mut binmods: Vec<TokenStream2> = Vec::new();
    let mut days: Vec<TokenStream2> = Vec::new();

    let mut entries: Vec<DirEntry> = fs::read_dir("./src/bin")
        .unwrap()
        .map(Result::unwrap)
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let fname = entry.file_name().into_string().unwrap();
        if !fname.starts_with("day") || !fname.ends_with(".rs") {
            continue;
        }

        let modname = fname.replace(".rs", "");
        let modident = format_ident!("{}", modname);
        binmods.push(quote! {
            pub mod #modident;
        });

        days.push(scan_day(
            entry.path().to_str().unwrap(),
            &modname,
            parse_quote!(crate::bin::#modident),
        ))
    }

    quote! {
        mod bin {
            #(#binmods)*
        }
        static #ident: once_cell::sync::Lazy<Vec<aoc::derived::Day>> = once_cell::sync::Lazy::new(|| vec![ #(#days),* ]);
    }
    .into()
}
