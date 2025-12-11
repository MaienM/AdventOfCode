use std::{env, fs::read_to_string};

use proc_macro::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{
    Attribute, Error, Expr, ExprCall, ExprClosure, ExprPath, Item, ItemFn, ItemMod, ItemStatic,
    Meta, PathSegment, Token, parse_file, parse_macro_input, parse_quote,
    punctuated::Punctuated,
    visit::{self, Visit},
};

use crate::{
    example_input, register_part,
    utils::{ParseNestedMetaExt, args_struct, return_err, source_crate},
};

args_struct! {
    struct Args {
        /// The book that the chapter is in.
        book: Option<String> = initial None,
        /// The title of the chapter.
        title: Option<String> = initial None,
    }
}

#[derive(Debug, Eq, PartialEq)]
enum FirstItem {
    Unknown,
    Register,
    Other,
}

trait HasAttrs {
    fn attrs(&self) -> &Vec<Attribute>;
    fn clear_attrs(&mut self);
}
macro_rules! impl_has_attrs {
    ($type:ty) => {
        impl HasAttrs for $type {
            fn attrs(&self) -> &Vec<Attribute> {
                &self.attrs
            }

            fn clear_attrs(&mut self) {
                self.attrs.clear()
            }
        }
    };
}
impl_has_attrs!(ItemStatic);
impl_has_attrs!(ItemFn);

struct ChapterScanner {
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

    /// If the item has the given attribute expand it now & feed the result back into this parser.
    ///
    /// Returns whether this was the case, which should trigger skipping further processing of the
    /// unexpanded version.
    fn expand_attr<N, F>(&mut self, node: &N, name: &str, expand_attr: F) -> bool
    where
        N: Clone + ToTokens + HasAttrs,
        F: Fn(TokenStream, TokenStream) -> TokenStream,
    {
        let Some(annotation) = node.attrs().iter().find(|attr| attr.path().is_ident(name)) else {
            return false;
        };

        let value = match &annotation.meta {
            Meta::Path(_) => quote!(),
            Meta::List(list) => list.tokens.clone(),
            Meta::NameValue(name_value) => {
                let value = &name_value.value;
                quote!(#value)
            }
        };

        let mut node = node.clone();
        node.clear_attrs();
        let result: TokenStream = expand_attr(value.into(), node.into_token_stream().into());
        visit::visit_file(self, &syn::parse(result).unwrap());

        true
    }
}
impl<'ast> Visit<'ast> for ChapterScanner {
    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        self.current_path.push(node.ident.clone().into());

        visit::visit_item_mod(self, node);
        self.current_path.pop();
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        // Check if this item is an unexpanded part. If it is expand it now and feed it back into this parser.
        if self.expand_attr(node, "register_part", register_part::main) {
            return;
        }

        visit::visit_item_fn(self, node);
    }

    fn visit_item_static(&mut self, node: &'ast ItemStatic) {
        // Check if this item is an expanded part.
        if node.ty == parse_quote!(::puzzle_runner::derived::Part) {
            let ident = &node.ident;
            self.parts.push(parse_quote!(#ident.clone()));
        }

        // Check if this item is an unexpanded example.
        if self.expand_attr(node, "example_input", example_input::main) {
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

pub fn main(input: TokenStream) -> TokenStream {
    let mut builder = Args::build();
    let args_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("book") {
            meta.set_empty_option(&mut builder.book, meta.parse_stringify_nonempty()?)?;
        } else if meta.path.is_ident("title") {
            meta.set_empty_option(&mut builder.title, meta.parse_stringify_nonempty()?)?;
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

    let main = if let Ok(name) = source_crate() {
        let crateident = format_ident!("{name}");
        quote! {
            pub fn main() {
                ::puzzle_runner::__internal::cfg_if! {
                    if #[cfg(feature = "bench")] {
                        ::puzzle_runner::__internal::bench::main(&*::#crateident::SERIES, &*CHAPTER);
                    } else {
                        ::puzzle_runner::__internal::single::main(&*::#crateident::SERIES, &*CHAPTER);
                    }
                }
            }
        }
    } else {
        quote!(todo!())
    };

    quote!{
        // Include prelude && register_part.
        #[allow(unused_imports)]
        use puzzle_lib::prelude::*;
        use puzzle_runner::register_part;

        // Store metadata in a static. This is used in the main method below, but it's also copied
        // to the full list used by the multi entrypoint.
        pub(crate) static CHAPTER: ::std::sync::LazyLock<::puzzle_runner::derived::Chapter> = ::std::sync::LazyLock::new(|| #expr );

        // Entrypoint that just runs this chapter.
        #main
    }
    .into_token_stream()
    .into()
}
