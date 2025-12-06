use std::fmt::Debug;

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
