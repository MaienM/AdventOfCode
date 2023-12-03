use std::ops::Sub;

pub mod parse;
pub mod point;
pub mod runner;

/// Calculate the absolute difference between two (possibly unsigned) integers.
pub fn abs_diff<T>(a: T, b: T) -> T
where
    T: PartialOrd + Sub<Output = T>,
{
    if a > b {
        a - b
    } else {
        b - a
    }
}

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
                .map(|line| line.strip_prefix("        ").unwrap())
                .collect::<Vec<_>>()
                .join("\n")
        });
    };
}
