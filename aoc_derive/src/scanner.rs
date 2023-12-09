use std::fs::{self, read_to_string};

use proc_macro::{Span, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_file, parse_macro_input, parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    visit::{self, Visit},
    Error, Expr, ExprPath, ForeignItemStatic, ItemFn, ItemStatic, LitStr, Meta, PathSegment, Token,
    Type,
};
use tap::prelude::*;

use crate::examples;

struct BinScanner {
    mod_root_path: Punctuated<PathSegment, Token![::]>,
    current_path: Punctuated<PathSegment, Token![::]>,
    pub(crate) name: String,
    pub(crate) part1: Option<Expr>,
    pub(crate) part2: Option<Expr>,
    pub(crate) examples: Vec<Expr>,
}
impl BinScanner {
    pub(crate) fn scan_file(path: &str, modpath: ExprPath) -> Self {
        let mut scanner = Self {
            name: path.split('/').last().unwrap().replace(".rs", ""),
            mod_root_path: modpath.path.segments.clone(),
            current_path: modpath.path.segments,
            part1: None,
            part2: None,
            examples: Vec::new(),
        };

        let contents = read_to_string(path).unwrap();
        let file = parse_file(&contents).unwrap();
        scanner.visit_file(&file);

        scanner
    }

    pub(crate) fn to_expr(&self) -> Expr {
        let BinScanner { name, examples, .. } = self;

        let num: u8 = name[3..].parse().unwrap();
        let part1: Expr = self.part1.as_ref().map_or(
            parse_quote!(::aoc::runner::Solver::NotImplemented),
            |f| parse_quote!(::aoc::runner::Solver::Implemented(#f)),
        );
        let part2: Expr = self.part2.as_ref().map_or(
            parse_quote!(::aoc::runner::Solver::NotImplemented),
            |f| parse_quote!(::aoc::runner::Solver::Implemented(#f)),
        );

        parse_quote! {
            ::aoc::derived::Day {
                name: #name,
                num: #num,
                part1: #part1,
                part2: #part2,
                examples: vec![ #(#examples),* ],
            }
        }
    }
}
impl<'ast> Visit<'ast> for BinScanner {
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        self.current_path.push(node.ident.clone().into());
        visit::visit_item_mod(self, node);
        self.current_path.pop();
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let cp = &self.current_path;
        if cp == &self.mod_root_path {
            match node.sig.ident.to_string().as_str() {
                "part1" => self.part1 = Some(parse_quote!(|i| #cp::part1(i).to_string())),
                "part2" => self.part2 = Some(parse_quote!(|i| #cp::part2(i).to_string())),
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

        // Check if this item is an expanded example.
        if node.ty == parse_quote!(::aoc::derived::Example) {
            self.examples.push(*node.expr.clone());
        }

        visit::visit_item_static(self, node);
    }
}

fn fill_static(def: ForeignItemStatic, ty: Type, expr: Expr) -> ItemStatic {
    ItemStatic {
        attrs: def.attrs,
        vis: def.vis,
        static_token: def.static_token,
        mutability: def.mutability,
        ident: def.ident,
        colon_token: def.colon_token,
        semi_token: def.semi_token,
        eq_token: parse_quote!(=),
        ty: Box::new(ty),
        expr: Box::new(expr),
    }
}

pub fn inject_days(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let mut path = ".".to_owned();
    let args_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("path") {
            path = meta.value()?.parse::<LitStr>()?.value();
        } else {
            return Err(meta.error("unsupported property"));
        }
        Ok(())
    });
    parse_macro_input!(input with args_parser);

    let itemdef = parse_macro_input!(annotated_item as ForeignItemStatic);
    if itemdef.ty != parse_quote!(Vec<Day>) {
        return Error::new(itemdef.ty.span(), "must be of type Vec<Day>".to_owned())
            .to_compile_error()
            .into();
    }

    let mut binmods: Vec<TokenStream2> = Vec::new();
    let mut dayexprs: Vec<TokenStream2> = Vec::new();

    let scanners: Vec<BinScanner> = fs::read_dir("./src/bin")
        .unwrap()
        .map(Result::unwrap)
        .collect::<Vec<_>>()
        .tap_mut(|list| list.sort_by_key(|e| e.file_name()))
        .into_iter()
        .filter_map(|entry| {
            let fname = entry.file_name().into_string().unwrap();
            if !fname.starts_with("day") || !fname.ends_with(".rs") {
                return None;
            }

            let modident = format_ident!("{}", fname.replace(".rs", ""));
            Some(BinScanner::scan_file(
                entry.path().to_str().unwrap(),
                parse_quote!(bin::#modident),
            ))
        })
        .collect();

    for scanner in scanners {
        let modident = format_ident!("{}", scanner.name);
        binmods.push(quote! {
            pub mod #modident;
        });
        dayexprs.push(scanner.to_expr().into_token_stream());
    }

    let itemdef = fill_static(
        itemdef,
        parse_quote!(once_cell::sync::Lazy<Vec<::aoc::derived::Day>>),
        parse_quote!(once_cell::sync::Lazy::new(|| vec![ #(#dayexprs),* ])),
    );

    quote! {
        #itemdef

        #[path = #path]
        #[allow(dead_code)]
        #[allow(unused_imports)]
        #[allow(unused_variables)]
        mod bin {
            #(#binmods)*
        }
    }
    .into()
}

pub fn inject_day(_input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let itemdef = parse_macro_input!(annotated_item as ForeignItemStatic);
    if itemdef.ty != parse_quote!(Day) {
        return Error::new(itemdef.ty.span(), "must be of type Day".to_owned())
            .to_compile_error()
            .into();
    }

    let file = {
        let mut span = Span::call_site();
        while let Some(parent) = span.parent() {
            span = parent;
        }
        span.source_file()
    };
    if !file.is_real() {
        return Error::new(
            itemdef.ty.span(),
            "unable to determine path of source file".to_owned(),
        )
        .to_compile_error()
        .into();
    }

    let scanner = BinScanner::scan_file(file.path().to_str().unwrap(), parse_quote!(self));
    let expr = scanner.to_expr();

    fill_static(
        itemdef,
        parse_quote!(once_cell::sync::Lazy<::aoc::derived::Day>),
        parse_quote!(once_cell::sync::Lazy::new(|| #expr )),
    )
    .into_token_stream()
    .into()
}
