use std::{env, fs::read_to_string};

use proc_macro::{Span, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use regex::Regex;
use syn::{
    Error, Expr, ExprCall, ExprClosure, ExprPath, Item, ItemFn, ItemMod, ItemStatic, Meta,
    PathSegment, Token, parse_file, parse_macro_input, parse_quote,
    punctuated::Punctuated,
    visit::{self, Visit},
};

use crate::{
    examples,
    utils::{ParseNestedMetaExt, args_struct, return_err},
};

args_struct! {
    struct Args {
        /// The book that the chapter is in.
        book: Option<String> = None,
        /// The title of the chapter.
        title: Option<String> = None,
    }
}

#[derive(Debug, Eq, PartialEq)]
enum FirstItem {
    Unknown,
    Register,
    Other,
}

struct ChapterScanner {
    mod_root_path: Punctuated<PathSegment, Token![::]>,
    current_path: Punctuated<PathSegment, Token![::]>,
    pub(crate) first_item: FirstItem,
    pub(crate) source_path: String,
    pub(crate) name: String,
    pub(crate) parts: Vec<Expr>,
    pub(crate) examples: Vec<Expr>,
}
impl ChapterScanner {
    pub(crate) fn scan_file(path: &str) -> Self {
        let modpath: ExprPath = parse_quote!(self);
        let mut scanner = Self {
            mod_root_path: modpath.path.segments.clone(),
            current_path: modpath.path.segments,
            first_item: FirstItem::Unknown,
            source_path: path.to_owned(),
            name: path.split('/').next_back().unwrap().replace(".rs", ""),
            parts: Vec::new(),
            examples: Vec::new(),
        };

        let contents = read_to_string(path).unwrap();
        let file = parse_file(&contents).unwrap();
        scanner.visit_file(&file);

        scanner.first_item = match file.items.first() {
            Some(Item::Macro(item))
                if item.mac.path == parse_quote!(puzzle_runner::register_chapter) =>
            {
                FirstItem::Register
            }
            Some(_) => FirstItem::Other,
            None => FirstItem::Unknown,
        };

        scanner
    }

    pub(crate) fn to_expr(&self, args: Args) -> Expr {
        let ChapterScanner {
            name,
            parts,
            examples,
            ..
        } = self;
        let Args { book, title } = args;

        let book: Expr = match book {
            Some(book) => parse_quote!(Some(#book)),
            None => parse_quote!(None),
        };

        let title: Expr = match title {
            Some(title) => parse_quote!(Some(#title)),
            None => parse_quote!(None),
        };

        let root_path = env::current_dir()
            .map_err(|err| format!("error determining working directory: {err}"))
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        let source_path = &self
            .source_path
            .strip_prefix(&format!("{root_path}/"))
            .unwrap_or(&self.source_path);

        parse_quote! {
            ::puzzle_runner::derived::Chapter {
                name: #name,
                book: #book,
                title: #title,
                source_path: #source_path,
                parts: vec![ #(#parts),* ],
                examples: vec![ #(#examples),* ],
            }
        }
    }
}
impl<'ast> Visit<'ast> for ChapterScanner {
    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        self.current_path.push(node.ident.clone().into());

        visit::visit_item_mod(self, node);
        self.current_path.pop();
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let cp = &self.current_path;
        if cp == &self.mod_root_path {
            let ident = &node.sig.ident;
            let name = ident.to_string();
            if name.starts_with("part")
                && let Ok(num) = name[4..].parse::<u8>()
            {
                self.parts.push(parse_quote! {
                    ::puzzle_runner::derived::Part {
                        num: #num,
                        implementation: |i| #cp::#ident(i).to_string(),
                    }
                });
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
            if is_example { Some(list) } else { None }
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
        if node.ty == parse_quote!(::std::sync::LazyLock<::puzzle_runner::derived::Example>)
            && let Expr::Call(ExprCall { func, args, .. }) = &*node.expr
            && let Expr::Path(ExprPath { path, .. }) = &**func
            && path == &parse_quote!(::std::sync::LazyLock::new)
            && let Some(Expr::Closure(ExprClosure { body, .. })) = args.first()
        {
            self.examples.push(*body.clone());
        }

        visit::visit_item_static(self, node);
    }
}

pub fn register_chapter(input: TokenStream) -> TokenStream {
    let mut builder = Args::build();
    let args_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("book") {
            meta.set_empty_option(&mut builder.book, Some(meta.parse_nonempty_string()?))?;
        } else if meta.path.is_ident("title") {
            meta.set_empty_option(&mut builder.title, Some(meta.parse_nonempty_string()?))?;
        } else {
            return Err(meta.error("unsupported property"));
        }
        Ok(())
    });
    parse_macro_input!(input with args_parser);
    let args = return_err!(builder.finalize());

    let expr = match Span::call_site().local_file() {
        Some(path) => {
            let scanner = ChapterScanner::scan_file(path.to_str().unwrap());
            if scanner.first_item != FirstItem::Register {
                return Error::new(
                    Span::call_site().into(),
                    "setup must be the first statement in the file",
                )
                .to_compile_error()
                .into();
            }
            scanner.to_expr(args)
        }
        None => {
            parse_quote! { ::core::todo!() }
        }
    };

    quote!{
        // Include prelude.
        #[allow(unused_imports)]
        use puzzle_lib::prelude::*;

        // Store metadata in a static. This is used in the main method below, but it's also copied
        // to the full list used by the multi entrypoint.
        pub(crate) static CHAPTER: ::std::sync::LazyLock<::puzzle_runner::derived::Chapter> = ::std::sync::LazyLock::new(|| #expr );

        // Generate entrypoint that just runs this chapter.
        pub fn main() {
            ::puzzle_runner::single::main(&*CHAPTER);
        }
    }
    .into_token_stream()
    .into()
}

fn find_chapters() -> Result<Vec<String>, String> {
    let mut span = Span::call_site();
    while let Some(parent) = span.parent() {
        span = parent;
    }
    let source_path = span.local_file().ok_or("path of source file is empty")?;

    let mut crate_root = env::current_dir()
        .map_err(|err| format!("error determining working directory: {err}"))?
        .join(source_path.clone());
    loop {
        if !crate_root.pop() {
            Err(format!(
                "failed to traverse up from {}",
                crate_root.display(),
            ))?;
        }
        crate_root = crate_root
            .canonicalize()
            .map_err(|err| format!("failed to resolve {}: {err}", crate_root.display()))?;
        match crate_root.join("Cargo.toml").try_exists() {
            Ok(true) => break,
            Ok(false) => {}
            Err(err) => Err(format!(
                "failed to find root of crate for source file {}: {err}",
                source_path.display(),
            ))?,
        }
    }
    let abs_path = crate_root.join("src").join("bin");

    let filename_regex = Regex::new(r"^\d{2}-\d{2}\.rs$").unwrap();
    let mut files = Vec::new();
    let dir = abs_path.read_dir().map_err(|err| {
        format!(
            "error listing crate binaries (resolved to {}): {err}",
            abs_path.display()
        )
    })?;
    for entry in dir {
        let entry = entry.map_err(|err| {
            format!(
                "error listing crate binaries (resolved to {}): {err}",
                abs_path.display()
            )
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

pub fn include_chapters(input: TokenStream) -> TokenStream {
    let args_parser = syn::meta::parser(|meta| Err(meta.error("unsupported property")));
    parse_macro_input!(input with args_parser);

    let mut mods: Vec<TokenStream2> = Vec::new();
    let mut chapters: Vec<TokenStream2> = Vec::new();

    let files = return_err!(find_chapters());
    for file in files {
        let modident = file
            .split('/')
            .next_back()
            .unwrap()
            .replace('-', "_")
            .strip_suffix(".rs")
            .unwrap()
            .to_owned();
        let modident = format_ident!("_{}", modident);
        chapters.push(quote!(crate::chapters::#modident::CHAPTER.clone()));
        mods.push(quote! {
            #[path = #file]
            pub mod #modident;
        });
    }

    quote! {
        pub static CHAPTERS: ::std::sync::LazyLock<Vec<::puzzle_runner::derived::Chapter>> = ::std::sync::LazyLock::new(|| {
            // Get the chapters.
            let chapters: Vec<::puzzle_runner::derived::Chapter> = {
                #[cfg(test)]
                { Vec::new() }
                #[cfg(not(test))]
                { vec![ #(#chapters),* ] }
            };

            // Validate that the titles are unique.
            let mut seen = ::std::collections::HashMap::new();
            for chapter in &chapters {
                let Some(title) = chapter.title else { continue };

                if let Some(other_bin) = seen.insert(title, chapter.name) {
                    panic!(
                        "Chapter {} and {} both have title '{title}', this is not valid.",
                        other_bin, chapter.name
                    );
                }
            }
            chapters
        });

        /// The chapters.
        #[cfg(not(test))]
        pub mod chapters {
            #(#mods)*
        }
    }
    .into()
}
