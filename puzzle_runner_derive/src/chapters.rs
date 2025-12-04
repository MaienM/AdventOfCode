use std::{env, fs::read_to_string};

use proc_macro::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Error, Expr, ExprCall, ExprClosure, ExprPath, Item, ItemFn, ItemMod, ItemStatic, Lit, Meta,
    PathSegment, Token, parse_file, parse_macro_input, parse_quote,
    punctuated::Punctuated,
    visit::{self, Visit},
};

use crate::examples;

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
    pub(crate) book: Option<String>,
    pub(crate) title: Option<String>,
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
            book: None,
            title: None,
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

    pub(crate) fn to_expr(&self) -> Expr {
        let ChapterScanner {
            name,
            parts,
            examples,
            ..
        } = self;

        let book: Expr = match &self.book {
            Some(book) => parse_quote!(Some(#book)),
            None => parse_quote!(None),
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
    let expr = match Span::call_site().local_file() {
        Some(path) => {
            let mut scanner = ChapterScanner::scan_file(path.to_str().unwrap());

            if scanner.first_item != FirstItem::Register {
                return Error::new(
                    Span::call_site().into(),
                    "setup must be the first statement in the file",
                )
                .to_compile_error()
                .into();
            }

            let args_parser = syn::meta::parser(|meta| {
                if meta.path.is_ident("book") {
                    match meta.value()?.parse::<Lit>()? {
                        Lit::Str(book) => {
                            if book.value().is_empty() {
                                return Err(meta.error("cannot be empty"));
                            }
                            scanner.book = Some(book.value());
                        }
                        _ => return Err(meta.error("unsupported value, must be a string")),
                    }
                } else if meta.path.is_ident("title") {
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
