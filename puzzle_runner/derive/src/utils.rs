use std::path::{Path, PathBuf};

use proc_macro::Span;
use quote::format_ident;
use syn::{Error, Expr, Lit, meta::ParseNestedMeta, parse_quote};

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

/// Get expressions that refer to the series' [`puzzle_runner::derived::Series`] &
/// [`puzzle_runner::controller::Controller`].
pub fn get_series_and_controller() -> (Expr, Expr) {
    if let Ok(name) = source_crate() {
        let crateident = format_ident!("{name}");
        (
            parse_quote!(*::#crateident::SERIES),
            parse_quote!(::#crateident::CONTROLLER),
        )
    } else {
        (
            parse_quote!(
                ::puzzle_runner::derived::SeriesBuilder::default()
                    .build()
                    .unwrap()
            ),
            parse_quote!(::puzzle_runner::controller::DefaultController),
        )
    }
}

pub(crate) trait ParseNestedMetaExt {
    /// Convert a Result<..., String> to Result<..., Error>.
    fn map_err<T>(&self, err: Result<T, String>) -> Result<T, Error>;

    /// Parse a string or number into a string value.
    fn parse_stringify(&self) -> Result<String, Error>;
}
impl ParseNestedMetaExt for ParseNestedMeta<'_> {
    fn map_err<T>(&self, err: Result<T, String>) -> Result<T, Error> {
        err.map_err(|e| self.error(e))
    }

    fn parse_stringify(&self) -> Result<String, Error> {
        match self.value()?.parse::<Lit>()? {
            Lit::Str(lit) => Ok(lit.value()),
            Lit::Int(lit) => Ok(lit.base10_digits().to_string()),
            Lit::Float(lit) => Ok(lit.base10_digits().to_string()),
            _ => Err(self.error("unsupported value, must be a string or number"))?,
        }
    }
}

macro_rules! args_struct {
    {$(
        $(#[$structmeta:meta])*
        struct $name:ident {
            $(
                $(#[$varmeta:meta])*
                $var:ident: $type:ident $(<$($typeargs:ty),+>)? $(= $default:expr)?
            ),+
            $(,)?
        }
    )+} => {
        ::paste::paste!{$(
            $(#[$structmeta])*
            pub struct $name {
                $(
                    $(#[$varmeta])*
                    $var: $crate::utils::args_struct!(@type; $type $(<$($typeargs),+>)?)
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
                    $var: $crate::utils::args_struct!(@builder_type; $type $(<$($typeargs),+>)?)
                ),+
            }
            impl [<$name Builder>] {
                /// Convert into args.
                pub fn finalize(self) -> Result<$name, String> {
                    Ok($name {
                        $(
                            $var: $crate::utils::args_struct!(@builder_finalize; self.$var => $type $($default)?)
                        ),+
                    })
                }

                $(
                    $crate::utils::args_struct!(@builder_setters; $var as $type $(<$($typeargs),+>)?);
                )+
            }
        )+}
    };

    (@type; List $($args:tt)+) => (::std::vec::Vec $($args)+);
    (@type; Map $($args:tt)+) => (::std::collections::HashMap $($args)+);
    (@type; $type:ty) => ($type);

    (@builder_type; Option $($args:tt)+) => (Option $($args)+);
    (@builder_type; List $($args:tt)+) => (::std::vec::Vec $($args)+);
    (@builder_type; Map $($args:tt)+) => (::std::collections::HashMap $($args)+);
    (@builder_type; $($type:tt)+) => (Option<$($type)+>);

    (@builder_setters; $var:ident as Option<$type:ty>) => {
        $crate::utils::args_struct!(@builder_setters; $var as $type);
    };
    (@builder_setters; $var:ident as List<$type:ty>) => {
        ::paste::paste! {
            pub fn [<$var _push>](&mut self, item: $type)
            {
                self.$var.push(item);
            }
        }
    };
    (@builder_setters; $var:ident as Map<$ktype:ty, $vtype:ty>) => {
        ::paste::paste! {
            pub fn [<$var _insert>](&mut self, key: $ktype, value: $vtype) -> Result<(), String>
            {
                if self.$var.contains_key(&key) {
                    return Err(format!("{}.{key} has already been set", stringify!($var)));
                }
                self.$var.insert(key, value);
                Ok(())
            }
        }
    };
    (@builder_setters; $var:ident as $type:ty) => {
        ::paste::paste! {
            pub fn $var(&mut self, value: $type) -> Result<(), String>
            {
                if self.$var.is_some() {
                    return Err(format!("{} has already been set", stringify!($var)));
                }
                self.$var = Some(value);
                Ok(())
            }
        }
    };

    (@builder_finalize; $expr:expr => Option $($default:expr)?) => ($expr);
    (@builder_finalize; $expr:expr => List $($default:expr)?) => ($expr);
    (@builder_finalize; $expr:expr => Map $($default:expr)?) => ($expr);
    (@builder_finalize; $expr:expr => $type:ident $default:expr) => {
        $expr.unwrap_or_else(|| $default)
    };
    (@builder_finalize; $self:ident.$var:ident => $type:ident) => {
        $self.$var.ok_or_else(|| {
            format!("{} must be set", stringify!($var))
        })?
    };
}

#[allow(unused_imports)]
pub(crate) use {args_struct, return_err};
