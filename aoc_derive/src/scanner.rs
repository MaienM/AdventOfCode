use std::{env, fs::read_to_string, path::PathBuf};

use proc_macro::{Span, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_file, parse_macro_input, parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    visit::{self, Visit},
    Error, Expr, ExprPath, ForeignItemStatic, ItemFn, ItemMod, ItemStatic, LitStr, Meta,
    PathSegment, Token, Type,
};

use crate::examples;

macro_rules! return_err {
    ($value:expr, $span:expr) => {
        match $value {
            Ok(value) => value,
            Err(err) => {
                return Error::new($span, err).to_compile_error().into();
            }
        }
    };
}

fn optional_expr(expr: &Option<Expr>) -> Expr {
    expr.as_ref()
        .map_or(parse_quote!(None), |f| parse_quote!(Some(#f)))
}

struct BinScanner {
    mod_root_path: Punctuated<PathSegment, Token![::]>,
    mod_visual_path: Punctuated<PathSegment, Token![::]>,
    current_path: Punctuated<PathSegment, Token![::]>,
    pub(crate) name: String,
    pub(crate) part1: Option<Expr>,
    pub(crate) part2: Option<Expr>,
    pub(crate) visual1: Option<Expr>,
    pub(crate) visual2: Option<Expr>,
    pub(crate) examples: Vec<Expr>,
}
impl BinScanner {
    pub(crate) fn scan_file(path: &str, modpath: ExprPath) -> Self {
        let mut scanner = Self {
            name: path.split('/').last().unwrap().replace(".rs", ""),
            mod_root_path: modpath.path.segments.clone(),
            mod_visual_path: {
                let mut p = modpath.path.segments.clone();
                p.push(parse_quote!(does_not_exist));
                p
            },
            current_path: modpath.path.segments,
            part1: None,
            part2: None,
            visual1: None,
            visual2: None,
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
        let part1 = optional_expr(&self.part1);
        let part2 = optional_expr(&self.part2);
        let visual1 = optional_expr(&self.visual1);
        let visual2 = optional_expr(&self.visual2);

        parse_quote! {
            ::aoc::derived::Day {
                name: #name,
                num: #num,
                part1: #part1,
                part2: #part2,
                #[cfg(feature = "visual")]
                visual1: #visual1,
                #[cfg(feature = "visual")]
                visual2: #visual2,
                examples: vec![ #(#examples),* ],
            }
        }
    }
}
impl<'ast> Visit<'ast> for BinScanner {
    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        self.current_path.push(node.ident.clone().into());

        if node.attrs.iter().any(|a| {
            a.meta == Meta::Path(parse_quote!(visual))
                || a.meta == Meta::Path(parse_quote!(aoc_derive::visual))
        }) {
            self.mod_visual_path = self.current_path.clone();
        }

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
        } else if cp == &self.mod_visual_path {
            match node.sig.ident.to_string().as_str() {
                "part1" => self.visual1 = Some(parse_quote!(|i| #cp::part1(i).into())),
                "part2" => self.visual2 = Some(parse_quote!(|i| #cp::part2(i).into())),
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

fn get_source_path() -> Result<PathBuf, String> {
    let file = {
        let mut span = Span::call_site();
        while let Some(parent) = span.parent() {
            span = parent;
        }
        span.source_file()
    };
    if file.is_real() {
        Ok(file.path())
    } else {
        Err("unable to determine path of source file".to_owned())
    }
}

fn scan_days(path: String) -> Result<Vec<BinScanner>, String> {
    let source_path = get_source_path()?;
    let abs_path = env::current_dir()
        .map_err(|err| format!("error determining working directory: {err}"))?
        .join(source_path.clone())
        .parent()
        .ok_or(format!(
            "failed to determine parent of source file {source_path:?}"
        ))?
        .join(path.clone())
        .canonicalize()
        .map_err(|err| format!("error resolving {path:?}: {err}"))?;
    let mut scanners = Vec::new();
    let dir = abs_path.read_dir().map_err(|err| {
        format!("error listing files in {path:?} (resolved to {abs_path:?}): {err}")
    })?;
    for entry in dir {
        let entry = entry.map_err(|err| {
            format!("error listing files in {path:?} (resolved to {abs_path:?}): {err}")
        })?;
        let fname = entry.file_name().into_string().map_err(|err| {
            let err = err.into_string().unwrap();
            format!("error getting filename for {entry:?}: {err}")
        })?;
        if !fname.starts_with("day") || !fname.ends_with(".rs") {
            continue;
        }

        let modident = format_ident!("{}", fname.replace(".rs", ""));
        scanners.push(BinScanner::scan_file(
            entry
                .path()
                .to_str()
                .ok_or(format!("error getting path for {entry:?}"))?,
            parse_quote!(bin::#modident),
        ));
    }
    scanners.sort_by_key(|s| s.name.clone());
    Ok(scanners)
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

    let scanners = return_err!(scan_days(path.clone()), itemdef.ty.span());
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

    let path = return_err!(get_source_path(), itemdef.ty.span());
    let scanner = BinScanner::scan_file(path.to_str().unwrap(), parse_quote!(self));
    let expr = scanner.to_expr();

    fill_static(
        itemdef,
        parse_quote!(once_cell::sync::Lazy<::aoc::derived::Day>),
        parse_quote!(once_cell::sync::Lazy::new(|| #expr )),
    )
    .into_token_stream()
    .into()
}
