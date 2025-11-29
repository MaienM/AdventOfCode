/// Produce an expreesion evaluating to the number of passed (comma-separated) items.
#[macro_export]
macro_rules! count {
    ($(,)?) => (0usize);
    ($item:expr, $($rest:expr),* $(,)?) => (1usize + $crate::count!($($rest),*,));
}

/// Combine a series of expressions with an infix operator.
///
/// The following lines are equivalent:
///
/// ```rust,ignore
/// op_chain!(&&, first, second, third)
/// first && second && third
/// ```
#[macro_export]
macro_rules! op_chain {
    ($op:tt, $expr:expr $(,)?) => ($expr);
    ($op:tt, $first:expr, $second:expr $(, $($exprs:expr),*)?) => {
        $crate::op_chain!($op, $first $op $second $(, $($exprs),*)?)
    };
}

/// Combine a series of expressions into a chained method call.
///
/// The following lines are equivalent:
///
/// ```rust,ignore
/// call_chain!(sub, first, second, third)
/// first.sub(second).sub(third)
/// ```)
#[macro_export]
macro_rules! call_chain {
    ($fn:ident, $expr:expr $(,)?) => ($expr);
    ($fn:ident, $first:expr, $second:expr $(, $($exprs:expr),*)?) => {
        $crate::call_chain!($fn, $first.$fn($second) $(, $($exprs),*)?)
    };
}

/// Expand to the second argument, ignoring the first.
///
/// Useful to expand a macro group into a list of static values without using any of the captured
/// values.
#[macro_export]
macro_rules! static_ {
    ($ignore:tt, $use:ty) => {
        $use
    };
    ($ignore:tt, $use:tt) => {
        $use
    };
}

/// Assert that the collection $actual contains all the provided items (and nothing else) in any
/// order.
#[macro_export]
macro_rules! assert_unordered_eq {
    ($actual:expr, $($expected:expr),+ $(,)?) => {{
        let actual: Vec<_> = $actual.collect();
        assert_eq!(actual.len(), $crate::count!($($expected),+,));
        $crate::assert_unordered_eq!(items; actual; $($expected),+,);
    }};
    (items; $actual:ident; $item:expr, $($rest:expr),* $(,)?) => {
        assert!($actual.contains(&$item));
        $crate::assert_unordered_eq!(items; $actual; $($rest),*,);
    };
    (items; $actual:ident; $(,)?) => {};
}
