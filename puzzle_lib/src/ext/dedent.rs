/// Remove common leading whitespace strings.
pub trait Dedent {
    /// Remove leading whitespace from each line, as wel as leading and trailing newlines.
    ///
    /// This looks at the leading whitespace for the first nonempty line, and then removes that
    /// from the start of each line. It also removes any leading or trailing newlines & empty lines.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::prelude::*;
    /// assert_eq!("\n\thello\n\tworld\n".dedent(), "hello\nworld");
    /// assert_eq!("\n hello\n  world\n".dedent(), "hello\n world");
    /// assert_eq!(
    ///     "
    ///         hello
    ///         world
    ///     "
    ///     .dedent(),
    ///     "hello\nworld"
    /// );
    /// ```
    fn dedent(&self) -> String;
}

impl Dedent for &str {
    fn dedent(&self) -> String {
        let trimmed = self.trim_start_matches('\n').trim_end();
        let Some(idx) = trimmed.find(|c: char| !c.is_whitespace()) else {
            return trimmed.to_owned();
        };
        let (indent, remainder) = trimmed.split_at(idx);
        remainder.replace(&format!("\n{indent}"), "\n")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn simple() {
        assert_eq!("\n\thello\n\tworld\n".dedent(), "hello\nworld");
        assert_eq!("\n hello\n  world\n".dedent(), "hello\n world");
    }

    #[test]
    fn multiline_literal() {
        assert_eq!(
            "
                hello
                world
            "
            .dedent(),
            "hello\nworld"
        );
    }
}
