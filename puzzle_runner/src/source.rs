//! Helpers to manage inputs for solutions from various sources.

use std::{fs::read_to_string, io::ErrorKind};

use clap::{
    builder::{StringValueParser, TypedValueParser},
    parser::ValueSource,
};

/// The source of a solution input or expected output.
#[derive(Clone, Debug)]
pub enum Source {
    /// A path that was explicitly passed in by the user.
    ///
    /// Any error while reading this will be reported back.
    ExplicitPath(String),

    /// A path that was automatically chosen by the program.
    ///
    /// A "file does not exist" error will be treated as if there was no path, but any other IO error will be reported back to the user.
    AutomaticPath(String),

    /// An inline value.
    Inline { source: String, contents: String },

    /// No value available.
    None(
        /// A description of the purpose of this path.
        String,
    ),
}
impl Source {
    fn read_path(path: &str) -> Result<String, (ErrorKind, String)> {
        read_to_string(path)
            .map(|contents| contents.strip_suffix('\n').unwrap_or(&contents).to_owned())
            .map_err(|err| (err.kind(), format!("Failed to read {path}: {err}")))
    }

    /// Get the source of the value, if any.
    pub fn source(&self) -> Result<String, String> {
        match self {
            Source::ExplicitPath(path) | Source::AutomaticPath(path) => Ok(path.clone()),
            Source::Inline { source, .. } => Ok(source.clone()),
            Source::None(description) => Err(format!("No value for {description}.")),
        }
    }

    /// Attempt to read the file at the provided path, returning [`None`] when a non-fatal error occurs.
    pub fn read_maybe(&self) -> Result<Option<String>, String> {
        match self {
            Source::ExplicitPath(path) => Ok(Some(Self::read_path(path).map_err(|(_, e)| e)?)),
            Source::AutomaticPath(path) => match Self::read_path(path) {
                Ok(contents) => Ok(Some(contents)),
                Err((ErrorKind::NotFound, _)) => Ok(None),
                Err((_, err)) => Err(err),
            },
            Source::Inline { contents, .. } => Ok(Some(contents.clone())),
            Source::None(_) => Ok(None),
        }
    }

    /// Attempt to read the file at the provided path, returning an error if this fails for any reason.
    pub fn read(&self) -> Result<String, String> {
        match self {
            Source::ExplicitPath(path) | Source::AutomaticPath(path) => {
                Self::read_path(path).map_err(|(_, e)| e)
            }
            Source::Inline { contents, .. } => Ok(contents.clone()),
            Source::None(description) => Err(format!("No value for {description}.")),
        }
    }

    /// Mutate the contained path (if any). Does nothing for [`Source::None`].
    #[must_use]
    pub fn mutate_path<F>(&self, f: F) -> Self
    where
        F: Fn(String) -> String,
    {
        let mut result = self.clone();
        match result {
            Source::ExplicitPath(ref mut path) | Source::AutomaticPath(ref mut path) => {
                *path = f(std::mem::take(path));
            }
            _ => {}
        };
        result
    }
}

/// Parse arguments to [`Source`]s.
#[derive(Clone)]
pub struct SourceValueParser;
impl TypedValueParser for SourceValueParser {
    type Value = Source;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        _value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        panic!("Should never be called as parse_ref_ is implemented.");
    }

    fn parse_ref_(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
        source: ValueSource,
    ) -> Result<Self::Value, clap::Error> {
        let value = StringValueParser::new().parse_ref_(cmd, arg, value, source)?;

        if source == ValueSource::DefaultValue {
            Ok(Source::AutomaticPath(value))
        } else if value.is_empty() {
            Ok(Source::None(
                arg.map_or("unknown".to_owned(), |a| a.get_id().to_string()),
            ))
        } else {
            Ok(Source::ExplicitPath(value))
        }
    }
}

macro_rules! source_path_fill_tokens {
    ($path:expr, bin = $bin:expr) => {
        $path.mutate_path(|p| {
            p.replace("{name}", &$bin.name)
                .replace("{year}", &$bin.year.to_string())
                .replace("{day}", &$bin.day.to_string())
                .replace("{day0}", &$bin.name[3..])
        })
    };
    ($path:expr, bin = $bin:expr, part = $part:expr) => {
        source_path_fill_tokens!($path, bin = $bin)
            .mutate_path(|p| p.replace("{part}", &$part.to_string()))
    };
}
pub(super) use source_path_fill_tokens;
