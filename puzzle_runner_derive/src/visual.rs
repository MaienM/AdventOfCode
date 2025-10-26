use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    visit::{self, Visit},
    Error, ItemEnum, ItemMod, ItemStruct, Path, PathSegment, Token, VisRestricted, Visibility,
};

struct ModScanner {
    current_path: Punctuated<PathSegment, Token![::]>,
    pub(crate) info: Result<Path, Error>,
}
impl ModScanner {
    pub(crate) fn rewrite_mod(module: &mut ItemMod) -> Self {
        let mut scanner = Self {
            current_path: Punctuated::default(),
            info: Err(Error::new(
                module.span(),
                "must contain `Info` enum".to_owned(),
            )),
        };

        scanner.visit_item_mod(module);
        scanner
    }
}
impl<'ast> Visit<'ast> for ModScanner {
    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        self.current_path.push(node.ident.clone().into());
        visit::visit_item_mod(self, node);
        self.current_path.pop();
    }

    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        if node.ident != "Info" {
            return;
        }

        match &node.vis {
            Visibility::Restricted(VisRestricted { path, .. }) if *path == parse_quote!(super) => {}
            _ => {
                self.info = Err(Error::new(node.span(), "must be pub(super)"));
                return;
            }
        };

        let ident = &node.ident;
        let cp = &self.current_path;
        self.info = Ok(parse_quote!(#cp::#ident));
    }
}

pub fn visual(_input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let mut module = parse_macro_input!(annotated_item as ItemMod);

    let scanner = ModScanner::rewrite_mod(&mut module);
    let info_path = match scanner.info {
        Ok(value) => value,
        Err(err) => return err.to_compile_error().into(),
    };

    quote! {
        #[cfg(feature = "visual")]
        #module

        /// Channel to send information to the current visualization.
        ///
        /// [`VISUAL_CHANNEL`] is available as a convenience for senders.
        #[cfg(feature = "visual")]
        static VISUAL_CHANNEL_FULL: ::std::sync::LazyLock<(
            ::std::sync::Arc<::std::sync::mpsc::Sender<#info_path>>,
            ::std::sync::Arc<::std::sync::Mutex<::std::sync::mpsc::Receiver<#info_path>>>,
        )> = ::std::sync::LazyLock::new(|| {
            let (tx, rx) = ::std::sync::mpsc::channel();
            (
                ::std::sync::Arc::new(tx),
                ::std::sync::Arc::new(::std::sync::Mutex::new(rx)),
            )
        });

        /// Sender to send information to the current visualization.
        #[cfg(feature = "visual")]
        static VISUAL_CHANNEL: ::std::sync::LazyLock<
            ::std::sync::Arc<::std::sync::mpsc::Sender<#info_path>>,
        > = ::std::sync::LazyLock::new(|| {
            VISUAL_CHANNEL_FULL.0.clone()
        });
    }
    .into()
}

pub fn derive_to_renderable(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let ident = &item.ident;

    quote! {
        impl ::puzzle_runner::visual::ToRenderable for #ident {}
        impl From<#ident> for ::std::boxed::Box<dyn ::puzzle_runner::visual::Renderable> {
            fn from(visual: #ident) -> Self {
                std::boxed::Box::new(::puzzle_runner::visual::CompleteVisual::new(
                    visual,
                    super::VISUAL_CHANNEL_FULL.1.lock().unwrap(),
                ))
            }
        }
    }
    .into()
}
