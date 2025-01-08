use std::{env, fs::read_to_string, path::PathBuf};

use proc_macro::{Span, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use regex::Regex;
use syn::{
    parse_file, parse_macro_input, parse_quote,
    punctuated::Punctuated,
    visit::{self, Visit},
    Error, Expr, ExprPath, ItemFn, ItemMod, ItemStatic, Lit, Meta, PathSegment, Token,
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

struct BinScanner {
    mod_root_path: Punctuated<PathSegment, Token![::]>,
    mod_visual_path: Punctuated<PathSegment, Token![::]>,
    current_path: Punctuated<PathSegment, Token![::]>,
    pub(crate) path: String,
    pub(crate) name: String,
    pub(crate) title: Option<String>,
    pub(crate) part1: Expr,
    pub(crate) part2: Expr,
    pub(crate) visual1: Expr,
    pub(crate) visual2: Expr,
    pub(crate) examples: Vec<Expr>,
}
impl BinScanner {
    pub(crate) fn scan_file(path: &str) -> Self {
        let modpath: ExprPath = parse_quote!(self);
        let mut scanner = Self {
            path: path.to_owned(),
            name: path.split('/').last().unwrap().replace(".rs", ""),
            title: None,
            mod_root_path: modpath.path.segments.clone(),
            mod_visual_path: {
                let mut p = modpath.path.segments.clone();
                p.push(parse_quote!(does_not_exist));
                p
            },
            current_path: modpath.path.segments,
            part1: parse_quote!(::puzzle_runner::derived::Solver::NotImplemented),
            part2: parse_quote!(::puzzle_runner::derived::Solver::NotImplemented),
            visual1: parse_quote!(::puzzle_runner::derived::Solver::NotImplemented),
            visual2: parse_quote!(::puzzle_runner::derived::Solver::NotImplemented),
            examples: Vec::new(),
        };

        let contents = read_to_string(path).unwrap();
        let file = parse_file(&contents).unwrap();
        scanner.visit_file(&file);

        scanner
    }

    pub(crate) fn to_expr(&self) -> Expr {
        let BinScanner {
            name,
            part1,
            part2,
            visual1,
            visual2,
            examples,
            ..
        } = self;

        let (year, day): (u8, u8) = if name == "template" {
            (0, 0)
        } else {
            (name[0..2].parse().unwrap(), name[3..5].parse().unwrap())
        };

        let title: Expr = match &self.title {
            Some(title) => parse_quote!(Some(#title)),
            None => parse_quote!(None),
        };

        let root_path = env::current_dir()
            .map_err(|err| format!("error determining working directory: {err}"))
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        let path = &self
            .path
            .strip_prefix(&format!("{root_path}/"))
            .unwrap_or(&self.path);

        parse_quote! {
            ::puzzle_runner::derived::Bin {
                name: #name,
                title: #title,
                source_path: #path,
                year: #year,
                day: #day,
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
                || a.meta == Meta::Path(parse_quote!(puzzle_runner::visual))
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
                "part1" => {
                    self.part1 = parse_quote!(::puzzle_runner::derived::Solver::Implemented(|i| #cp::part1(i).to_string()));
                }
                "part2" => {
                    self.part2 = parse_quote!(::puzzle_runner::derived::Solver::Implemented(|i| #cp::part2(i).to_string()));
                }
                _ => {}
            }
        } else if cp == &self.mod_visual_path {
            match node.sig.ident.to_string().as_str() {
                "part1" => {
                    self.visual1 = parse_quote!(::puzzle_runner::derived::Solver::Implemented(|i| #cp::part1(i).into()));
                }
                "part2" => {
                    self.visual2 = parse_quote!(::puzzle_runner::derived::Solver::Implemented(|i| #cp::part2(i).into()));
                }
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
            let is_example = list.path.get_ident().is_some_and(|i| *i == "example_input");
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
        if node.ty == parse_quote!(::puzzle_runner::derived::Example) {
            self.examples.push(*node.expr.clone());
        }

        visit::visit_item_static(self, node);
    }
}

enum SourcePath {
    Ok(PathBuf),
    Empty,
    Error(String),
}
impl SourcePath {
    pub fn unwrap(self) -> Result<PathBuf, String> {
        match self {
            SourcePath::Ok(path) => Ok(path),
            SourcePath::Empty => Err("path of source file is empty".to_owned()),
            SourcePath::Error(err) => Err(err),
        }
    }
}

fn get_source_path() -> SourcePath {
    let file = {
        let mut span = Span::call_site();
        while let Some(parent) = span.parent() {
            span = parent;
        }
        span.source_file()
    };
    if file.is_real() {
        let path = file.path();
        if path.to_str() == Some("") {
            // This is likely a tool such as rust_analyzer, which provides an call site with an empty path.
            SourcePath::Empty
        } else {
            SourcePath::Ok(path)
        }
    } else {
        SourcePath::Error("unable to determine path of source file".to_owned())
    }
}

fn find_binaries() -> Result<Vec<String>, String> {
    let path = "bin".to_owned();
    let filename_regex = Regex::new(r"^\d{2}-\d{2}\.rs$").unwrap();
    let source_path = get_source_path().unwrap()?;
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
    let mut files = Vec::new();
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
        if !filename_regex.is_match(&fname) {
            continue;
        }
        files.push(
            entry
                .path()
                .to_str()
                .ok_or(format!("error getting path for {entry:?}"))?
                .to_owned(),
        );
    }
    files.sort_unstable();
    Ok(files)
}

pub fn register_crate(input: TokenStream) -> TokenStream {
    let args_parser = syn::meta::parser(|meta| Err(meta.error("unsupported property")));
    parse_macro_input!(input with args_parser);

    let mut mods: Vec<TokenStream2> = Vec::new();
    let mut bins: Vec<TokenStream2> = Vec::new();

    let files = return_err!(find_binaries(), Span::call_site().into());
    for file in files {
        let modident = file
            .split('/')
            .last()
            .unwrap()
            .replace('-', "_")
            .strip_suffix(".rs")
            .unwrap()
            .to_owned();
        let modident = format_ident!("_{}", modident);
        bins.push(quote!(#modident::BIN.clone()));
        mods.push(quote! {
            #[path = #file]
            pub mod #modident;
        });
    }

    quote! {
        pub mod bins {
            // Store list of binaries in a static. This is used in the main methods below, but it's
            // also imported by the WASM create.
            pub static BINS: ::once_cell::sync::Lazy<Vec<::puzzle_runner::derived::Bin>> = ::once_cell::sync::Lazy::new(|| {
                let bins: Vec<::puzzle_runner::derived::Bin> = vec![ #(#bins),* ];

                let mut seen = ::std::collections::HashMap::new();
                for bin in &bins {
                    let Some(title) = bin.title else { continue };

                    if let Some(other_bin) = seen.insert(title, bin.name) {
                        panic!(
                            "Binary {} and {} both have title '{title}', this is not valid.",
                            other_bin, bin.name
                        );
                    }
                }

                bins
            });

            #(#mods)*

            pub fn multi() {
                puzzle_runner::multi::BINS.get_or_init(|| Box::new(BINS.clone()));
                puzzle_runner::multi::main();
            }

            #[cfg(feature = "bench")]
            pub fn bench() {
                puzzle_runner::multi::BINS.get_or_init(|| Box::new(BINS.clone()));
                puzzle_runner::bench::main();
            }
        }
    }
    .into()
}

pub fn register(input: TokenStream) -> TokenStream {
    let expr = match get_source_path() {
        SourcePath::Ok(path) => {
            let mut scanner = BinScanner::scan_file(path.to_str().unwrap());

            let args_parser = syn::meta::parser(|meta| {
                if meta.path.is_ident("title") {
                    match meta.value()?.parse::<Lit>()? {
                        Lit::Str(title) => {
                            if title.value().is_empty() {
                                return Err(meta.error("cannot be empty"));
                            }
                            scanner.title = Some(title.value());
                        }
                        _ => return Err(meta.error("unsupported value, must be a string")),
                    }
                } else {
                    return Err(meta.error("unsupported property"));
                }
                Ok(())
            });
            parse_macro_input!(input with args_parser);

            scanner.to_expr()
        }
        SourcePath::Empty => {
            parse_quote! { ::core::todo!() }
        }
        SourcePath::Error(err) => {
            return Error::new(Span::call_site().into(), err)
                .to_compile_error()
                .into();
        }
    };

    quote!{
        // Store metadata in a static. This is used in the main method below, but it's also copied
        // to the full list used by the multi entrypoint.
        pub(crate) static BIN: ::once_cell::sync::Lazy<::puzzle_runner::derived::Bin> = ::once_cell::sync::Lazy::new(|| #expr );

        // Generate entrypoint that just runs this day.
        pub fn main() {
            ::puzzle_runner::single::main(&*BIN);
        }
    }
    .into_token_stream()
    .into()
}
