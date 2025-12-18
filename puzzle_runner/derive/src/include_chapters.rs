use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use regex::Regex;

use crate::utils::{find_crate_root, source_call_site};

fn find_chapters() -> Result<Vec<String>, String> {
    let span = source_call_site();
    let path = span
        .local_file()
        .ok_or("path of source file is empty".to_owned())?;
    let path = path
        .canonicalize()
        .map_err(|err| format!("failed to resolve {}: {err}", path.display()))?;
    let crate_root = find_crate_root(&path)?;
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

pub fn include_chapters(force: bool) -> TokenStream2 {
    #[allow(unused)]
    let mut include_chapters = force;
    #[cfg(feature = "include-chapters")]
    {
        include_chapters = true;
    }

    let mut mods: Vec<TokenStream2> = Vec::new();
    let mut chapters: Vec<TokenStream2> = Vec::new();

    if include_chapters {
        let files = find_chapters().unwrap_or_default();
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
    }

    quote! {
        pub static CHAPTERS: ::std::sync::LazyLock<Vec<::puzzle_runner::derived::Chapter>> = ::std::sync::LazyLock::new(|| {
            // Get the chapters.
            let chapters: Vec<::puzzle_runner::derived::Chapter> = {
                ::puzzle_runner::__internal::cfg_if! {
                    if #[cfg(any(test, doctest))] {
                        { Vec::new() }
                    } else {
                        { vec![ #(#chapters),* ] }
                    }
                }
            };

            // Validate that the titles are unique.
            let mut seen = ::std::collections::HashMap::new();
            for chapter in &chapters {
                let Some(ref title) = chapter.title else { continue };

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
        #[cfg(not(any(test, doctest)))]
        pub mod chapters {
            #(#mods)*
        }
    }
}
