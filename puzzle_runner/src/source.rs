//! Handle inputs for solutions from various sources.

use std::{fs, io::ErrorKind, path::PathBuf};

/// The result of an operation on a [`Source`] or [`ChapterSources`].
pub enum IOResult<T> {
    /// The operation was successful.
    Ok(T),
    /// The item was not found.
    NotFound(String),
    /// An error occurred.
    Err(String),
}
impl<T> IOResult<T>
where
    T: Clone,
{
    /// Convert to the contained value in an option, converting [`NotFound`] to [`None`].
    pub fn to_option(&self) -> Result<Option<T>, String> {
        match self {
            IOResult::Ok(contents) => Ok(Some(contents.clone())),
            IOResult::NotFound(_) => Ok(None),
            IOResult::Err(err) => Err(err.clone()),
        }
    }

    /// Convert to the contained value, converting [`NotFound`] to an error.
    pub fn to_value(&self) -> Result<T, String> {
        match self {
            IOResult::Ok(contents) => Ok(contents.clone()),
            IOResult::NotFound(path) => Err(format!("{path} does not exist")),
            IOResult::Err(err) => Err(err.clone()),
        }
    }
}
impl<T> From<IOResult<T>> for Result<Option<T>, String>
where
    T: Clone,
{
    fn from(val: IOResult<T>) -> Self {
        val.to_option()
    }
}
impl<T> From<IOResult<T>> for Result<T, String>
where
    T: Clone,
{
    fn from(val: IOResult<T>) -> Self {
        val.to_value()
    }
}

/// Join the given paths and return as string.
fn join_paths(base: &str, tail: &str) -> Result<String, String> {
    let mut buf = PathBuf::new();
    buf.push(base);
    buf.push(tail);
    buf.to_str()
        .map(ToOwned::to_owned)
        .ok_or("failed to join paths".to_owned())
}

/// The source of a solution input or expected output.
#[derive(Clone, Debug)]
pub enum Source {
    /// A path on the filesystem.
    ///
    /// This can be read from and writen to. It can result in [`SourceIOResult::NotFound`].
    Path(String),

    /// An inline value.
    ///
    /// This can be read from but not written to. It cannot result in [`SourceIOResult::NotFound`].
    Inline { source: String, contents: String },
}
impl Source {
    /// Read the contents of the source.
    #[must_use]
    pub fn read(&self) -> IOResult<String> {
        match self {
            Source::Path(path) => match fs::read_to_string(path) {
                Ok(contents) => {
                    IOResult::Ok(contents.strip_suffix('\n').unwrap_or(&contents).to_owned())
                }
                Err(err) if err.kind() == ErrorKind::NotFound => IOResult::NotFound(path.clone()),
                Err(err) => IOResult::Err(format!("unable to read {path}: {err}")),
            },
            Source::Inline { contents, .. } => IOResult::Ok(contents.clone()),
        }
    }

    /// Replace the contents of the source.
    #[must_use]
    pub fn write(&self, contents: &str) -> IOResult<String> {
        match self {
            Source::Path(path) => match fs::write(path, contents) {
                Ok(()) => IOResult::Ok(contents.to_owned()),
                Err(err) if err.kind() == ErrorKind::NotFound => IOResult::NotFound(path.clone()),
                Err(err) => IOResult::Err(format!("unable to write {path}: {err}")),
            },
            Source::Inline { .. } => IOResult::Err("inline value, cannot be written".to_owned()),
        }
    }

    /// Get a human-readable description of the source of the value.
    #[must_use]
    pub fn source(&self) -> String {
        match self {
            Source::Path(path) => path.clone(),
            Source::Inline { source, .. } => source.clone(),
        }
    }

    /// Make a copy, applying the transformation to the path (if there is one).
    pub fn transform_path<F>(self, f: F) -> Self
    where
        F: FnOnce(String) -> String,
    {
        match self {
            Source::Path(path) => Source::Path(f(path)),
            value @ Source::Inline { .. } => value,
        }
    }
}

/// The set of sources (input & expected results) for a chapter.
#[derive(Clone)]
pub enum ChapterSources {
    /// A path on the filesystem.
    ///
    /// This must point to a directory. The input will refer to `input.txt` in this directory, and
    /// the parts to `partN.txt`.
    Path(String),

    /// An [`Example`](puzzle_runner::derived::Example).
    Example(Example),
}
impl ChapterSources {
    pub fn input(&self) -> IOResult<Source> {
        match self {
            ChapterSources::Path(path) => match join_paths(path, "input.txt") {
                Ok(path) => IOResult::Ok(Source::Path(path)),
                Err(err) => IOResult::Err(err),
            },
            ChapterSources::Example(example) => IOResult::Ok(Source::Inline {
                source: example.name.to_owned(),
                contents: example.input.to_owned(),
            }),
        }
    }

    pub fn part(&self, num: u8) -> IOResult<Source> {
        match self {
            ChapterSources::Path(path) => match join_paths(path, &format!("part{num}.txt")) {
                Ok(path) => IOResult::Ok(Source::Path(path)),
                Err(err) => IOResult::Err(err),
            },
            ChapterSources::Example(example) => {
                if let Some(contents) = example.parts.get(&num) {
                    IOResult::Ok(Source::Inline {
                        source: format!("{} part {num}", example.name),
                        contents: (*contents).to_owned(),
                    })
                } else {
                    IOResult::NotFound(format!("{} part {num}", example.name))
                }
            }
        }
    }

    /// Make a copy, applying the transformation to the path (if there is one).
    pub fn transform_path<F>(self, f: F) -> Self
    where
        F: FnOnce(String) -> String,
    {
        match self {
            ChapterSources::Path(path) => ChapterSources::Path(f(path)),
            value @ ChapterSources::Example(_) => value,
        }
    }
}

macro_rules! source_path_fill_tokens {
    ($path:expr, $($name:ident = $value:expr),+ $(,)?) => {
        $path.transform_path(|p| {
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

use crate::derived::Example;
