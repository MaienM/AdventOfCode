//! Helpers for parsing text into structures

#[doc(hidden)]
#[macro_export]
macro_rules! __parse {
    ($input:expr => { $($sections:tt)+ } => $result:expr) => {
        {
            $crate::utils::parser::__parse__!([[ tmpvar ]] $input => $($sections)*);
            $result
        }
    };
    ($input:expr => $($sections:tt)+) => {
        $crate::utils::parser::__parse__!([[ tmpvar ]] $input => $($sections)*);
    };
}

/// Macro to quickly perform common parsing operations on a string.
///
/// This macro will take the given input, attempt to parse all the provided sections from it,
/// (defining the variables and performing the transformations defined by these sections), and then
/// ensure that the input is fully consumed by this. Any failures to do so will cause it to panic
/// with a (hopefully) informative message.
///
/// The `parse!(input => { sections } => result)` form is a shorthand for `{ parse!(input =>
/// sections); result }`, turning the result of the macro into a single expression rather than a
/// list of assignments.
///
/// # Sections
///
/// The following sections are supported.
///
/// - [A literal value](#literals). When used at the start this will strip a prefix, and when used
///   after any assignment it will set the boundary for the text used for that assignment; that is,
///   the first matching instance of this literal will be found and the text between that and the
///   previous literal/start of input will be used for the assignment. The matched literal itself
///   will be discarded (i.e. not assigned to any variables).
/// - An identifier (`name`). This is a shorthand for `[name]`.
/// - An assignment (`[name]`). This will take the value that is matched at the given location and
///   store it in a variable with that name. This supports [transformations](#transformations)
///   (e.g. `[name as type]`).
///
/// # Literals
///
/// The following types of literals are supported:
///
/// - A literal string (`"foo"`).
/// - A literal character (`'a'`).
/// - A regular expression. (`/"pattern/"`). You can also use a [raw string
///   literal](https://doc.rust-lang.org/reference/tokens.html#raw-string-literals) between the
///   slashes if convenient (`/r#"\w+"#/`).
///
/// # Transformations
///
/// The following options can be used to transform a matched value:
///
/// - A typecast (`as type`). This will convert the matched string into the given type, either
///   using one of the built-in methods for some of the rust primitives, or using `type::from` as a
///   fallback for any other types.
/// - A call (`with func`). This will call the provided function with a single argument (the
///   matched string), and it will then store the result of that call. When used on an indexed
///   iterator the single argument will instead be a tuple (`(index: usize, matched: &str)`).
/// - A nested parse (`with { segments } => result`). This take the matched string and feed it into
///   a nested call of this macro with the given segments & result mapping.
/// - [A split operation](#split).
///
/// # Split
///
/// The split keyword will take the matched string and split it into an iterator or collection.
/// This entire operation is described by a series of options (most optional) which describe how to
/// split the string and how to process the resulting iterator.
///
/// Here are all the options, in the order they must appear in:
///
/// - The thing to split on (required). Options:
///   - `split`. This is a shorthand for `split on ' '`.
///   - `split on literal`. This will split the string on the given [literal](#literal), discarding
///     the separators and keeping the elements between the separators..
///   - `chars`. This will iterate over the individual [`char`]acters of the string.
///   - `find /"pattern"/`. This will look for all matches of the given regular expression and keep
///     the entire matched string, ignoring any capture groups.
///   - `capture /"pattern"/`. This will look for all matches of the given regular expression and
///     keep the [`regex::Captures`] objects.
/// - `indexed` (optional). This will chain [`std::iter::Iterator::enumerate`] on the iterator.
/// - The collection to transform the iterator into (optional, default `into (Vec<_>)`):
///   - `into iterator`. This will keep the result as an [`std::iter::Iterator`] (the exact type of
///     which depends on the other options).
///   - `into (type)`. This will transform the iterator into the given collection (i.e.
///     `.collect::<type>()`).
/// - A transformation (optional):
///   - Any of the base [transformations](#transformations).
///   - `try transformation`. A base [transformation](#transformations) that's allowed to fail,
///     with failing elements dropped from the resulting iterator. Only supported for typecasts and
///     calls.
///
/// # Examples
///
/// Literals in various positions:
///
/// ```
/// # use aoc::utils::parse;
/// parse!("hello world" => "hello " subject);
/// assert_eq!(subject, "world");
/// ```
/// ```
/// # use aoc::utils::parse;
/// parse!("howdy pardner" => greeting /r#" \w+"#/);
/// assert_eq!(greeting, "howdy");
/// ```
/// ```
/// # use aoc::utils::parse;
/// parse!("g'day mate" => greeting ' ' subject);
/// assert_eq!(greeting, "g'day");
/// assert_eq!(subject, "mate");
/// ```
///
/// Transformations:
///
/// ```
/// # use aoc::utils::parse;
/// parse!("detonating in 4..." => "detonating in " [seconds as u8] "...");
/// assert_eq!(seconds, 4u8);
/// ```
/// ```
/// # use aoc::utils::parse;
/// parse!("boo" => [word with str::to_uppercase]);
/// assert_eq!(word, "BOO");
/// ```
/// ```
/// # use aoc::utils::parse;
/// //parse!("|1 -1|" => '|' [pair with { [l as i8] ' ' [r as i8] } => (l, r)] '|');
/// //assert_eq!(pair, (1i8, -1i8));
/// ```
///
/// Split:
///
/// ```
/// # use aoc::utils::parse;
/// parse!("fee-fi-fo-fum" => [words split on '-']);
/// assert_eq!(words, vec!["fee", "fi", "fo", "fum"]);
/// ```
/// ```
/// # use aoc::utils::parse;
/// parse!("10 -10" => [nums split into iterator as i8]);
/// assert_eq!(nums.next(), Some(10));
/// assert_eq!(nums.next(), Some(-10));
/// assert_eq!(nums.next(), None);
/// ```
/// ```
/// # use std::collections::HashSet;
/// # use aoc::utils::parse;
/// parse!("n33dl3 in 4 h4yst4ck" => [nums chars into (HashSet<_>) try as u8]);
/// assert_eq!(nums, HashSet::from([3, 4]));
/// ```
/// ```
/// # use aoc::utils::parse;
/// parse!("2 fast 2 furious" => [words find /"[a-z]+"/]);
/// assert_eq!(words, vec!["fast", "furious"]);
/// ```
/// ```
/// # use std::collections::HashMap;
/// # use regex::Captures;
/// # use aoc::utils::parse;
/// fn to_pair(capture: Captures) -> (&str, u8) {
///     (
///         capture.get(1).unwrap().as_str(),
///         capture.get(2).unwrap().as_str().parse().unwrap(),
///     )
/// }
/// parse!("foo=1 bar baz=2" => [map capture /r#"(\w+)=(\d+)"#/ into (HashMap<_, _>) with to_pair]);
/// assert_eq!(map.len(), 2);
/// assert_eq!(map.get(&"foo"), Some(&1));
/// assert_eq!(map.get(&"baz"), Some(&2));
/// ```
pub use __parse as parse;

