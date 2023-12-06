pub mod parse;
pub mod point;
pub mod runner;
pub mod utils;

/// Define an example for the tests which will be stripped of leading/trailing newline and a static amount of indentation.
#[macro_export]
macro_rules! example {
    (
        $( #[$attrs:meta] )*
        $pub:vis
        static $name:ident: String = $data:expr ;
    ) => {
        $( #[$attrs] )*
        $pub
        static $name: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
            ($data)
                .strip_prefix('\n')
                .unwrap()
                .trim_end_matches(' ')
                .strip_suffix('\n')
                .unwrap()
                .split('\n')
                .map(|line| line.strip_prefix("        ").unwrap_or(line))
                .collect::<Vec<_>>()
                .join("\n")
        });
    };
}
