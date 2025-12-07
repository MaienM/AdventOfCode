use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use proc_macro::Span;
use syn::{Error, Lit, meta::ParseNestedMeta};

/// Wrap a Result<T, String> to return on error in a function that returns a [`proc_macro::TokenStream`].
macro_rules! return_err {
    ($value:expr) => {
        match $value {
            Ok(value) => value,
            Err(err) => {
                return ::syn::Error::new(::proc_macro::Span::call_site().into(), err)
                    .to_compile_error()
                    .into();
            }
        }
    };
}

/// Get the location of the macro invocation that eventually lead to the current proc macro being
/// executed.
///
/// That is, if the proc macro's invocation was a result of the expansion of another macro this
/// will return the location of the call to that other macro, whereas [`Span::call_site`] would
/// return the location of that other macro's definition.
pub fn source_call_site() -> Span {
    let mut span = Span::call_site();
    while let Some(parent) = span.parent() {
        span = parent;
    }
    span
}

/// Get the name of the crate that invoked this macro.
///
/// See [`source_call_site`] for information on how this resolves nested macros. This assumes that
/// the name of the directory containing `Cargo.toml` matches the crate name, which is not
/// necessarily true, but is true for this repostory.
pub fn source_crate() -> Result<String, String> {
    let path = source_call_site()
        .local_file()
        .ok_or_else(|| "failed to determine crate name".to_owned())?;
    let path = find_crate_root(&path)?;
    path.file_name()
        .ok_or_else(|| {
            format!(
                "failed to determine crate name from path {}",
                path.display()
            )
        })
        .and_then(|n| {
            n.to_str()
                .ok_or_else(|| format!("failed to convert path {} to string", n.display()))
                .map(ToOwned::to_owned)
        })
}

/// Find the root of the crate containing the given path.
pub fn find_crate_root(path: &Path) -> Result<PathBuf, String> {
    let mut crate_root = path.to_path_buf();
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
                path.display(),
            ))?,
        }
    }
    Ok(crate_root)
}

pub(crate) trait ParseNestedMetaExt {
    /// Store the parsed value into an option, erroring if it is already set.
    fn set_empty_option<T>(&self, target: &mut Option<T>, value: T) -> Result<(), Error>
    where
        T: Debug;

    /// Parse a nonempty string value.
    fn parse_nonempty_string(&self) -> Result<String, Error>;
}
impl ParseNestedMetaExt for ParseNestedMeta<'_> {
    fn set_empty_option<T>(&self, target: &mut Option<T>, value: T) -> Result<(), Error>
    where
        T: Debug,
    {
        if let Some(value) = target {
            Err(self.error(format!(
                "duplicate value for {}, first value {value:?}",
                self.path.get_ident().unwrap()
            )))
        } else {
            *target = Some(value);
            Ok(())
        }
    }

    fn parse_nonempty_string(&self) -> Result<String, Error> {
        match self.value()?.parse::<Lit>()? {
            Lit::Str(value) => {
                if value.value().is_empty() {
                    return Err(self.error("cannot be empty"));
                }
                Ok(value.value())
            }
            _ => Err(self.error("unsupported value, must be a string")),
        }
    }
}

macro_rules! args_struct {
    {
        $(#[$structmeta:meta])*
        struct $name:ident {
            $(
                $(#[$varmeta:meta])*
                $var:ident: $type:ty $(= $default:expr)?
            ),+
            $(,)?
        }
    } => {
        ::paste::paste!{
            $(#[$structmeta])*
            pub struct $name {
                $(
                    $(#[$varmeta])*
                    pub $var: $type
                ),+
            }
            impl $name {
                #[allow(private_interfaces)]
                pub fn build() -> [<$name Builder>] {
                    [<$name Builder>]::default()
                }
            }

            #[derive(Default)]
            struct [<$name Builder>] {
                $(
                    pub $var: Option<$type>
                ),+
            }
            impl [<$name Builder>] {
                /// Convert into
                /// [`Args`].
                pub fn finalize(self) -> Result<Args, String> {
                    Ok(Args {
                        $(
                            $var: self.$var$(.or(Some($default)))?.ok_or_else(|| {
                                format!("{} must be set", stringify!($var))
                            })?
                        ),+
                    })
                }
            }
        }
    }
}

#[allow(unused_imports)]
pub(crate) use {args_struct, return_err};