/// Implementation of [`parse!`].
#[doc(hidden)]
#[macro_export]
macro_rules! __parse__ {
    // Store element as identifier.
    // [
    //   $name
    //   (
    //      as $type ||
    //      with $transformer ||
    //      with [{ nested } => result]
    //   )
    // ]
    // name; $type will be &str

    ([[ $($tmpnames:ident)+ ]] $input:expr => [ $ident:ident as $type:tt ]) => {
        let $ident = $crate::utils::parser::__parse_type__!($input => str => $type);
    };
    ([[ $($tmpnames:ident)+ ]] $input:expr => [ $ident:ident with $transformer:expr ]) => {
        let $ident = $transformer($input);
    };
    ([[ $($tmpnames:ident)+ ]] $input:expr => [ $ident:ident with { $($nested:tt)+ } => $result:expr ]) => {
        let $ident = $crate::utils::parser::parse!($input => { $($nested)+ } => $result);
    };
    ([[ $($tmpnames:ident)+ ]] $input:expr => $ident:ident) => {
        $crate::utils::parser::__parse__!([[ $($tmpnames)* ]] $input => [ $ident as str ]);
    };

    // Split element into a collection.
    // [
    //   $name
    //   (
    //      chars ||
    //      find /$pattern/ ||
    //      capture /$pattern/ ||
    //      split on $sep; default " "
    //   )
    //   indexed?
    //   (
    //      into iterator ||
    //      into ($collection);
    //      default $collection Vec
    //   )
    //   (
    //      try? as $type ||
    //      with [nested-bracketed] ||
    //      with { nested } => result ||
    //      try? with $transformer;
    //      default $type &str)
    //   )
    // ]

    // chars
    ([[ $($tmpnames:ident)+ ]] $input:expr => [ $ident:ident chars $($rest:tt)* ]) => {
        #[allow(unused_mut)]
        let mut $ident = $crate::utils::parser::__parse__!(split; char; $input => [ sel::[chars] ]; $($rest)*);
    };
    // find /"regex"/
    ([[ $($tmpnames:ident)+ ]] $input:expr => [ $ident:ident find /$pattern:literal/ $($rest:tt)* ]) => {
        #[allow(unused_mut)]
        let mut $ident = $crate::utils::parser::__parse__!(split; str; $input => [ sel::[find $pattern] ]; $($rest)*);
    };
    // capture /"regex"/
    ([[ $($tmpnames:ident)+ ]] $input:expr => [ $ident:ident capture /$pattern:literal/ $($rest:tt)* ]) => {
        #[allow(unused_mut)]
        let mut $ident = $crate::utils::parser::__parse__!(split; str; $input => [ sel::[capture $pattern] ]; $($rest)*);
    };
    // split
    ([[ $($tmpnames:ident)+ ]] $input:expr => [ $ident:ident split $($rest:tt)* ]) => {
        #[allow(unused_mut)]
        let mut $ident = $crate::utils::parser::__parse__!(split; str; $input => [ ]; $($rest)*);
    };
    // on $sep
    (split; $ty:tt; $input:expr => [ ]; on /$sep:literal/ $($rest:tt)*) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [ sel::[on regex $sep] ]; $($rest)*)
    };
    (split; $ty:tt; $input:expr => [ ]; on $sep:literal $($rest:tt)*) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [ sel::[on literal $sep] ]; $($rest)*)
    };
    (split; $ty:tt; $input:expr => [ ]; $($rest:tt)*) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [ sel::[on literal " "] ]; $($rest)*)
    };
    // indexed
    (split; $ty:tt; $input:expr => [ sel::$selargs:tt ]; indexed $($rest:tt)*) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [ sel::$selargs into::[indexed] ]; $($rest)*)
    };
    (split; $ty:tt; $input:expr => [ sel::$selargs:tt ]; $($rest:tt)*) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [ sel::$selargs into::[] ]; $($rest)*)
    };
    // into $collection
    (split; $ty:tt; $input:expr => [ sel::$selargs:tt into::[$($flags:ident)*] ]; into iterator $($rest:tt)*) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [ sel::$selargs into::[$($flags)*; Iterator] ]; $($rest)*)
    };
    (split; $ty:tt; $input:expr => [ sel::$selargs:tt into::[$($flags:ident)*] ]; into ($collection:ty) $($rest:tt)*) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [ sel::$selargs into::[$($flags)*; $collection] ]; $($rest)*)
    };
    (split; $ty:tt; $input:expr => [ sel::$selargs:tt into::[$($flags:ident)*] ]; $($rest:tt)*) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [ sel::$selargs into::[$($flags)*; Vec<_>] ]; $($rest)*)
    };
    // (try)? as $type
    (split; $ty:tt; $input:expr => [ sel::$selargs:tt into::$iterargs:tt ]; as $type:tt) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [ sel::$selargs into::$iterargs with::[
            |item| $crate::utils::parser::__parse_type__!(item => $ty => $type)
        ] ];)
    };
    (split; $ty:tt; $input:expr => [ sel::$selargs:tt into::$iterargs:tt ]; try as $type:tt) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [ sel::$selargs into::$iterargs with::[try
            |item| $crate::utils::parser::__parse_type__!(item => $ty => try $type)
        ] ];)
    };
    // with [nested-bracketed]
    (split; $ty:tt; $input:expr => [ sel::$selargs:tt into::$iterargs:tt ]; with [ $($nested:tt)+ ]) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [ sel::$selargs into::$iterargs with::[
            |item| {
                $crate::utils::parser::__parse__!([[ tmpvar ]] item => [ result $($nested)+ ]);
                result
            }
        ] ];)
    };
    // with { nested } => result
    (split; $ty:tt; $input:expr => [ sel::$selargs:tt into::$iterargs:tt ]; with { $($nested:tt)+ } => $result:expr) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [ sel::$selargs into::$iterargs with::[
            |item| $crate::utils::parser::parse!(item => { $($nested)+ } => $result)
        ] ];)
    };
    // (try)? with $transformer
    (split; $ty:tt; $input:expr => [ sel::$selargs:tt into::[indexed; $($iterargs:tt)+] ]; with $transformer:expr) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [ sel::$selargs into::[; $($iterargs)+] with::[indexed $transformer] ];)
    };
    (split; $ty:tt; $input:expr => [ sel::$selargs:tt into::[indexed; $($iterargs:tt)+] ]; try with $transformer:expr) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [ sel::$selargs into::[; $($iterargs)+] with::[try indexed $transformer] ];)
    };
    (split; $ty:tt; $input:expr => [ sel::$selargs:tt into::$iterargs:tt ]; with $transformer:expr) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [ sel::$selargs into::$iterargs with::[$transformer] ];)
    };
    (split; $ty:tt; $input:expr => [ sel::$selargs:tt into::$iterargs:tt ]; try with $transformer:expr) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [ sel::$selargs into::$iterargs with::[try $transformer] ];)
    };
    // done
    (split; $ty:tt; $input:expr => [ $($args:tt)* ];) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [[ $($args)* ]])
    };
    (split; $ty:tt; $input:expr => [[ sel::$selargs:tt into::[] $($rest:tt)* ]]) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [[ sel::$selargs $($rest)* ]])
    };
    (split; $ty:tt; $input:expr => [[ sel::$selargs:tt into::[indexed] $($rest:tt)* ]]) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [[ sel::$selargs $($rest)* ]]).enumerate()
    };
    (split; $ty:tt; $input:expr => [[ sel::$selargs:tt into::[$($flags:ident)*; Iterator] $($rest:tt)* ]]) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [[ sel::$selargs into::[$($flags)*] $($rest)* ]])
    };
    (split; $ty:tt; $input:expr => [[ sel::$selargs:tt into::[$($flags:ident)*; $collection:ty] $($rest:tt)* ]]) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [[ sel::$selargs into::[$($flags)*] $($rest)* ]]).collect::<$collection>()
    };
    (split; $ty:tt; $input:expr => [[ sel::$selargs:tt with::[try $($flags:ident)* $transformer:expr] ]]) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [[ sel::$selargs with::[$($flags)* $transformer] ]]).filter_map(Result::ok)
    };
    (split; $ty:tt; $input:expr => [[ sel::$selargs:tt with::[indexed $transformer:expr] ]]) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [[ sel::$selargs ]]).enumerate().map($transformer)
    };
    (split; $ty:tt; $input:expr => [[ sel::$selargs:tt with::[$transformer:expr] ]]) => {
        $crate::utils::parser::__parse__!(split; $ty; $input => [[ sel::$selargs ]]).map($transformer)
    };
    (split; $ty:tt; $input:expr => [[ sel::[chars] ]]) => {
        $input.chars()
    };
    (split; $ty:tt; $input:expr => [[ sel::[on $sepkind:ident $sep:literal] ]]) => {
        $crate::utils::parser::__parse_literal__!(
            $sepkind;
            split;
            $input;
            $crate::utils::parser::__parse_literal__!($sepkind; create; $sep)
        )
    };
    (split; $ty:tt; $input:expr => [[ sel::[find $pattern:literal] ]]) => {
        ::regex::Regex::new($pattern).unwrap().find_iter($input).map(|m| m.as_str())
    };
    (split; $ty:tt; $input:expr => [[ sel::[capture $pattern:literal] ]]) => {
        ::regex::Regex::new($pattern).unwrap().captures_iter($input)
    };

    // Empty tail.
    ([[ $($tmpnames:ident)+ ]] $input:expr =>) => {
        assert_eq!($input, "", "unparsed tokens in input");
    };

    // Ignore element.
    ([[ $($tmpnames:ident)+ ]] $input:expr => _) => {
        let _ = $input;
    };

    // Leading literal.
    ([[ $($tmpnames:ident)+ ]] $input:expr => $prefix:literal $($rest:tt)*) => {
        $crate::utils::parser::__parse__!(literal; literal; [[ $($tmpnames)+ ]] $input => $prefix $($rest)*);
    };
    ([[ $($tmpnames:ident)+ ]] $input:expr => /$prefix:literal/ $($rest:tt)*) => {
        $crate::utils::parser::__parse__!(literal; regex; [[ $($tmpnames)+ ]] $input => $prefix $($rest)*);
    };
    (literal; $litkind:ident; [[ $($tmpnames:ident)+ ]] $input:expr => $prefix:literal $($rest:tt)*) => {
        ::paste::paste!{
            let [< $($tmpnames)+ _input >] = $input;
            let [< $($tmpnames)+ _prefix >] = $crate::utils::parser::__parse_literal__!($litkind; create; $prefix);
            let [< $($tmpnames)+ _stripped >] = $crate::utils::parser::__parse_literal__!(
                $litkind;
                strip-prefix;
                ::paste::paste!([< $($tmpnames)+ _input >]);
                ::paste::paste!([< $($tmpnames)+ _prefix >])
            );
        };
        $crate::utils::parser::__parse__!([[ $($tmpnames)+ ]] ::paste::paste!([< $($tmpnames)+ _stripped >]).ok_or_else(|| {
            format!(
                "couldn't find {:?} at the start of {:?}",
                ::paste::paste!([< $($tmpnames)+ _prefix >]),
                ::paste::paste!([< $($tmpnames)+ _input >]),
            )
        }).unwrap() => $($rest)*);
    };

    // Recursively process everything until the next instance of a given literal.
    ([[ $($tmpnames:ident)+ ]] $input:expr => $first:tt $sep:literal $($rest:tt)*) => {
        $crate::utils::parser::__parse__!(literal; literal; [[ $($tmpnames)+ ]] $input => $first $sep $($rest)*);
    };
    ([[ $($tmpnames:ident)+ ]] $input:expr => $first:tt /$sep:literal/ $($rest:tt)*) => {
        $crate::utils::parser::__parse__!(literal; regex; [[ $($tmpnames)+ ]] $input => $first $sep $($rest)*);
    };
    (literal; $litkind:ident; [[ $($tmpnames:ident)+ ]] $input:expr => $first:tt $sep:literal $($rest:tt)*) => {
        ::paste::paste!{
            let [< $($tmpnames)+ _input >] = $input;
            let [< $($tmpnames)+ _sep >] = $crate::utils::parser::__parse_literal__!($litkind; create; $sep);
            let mut [< $($tmpnames)+ >] = $crate::utils::parser::__parse_literal__!(
                $litkind;
                split-once;
                ::paste::paste!([< $($tmpnames)+ _input >]);
                ::paste::paste!([< $($tmpnames)+ _sep >])
            );
        };
        $crate::utils::parser::__parse__!([[ $($tmpnames)+ _1 ]] ::paste::paste!([< $($tmpnames)+ >]).next().unwrap() => $first);
        $crate::utils::parser::__parse__!([[ $($tmpnames)+ _2 ]] ::paste::paste!([< $($tmpnames)+ >]).next().ok_or_else(|| {
            format!(
                "couldn't find {:?} in {:?}",
                ::paste::paste!([< $($tmpnames)+ _sep >]),
                ::paste::paste!([< $($tmpnames)+ _input >]),
            )
        }).unwrap() => $($rest)*);
    };
}
#[doc(hidden)]
pub use __parse__;

