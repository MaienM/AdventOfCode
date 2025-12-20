use std::{env, fs::read_to_string};

use proc_macro::{Span, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    Attribute, Error, Expr, ExprCall, ExprClosure, ExprPath, Ident, Item, ItemFn, ItemMod,
    ItemStatic, Meta, PathSegment, Token, parse_file, parse_macro_input, parse_quote,
    punctuated::Punctuated,
    visit::{self, Visit},
};

use crate::{
    example_input, register_part,
    utils::{ParseNestedMetaExt, args_struct, get_series_and_controller, return_err},
};

args_struct! {
    struct Args {
        /// Metadata to pass directly to the builder.
        metadata: Map<Ident, Expr>,
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

    pub(crate) fn to_expr(&self) -> TokenStream2 {
        let ChapterScanner {
            name,
            parts,
            examples,
            ..
        } = self;

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

        quote! {
            builder.name(#name);
            builder.source_path(#source_path);
            builder.parts(vec![ #(#parts),* ]);
            builder.examples(vec![ #(#examples),* ]);
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
        if node.ty == parse_quote!(::std::sync::LazyLock<::puzzle_runner::derived::Part>) {
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
        if let Some(key) = meta.path.get_ident() {
            meta.map_err(builder.metadata_insert(key.clone(), meta.value()?.parse()?))?;
        }
        Ok(())
    });
    parse_macro_input!(input with args_parser);
    let args = return_err!(builder.finalize());

    let scanner_expressions = match Span::call_site().local_file() {
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
            scanner.to_expr()
        }
        None => {
            parse_quote! { ::core::todo!() }
        }
    };

    let metadata_expressions = args
        .metadata
        .into_iter()
        .map(|(k, v)| quote!(builder.#k(#v);));

    let (series, controller) = get_series_and_controller();

    quote!{
        // Include prelude && register_part.
        #[allow(unused_imports)]
        use puzzle_lib::prelude::*;
        use puzzle_runner::register_part;

        // Store metadata in a static. This is used in the main method below, but it's also copied
        // to the full list used by the multi entrypoint.
        pub(crate) static CHAPTER: ::std::sync::LazyLock<::puzzle_runner::derived::Chapter> = ::std::sync::LazyLock::new(|| {
            let mut builder = ::puzzle_runner::derived::ChapterBuilder::default();
            #scanner_expressions
            #(#metadata_expressions)*
            #controller.process_chapter(&mut builder).unwrap();
            builder.build().unwrap()
        });

        // Entrypoint that just runs this chapter.
        pub fn main() {
            ::puzzle_runner::__internal::cfg_if! {
                if #[cfg(feature = "bench")] {
                    ::puzzle_runner::__internal::bench::main(&#series, &*CHAPTER);
                } else {
                    ::puzzle_runner::__internal::single::main(&#series, &*CHAPTER);
                }
            }
        }
    }
    .into_token_stream()
    .into()
}
