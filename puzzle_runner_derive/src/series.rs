use std::env;

use proc_macro::{Span, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use regex::Regex;
use syn::{Error, Lit, parse_macro_input};

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

fn find_chapters() -> Result<Vec<String>, String> {
    let path = "bin".to_owned();
    let filename_regex = Regex::new(r"^\d{2}-\d{2}\.rs$").unwrap();
    let source_path = Span::call_site()
        .local_file()
        .ok_or("path of source file is empty")?;
    let abs_path = env::current_dir()
        .map_err(|err| format!("error determining working directory: {err}"))?
        .join(source_path.clone())
        .parent()
        .ok_or(format!(
            "failed to determine parent of source file {}",
            source_path.display(),
        ))?
        .join(path.clone())
        .canonicalize()
        .map_err(|err| format!("error resolving {path:?}: {err}"))?;
    let mut files = Vec::new();
    let dir = abs_path.read_dir().map_err(|err| {
        format!(
            "error listing files in {path:?} (resolved to {}): {err}",
            abs_path.display()
        )
    })?;
    for entry in dir {
        let entry = entry.map_err(|err| {
            format!(
                "error listing files in {path:?} (resolved to {}): {err}",
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

pub fn register_series(input: TokenStream) -> TokenStream {
    let mut name: Option<String> = None;
    let mut title: Option<String> = None;
    let args_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("name") {
            match meta.value()?.parse::<Lit>()? {
                Lit::Str(value) => {
                    if value.value().is_empty() {
                        return Err(meta.error("cannot be empty"));
                    }
                    name = Some(value.value());
                }
                _ => return Err(meta.error("unsupported value, must be a string")),
            }
        } else if meta.path.is_ident("title") {
            match meta.value()?.parse::<Lit>()? {
                Lit::Str(value) => {
                    if value.value().is_empty() {
                        return Err(meta.error("cannot be empty"));
                    }
                    title = Some(value.value());
                }
                _ => return Err(meta.error("unsupported value, must be a string")),
            }
        } else {
            return Err(meta.error("unsupported property"));
        }
        Ok(())
    });
    parse_macro_input!(input with args_parser);

    let Some(name) = name else {
        return Error::new(Span::call_site().into(), "name must be set")
            .to_compile_error()
            .into();
    };
    let Some(title) = title else {
        return Error::new(Span::call_site().into(), "title must be set")
            .to_compile_error()
            .into();
    };

    let mut mods: Vec<TokenStream2> = Vec::new();
    let mut chapters: Vec<TokenStream2> = Vec::new();

    let files = return_err!(find_chapters(), Span::call_site().into());
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
        chapters.push(quote!(crate::generated::chapters::#modident::CHAPTER.clone()));
        mods.push(quote! {
            #[path = #file]
            pub mod #modident;
        });
    }

    quote! {
        #[cfg(not(test))]
        pub mod generated {
            // Series info. This is used in the entrypoints below, but it's also imported by the WASM
            // create.
            pub static SERIES: ::std::sync::LazyLock<::puzzle_runner::derived::Series> = ::std::sync::LazyLock::new(|| {
                // Get chapters & validate that their names are unique.
                let chapters: Vec<::puzzle_runner::derived::Chapter> = vec![ #(#chapters),* ];
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

                ::puzzle_runner::derived::Series {
                    name: #name,
                    title: #title,
                    chapters,
                }
            });

            /// Entrypoint for the combined binary.
            pub fn multi() {
                puzzle_runner::multi::main(&SERIES);
            }

            /// Entrypoint for the benchmarks.
            #[cfg(feature = "bench")]
            pub fn bench() {
                puzzle_runner::bench::main(&SERIES);
            }

            /// The chapters.
            pub mod chapters {
                #(#mods)*
            }
        }
    }
    .into()
}