/// Helper macro to convert text to another type.
#[doc(hidden)]
#[macro_export]
#[rustfmt::skip]
macro_rules! __parse_type__ {
    ($var:expr => str => str) => ($var);

    ($var:expr => str => try str) => (Ok($var));
    ($var:expr => str => try char) => ({
        let t = $var;
        if t.len() == 1 {
            t.chars().next()
        } else {
            Err(format!("cannot convert {t:?} to char as it's more than one character"))
        }
    });
    ($var:expr => str => try usize) => ($var.parse::<usize>());
    ($var:expr => str => try u128) => ($var.parse::<u128>());
    ($var:expr => str => try u64) => ($var.parse::<u64>());
    ($var:expr => str => try u32) => ($var.parse::<u32>());
    ($var:expr => str => try u16) => ($var.parse::<u16>());
    ($var:expr => str => try u8) => ($var.parse::<u8>());
    ($var:expr => str => try isize) => ($var.parse::<isize>());
    ($var:expr => str => try i128) => ($var.parse::<i128>());
    ($var:expr => str => try i64) => ($var.parse::<i64>());
    ($var:expr => str => try i32) => ($var.parse::<i32>());
    ($var:expr => str => try i16) => ($var.parse::<i16>());
    ($var:expr => str => try i8) => ($var.parse::<i8>());
    ($var:expr => str => try f64) => ($var.parse::<f64>());
    ($var:expr => str => try f32) => ($var.parse::<f32>());

    ($var:expr => char => char) => ($var);
    ($var:expr => char => str) => ($var.to_string());

    ($var:expr => char to digit) => ({
        let t = $var;
        t.to_digit(10).ok_or_else(|| format!("cannot convert character {t} to a number"))
    });
    ($var:expr => char => try usize) => ($crate::utils::parser::__parse_type__!($var => char to digit).map(|v| v as usize));
    ($var:expr => char => try u128) => ($crate::utils::parser::__parse_type__!($var => char to digit).map(|v| v as u128));
    ($var:expr => char => try u64) => ($crate::utils::parser::__parse_type__!($var => char to digit).map(|v| v as u64));
    ($var:expr => char => try u32) => ($crate::utils::parser::__parse_type__!($var => char to digit));
    ($var:expr => char => try u16) => ($crate::utils::parser::__parse_type__!($var => char to digit).map(|v| v as u16));
    ($var:expr => char => try u8) => ($crate::utils::parser::__parse_type__!($var => char to digit).map(|v| v as u8));
    ($var:expr => char => try isize) => ($crate::utils::parser::__parse_type__!($var => char to digit).map(|v| v as isize));
    ($var:expr => char => try i128) => ($crate::utils::parser::__parse_type__!($var => char to digit).map(|v| v as i128));
    ($var:expr => char => try i64) => ($crate::utils::parser::__parse_type__!($var => char to digit).map(|v| v as i64));
    ($var:expr => char => try i32) => ($crate::utils::parser::__parse_type__!($var => char to digit).map(|v| v as i32));
    ($var:expr => char => try i16) => ($crate::utils::parser::__parse_type__!($var => char to digit => char to digit));
    ($var:expr => char => try i8) => ($crate::utils::parser::__parse_type__!($var => char to digit).map(|v| v as i8));
    ($var:expr => char => try f64) => ($crate::utils::parser::__parse_type__!($var => char to digit).map(|v| v as f64));
    ($var:expr => char => try f32) => ($crate::utils::parser::__parse_type__!($var => char to digit).map(|v| v as f32));

    ($var:expr => $from:tt => $to:tt) => {
        $crate::utils::parser::__parse_type__!($var => $from => try $to).unwrap()
    };

    ($var:expr => $from:tt => try $type:tt) => ($type::try_from($var));
    ($var:expr => $from:tt => $type:tt) => ($type::from($var));
}
#[doc(hidden)]
pub use __parse_type__;

