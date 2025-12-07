//! Handle inputs for solutions from various sources.

use std::{fs, io::ErrorKind, path::PathBuf};

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
        fs::read_to_string(path)
            .map(|contents| contents.strip_suffix('\n').unwrap_or(&contents).to_owned())
            .map_err(|err| (err.kind(), format!("Failed to read {path}: {err}")))
    }

    fn write_path(path: &str, contents: &str) -> Result<bool, (ErrorKind, String)> {
        fs::write(path, contents)
            .and(Ok(true))
            .map_err(|err| (err.kind(), format!("Failed to write {path}: {err}")))
    }

    fn join_paths(base: &str, tail: &str) -> Result<String, String> {
        let mut buf = PathBuf::new();
        buf.push(base);
        buf.push(tail);
        buf.to_str()
            .map(ToOwned::to_owned)
            .ok_or("failed to join paths".to_owned())
    }

    /// Get the source of the value, if any.
    pub fn source(&self) -> Result<String, String> {
        match self {
            Source::ExplicitPath(path) | Source::AutomaticPath(path) => Ok(path.clone()),
            Source::Inline { source, .. } => Ok(source.clone()),
            Source::None(description) => Err(format!("No value for {description}.")),
        }
    }

    /// Attempt to read the file at the provided path, returning [`Ok(None)`] when a non-fatal
    /// error occurs or there is no source to read from.
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

    /// Attempt to write the file at the provided path, returning [`Ok(false)`] when a non-fatal
    /// error occurs or there is no file source to write to.
    pub fn write_maybe(&self, contents: &str) -> Result<bool, String> {
        match self {
            Source::ExplicitPath(path) => Ok(Self::write_path(path, contents).map_err(|(_, e)| e)?),
            Source::AutomaticPath(path) => match Self::write_path(path, contents) {
                Ok(result) => Ok(result),
                Err((ErrorKind::NotFound, _)) => Ok(false),
                Err((_, err)) => Err(err),
            },
            _ => Ok(false),
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
        }
        result
    }

    /// Create an instance for a subpath.
    pub fn join(&self, subpath: &str, purpose: &str) -> Result<Source, String> {
        match self {
            Source::ExplicitPath(path) => {
                Ok(Source::ExplicitPath(Source::join_paths(path, subpath)?))
            }
            Source::AutomaticPath(path) => {
                Ok(Source::AutomaticPath(Source::join_paths(path, subpath)?))
            }
            Source::Inline { .. } => Err("Cannot .join on an inline source.".to_owned()),
            Source::None(_) => Ok(Source::None(purpose.to_owned())),
        }
    }
}

/// The source of the folder containing a chapter's solution input & expected outputs.
#[derive(Clone, Debug)]
pub struct ChapterSources(Source);
impl ChapterSources {
    pub fn input(&self) -> Result<Source, String> {
        self.0.join("input.txt", "The input for the chapter.")
    }

    pub fn part(&self, num: u8) -> Result<Source, String> {
        self.0.join(
            &format!("part{num}.txt"),
            &format!("The solution for part {num}."),
        )
    }

    /// Mutate the contained path (if any). Does nothing for [`Source::None`].
    #[must_use]
    pub fn mutate_path<F>(&self, f: F) -> Self
    where
        F: Fn(String) -> String,
    {
        Self(self.0.mutate_path(f))
    }
}

/// Parse argument to [`ChapterSources`].
#[derive(Clone)]
pub struct ChapterSourcesValueParser;
impl TypedValueParser for ChapterSourcesValueParser {
    type Value = ChapterSources;

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
            Ok(ChapterSources(Source::AutomaticPath(value)))
        } else if value.is_empty() {
            Ok(ChapterSources(Source::None(
                arg.map_or("unknown".to_owned(), |a| a.get_id().to_string()),
            )))
        } else {
            Ok(ChapterSources(Source::ExplicitPath(value)))
        }
    }
}

macro_rules! source_path_fill_tokens {
    ($path:expr, $($name:ident = $value:expr),+ $(,)?) => {
        $path.mutate_path(|p| {
            source_path_fill_tokens!(@replace; p, $($name = $value),+)
        })
    };
    (@replace; $chain:expr, $name:ident = $value:expr $(, $($restname:ident = $restvalue:expr),*)?) => {
        source_path_fill_tokens!(@replace; $chain.replace(&format!("{{{}}}", stringify!($name)), &$value.to_string()), $($($restname = $restvalue),*)?)
    };
    (@replace; $chain:expr, $(,)?) => {
        $chain
    };
}
pub(super) use source_path_fill_tokens;
