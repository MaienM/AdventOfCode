#[doc(hidden)]
#[macro_export]
macro_rules! __count {
    ($(,)?) => (0usize);
    ($item:expr, $($rest:expr),* $(,)?) => (1usize + $crate::__count!($($rest),*,));
}

/// Assert that the collection $actual contains all the provided items (and nothing else) in any
/// order.
#[macro_export]
macro_rules! assert_unordered_eq {
    ($actual:expr, $($expected:expr),+ $(,)?) => {{
        let actual: Vec<_> = $actual.collect();
        assert_eq!(actual.len(), $crate::__count!($($expected),+,));
        $crate::assert_unordered_eq!(items; actual; $($expected),+,);
    }};
    (items; $actual:ident; $item:expr, $($rest:expr),* $(,)?) => {
        assert!($actual.contains(&$item));
        $crate::assert_unordered_eq!(items; $actual; $($rest),*,);
    };
    (items; $actual:ident; $(,)?) => {};
}