/// Helper macro to handle the various types of literals (plain strings, regexes).
#[doc(hidden)]
#[macro_export]
#[rustfmt::skip]
macro_rules! __parse_literal__ {
    (literal; create; $sep:literal) => ($sep);
    (literal; split; $input:expr; $sep:expr) => ($input.split($sep));
    (literal; split-once; $input:expr; $sep:expr) => ($input.splitn(2, $sep));
    (literal; strip-prefix; $input:expr; $prefix:expr) => ($input.strip_prefix($prefix));

    (regex; create; $sep:literal) => (::regex::Regex::new($sep).unwrap());
    (regex; split; $input:expr; $sep:expr) => ($sep.split($input));
    (regex; split-once; $input:expr; $sep:expr) => ($sep.splitn($input, 2));
    (regex; strip-prefix; $input:expr; $prefix:expr) => {
        $prefix.find($input).and_then(|m| {
            if m.start() > 0 {
                None
            } else {
                Some(&$input[m.start()..])
            }
        })
    };
}
#[doc(hidden)]
pub use __parse_literal__;

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn parse_singular() {
        parse!("foo" => foo);
        assert_eq!(foo, "foo");
    }

    #[test]
    fn parse_singular_type() {
        parse!("12" => [foo as u8]);
        assert_eq!(foo, 12);
    }

    #[test]
    fn parse_singular_custom() {
        parse!("Hello" => [foo with str::to_lowercase]);
        assert_eq!(foo, "hello");
    }

    #[test]
    fn parse_single_sep() {
        parse!("foo, 12" => foo ", " [bar as u8]);
        assert_eq!(foo, "foo");
        assert_eq!(bar, 12);
    }

    #[test]
    fn parse_multi_sep() {
        parse!("foo, 12, -22" => foo ", " [bar as u8] ", " [baz as i8]);
        assert_eq!(foo, "foo");
        assert_eq!(bar, 12);
        assert_eq!(baz, -22);
    }

    #[test]
    fn parse_skip() {
        parse!("foo, baz, bar" => foo ", " _ ", " bar);
        assert_eq!(foo, "foo");
        assert_eq!(bar, "bar");
    }

    #[test]
    fn parse_multi_literal() {
        parse!("foo, 12" => foo "," " " [bar as u8]);
        assert_eq!(foo, "foo");
        assert_eq!(bar, 12);
    }

    #[test]
    fn parse_leading_literal() {
        parse!("(foo, bar)" => "(" foo ", " bar);
        assert_eq!(foo, "foo");
        assert_eq!(bar, "bar)");
    }

    #[test]
    fn parse_trailing_literal() {
        parse!("(foo, bar)" => foo ", " bar ")");
        assert_eq!(foo, "(foo");
        assert_eq!(bar, "bar");
    }

    #[test]
    fn parse_surrounding_literals() {
        parse!("(foo, bar)" => "(" foo ", " bar ")");
        assert_eq!(foo, "foo");
        assert_eq!(bar, "bar");
    }

    #[test]
    fn parse_leading_literal_regex() {
        parse!("a6" => /"a"/ _v);
    }

    #[test]
    fn parse_trailing_literal_regex() {
        parse!("6aaa" => _v /"a+"/);
    }

    #[test]
    fn parse_type() {
        parse!("1 2" => [foo as u8] " " [bar as usize]);
        assert_eq!(foo, 1);
        assert_eq!(bar, 2);
    }

    #[test]
    fn parse_with_func() {
        parse!("foo" => [name with str::to_uppercase]);
        assert_eq!(name, "FOO");
    }

    // TODO: Fix this.
    // #[test]
    // fn parse_with_nested() {
    //     parse!("1 -1" => [pair with { [l as i8] ' ' [r as i8] } => (l, r)]);
    //     assert_eq!(pair, (1i8, -1i8));
    // }

    #[test]
    fn parse_list() {
        parse!("1 2" => [items split]);
        assert_eq!(items, vec!["1", "2"]);
    }

    #[test]
    fn parse_list_indexed() {
        parse!("a b" => [items split indexed]);
        assert_eq!(items, vec![(0, "a"), (1, "b")]);
    }

    #[test]
    fn parse_list_custom_sep() {
        parse!("1-2" => [items split on "-"]);
        assert_eq!(items, vec!["1", "2"]);
    }

    #[test]
    fn parse_list_custom_sep_regex() {
        parse!("1-2,3" => [items split on /r"\D"/]);
        assert_eq!(items, vec!["1", "2", "3"]);
    }

    #[test]
    fn parse_list_find() {
        parse!("1-2,3" => [items find /r"\d"/]);
        assert_eq!(items, vec!["1", "2", "3"]);
    }

    #[test]
    fn parse_list_capture() {
        parse!("a1,b2!!!c3" => [items capture /r"\w(\d)"/]);
        assert_eq!(
            items
                .into_iter()
                .map(|c| c.get(1).unwrap().as_str())
                .collect::<Vec<_>>(),
            vec!["1", "2", "3"]
        );
    }

    #[test]
    fn parse_list_custom_type() {
        parse!("1 2" => [items split as u8]);
        assert_eq!(items, vec![1, 2]);
    }

    // TODO: Fix this.
    // #[test]
    // fn parse_list_custom_type_char() {
    //     parse!("a b c" => [items split as char]);
    //     assert_eq!(items, vec!['a', 'b', 'c']);
    // }

    #[test]
    fn parse_list_custom_collection() {
        parse!("1 2" => [items split into (HashSet<_>)]);
        assert_eq!(items, HashSet::from(["1", "2"]));
    }

    #[test]
    fn parse_list_iterator() {
        parse!("1 2" => [items split into iterator]);
        assert_eq!(items.next(), Some("1"));
        assert_eq!(items.next(), Some("2"));
        assert_eq!(items.next(), None);
    }

    #[test]
    fn parse_list_to_map() {
        let sub: for<'a> fn(&'a str) -> (&'a str, u8) = |pair| {
            parse!(pair => name "=" [value as u8]);
            (name, value)
        };
        parse!("a=1 b=2" => [items split on " " into (HashMap<_, _>) with sub]);
        assert_eq!(items, HashMap::from([("a", 1), ("b", 2)]));
    }

    #[test]
    fn parse_list_indexed_to_map() {
        let sub: for<'a> fn((usize, &'a str)) -> (&'a str, u8) = |(idx, name)| (name, idx as u8);
        parse!("a b" => [items split on " " indexed into (HashMap<_, _>) with sub]);
        assert_eq!(items, HashMap::from([("a", 0), ("b", 1)]));
    }

    #[test]
    fn parse_list_try_as() {
        parse!("12 angry men" => [items split try as u8]);
        assert_eq!(items, vec![12]);
    }

    #[test]
    fn parse_list_surrounding_literals() {
        parse!("(1 2)" => "(" [items split] ")");
        assert_eq!(items, vec!["1", "2"]);
    }

    #[test]
    fn parse_list_nested_list() {
        parse!("1,2 3,4" => [items split with [split on ',']]);
        assert_eq!(items, vec![vec!["1", "2"], vec!["3", "4"]]);
    }

    #[test]
    fn parse_list_indexed_nested_list() {
        parse!("a,b c,d" => [items split indexed with [split on ',']]);
        assert_eq!(items, vec![(0, vec!["a", "b"]), (1, vec!["c", "d"])]);
    }

    #[test]
    fn parse_list_nested_chars() {
        parse!("12 34" => [items split with [chars]]);
        assert_eq!(items, vec![vec!['1', '2'], vec!['3', '4']]);
    }

    #[test]
    fn parse_list_nested_with_expression() {
        parse!("a=1 b=2" => [items split with { key "=" [value as u8] } => (key, value)]);
        assert_eq!(items, vec![("a", 1), ("b", 2)]);
    }

    #[test]
    fn parse_list_nested_with_expression_nested_list() {
        parse!("1,2 3,4" => [items split with { [pair split on "," as u8] } => pair.into_iter().max().unwrap()]);
        assert_eq!(items, vec![2, 4]);
    }

    #[test]
    fn parse_list_with_transform() {
        parse!("Hello WORLD" => [items split with str::to_lowercase]);
        assert_eq!(items, vec!["hello", "world"]);
    }

    #[test]
    fn parse_chars() {
        parse!("12" => [items chars]);
        assert_eq!(items, vec!['1', '2']);
    }

    #[test]
    fn parse_chars_indexed() {
        parse!("ab" => [items chars indexed]);
        assert_eq!(items, vec![(0, 'a'), (1, 'b')]);
    }

    #[test]
    fn parse_chars_custom_type() {
        parse!("12" => [items chars as u8]);
        assert_eq!(items, vec![1, 2]);
    }

    #[test]
    fn parse_chars_custom_collection() {
        parse!("12" => [items chars into (HashSet<_>)]);
        assert_eq!(items, HashSet::from(['1', '2']));
    }

    #[test]
    fn parse_chars_try_as() {
        parse!("1a2b" => [items chars try as u8]);
        assert_eq!(items, vec![1, 2]);
    }

    #[test]
    fn parse_chars_with_transformer() {
        parse!("1a" => [items chars with |c| c.to_digit(16).unwrap()]);
        assert_eq!(items, vec![1, 10]);
    }

    #[test]
    fn parse_result_expression() {
        let result = parse!("foo bar" => { foo " " bar } => (foo, bar));
        assert_eq!(result, ("foo", "bar"));
    }

    #[test]
    #[should_panic = "unparsed tokens in input"]
    fn parse_fail_incomplete() {
        parse!("1 a" => _ ' ');
    }

    #[test]
    #[should_panic = "couldn't find '-'"]
    fn parse_fail_unmatched_literal() {
        parse!("1 a" => _ '-' _);
    }

    #[test]
    #[should_panic = "couldn't find 'b'"]
    fn parse_fail_unmatched_literal_leading() {
        parse!("a1" => 'b' _);
    }

    #[test]
    #[should_panic = "couldn't find 'b'"]
    fn parse_fail_unmatched_literal_trailing() {
        parse!("1a" => _ 'b');
    }

    #[test]
    #[should_panic = "couldn't find 'd'"]
    fn parse_fail_unmatched_literal_chain_message() {
        parse!("1a2b3c4" => _ 'a' _ 'b' _ 'd' _);
    }

    #[test]
    #[should_panic = "ParseIntError"]
    fn parse_fail_typecast() {
        parse!("a" => [_v as u8]);
    }
}
