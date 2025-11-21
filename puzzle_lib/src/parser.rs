//! Helpers for parsing text into structures
#![doc(hidden)]

/// Things that can be used with the `try` construct in [`parse!`].
pub trait Tryable<T> {
    fn to_option(self) -> Option<T>;
}
impl<T> Tryable<T> for Option<T> {
    #[inline]
    fn to_option(self) -> Option<T> {
        self
    }
}
impl<T, E> Tryable<T> for Result<T, E> {
    #[inline]
    fn to_option(self) -> Option<T> {
        self.ok()
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse {
    ($input:tt => { $($sections:tt)+ } => $result:expr) => {
        {
            $crate::parser::parse!($input => $($sections)+);
            $result
        }
    };
    ($input:tt => $($sections:tt)+) => {
        $crate::parser::__parse__!(@parse; [ input=($input) tmp=(tmp) ]; $($sections)*);
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
/// - An assignment of a fixed size (`[name take num]`). Like the previous item, but instead of
///   taking all the text matched at the given location it only takes the specified number of
///   bytes and leaves the rest for the next section.
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
/// - A combination of previous two (`as type with func`). This will convert to the given
///   type (as per above) and then call the provided function (as per above) with the result of
///   this conversion.
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
///     keep the [`Captures`](regex::Captures).
/// - `indexed` (optional). This will chain [`enumerate`](Iterator::enumerate) on the iterator.
/// - The collection to transform the iterator into (optional, default `into (Vec<_>)`):
///   - `into iterator`. This will keep the result as an [`Iterator`] (the exact type of which
///     depends on the other options).
///   - `into (type)`. This will call [`collect`](Iterator::collect) on the iterator to transform
///     it into the given type.
///   - `try into (type)`. This will first call [`collect`](Iterator::collect) on the iterator to
///     transform it into a [`Vec`], and then it wil call [`TryInto::try_into`] to attemp to
///     convert it into the given type.
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
/// # use puzzle_lib::parser::parse;
/// parse!("hello world" => "hello " subject);
/// assert_eq!(subject, "world");
/// ```
/// ```
/// # use puzzle_lib::parser::parse;
/// parse!("howdy pardner" => greeting /r#" \w+"#/);
/// assert_eq!(greeting, "howdy");
/// ```
/// ```
/// # use puzzle_lib::parser::parse;
/// parse!("g'day mate" => greeting ' ' subject);
/// assert_eq!(greeting, "g'day");
/// assert_eq!(subject, "mate");
/// ```
///
/// Transformations:
///
/// ```
/// # use puzzle_lib::parser::parse;
/// parse!("detonating in 4..." => "detonating in " [seconds as u8] "...");
/// assert_eq!(seconds, 4u8);
/// ```
/// ```
/// # use puzzle_lib::parser::parse;
/// parse!("+12" => [prefix take 1 as char] [num as u8]);
/// assert_eq!(prefix, '+');
/// assert_eq!(num, 12);
/// ```
/// ```
/// # use puzzle_lib::parser::parse;
/// parse!("boo" => [word with str::to_uppercase]);
/// assert_eq!(word, "BOO");
/// ```
/// ```
/// # use puzzle_lib::parser::parse;
/// parse!("sqrt(25)" => "sqrt(" [root as u8 with u8::isqrt] ')');
/// assert_eq!(root, 5);
/// ```
/// ```
/// # use puzzle_lib::parser::parse;
/// //parse!("|1 -1|" => '|' [pair with { [l as i8] ' ' [r as i8] } => (l, r)] '|');
/// //assert_eq!(pair, (1i8, -1i8));
/// ```
///
/// Split:
///
/// ```
/// # use puzzle_lib::parser::parse;
/// parse!("fee-fi-fo-fum" => [words split on '-']);
/// assert_eq!(words, vec!["fee", "fi", "fo", "fum"]);
/// ```
/// ```
/// # use puzzle_lib::parser::parse;
/// parse!("10 -10" => [nums split into iterator as i8]);
/// assert_eq!(nums.next(), Some(10));
/// assert_eq!(nums.next(), Some(-10));
/// assert_eq!(nums.next(), None);
/// ```
/// ```
/// # use std::collections::HashSet;
/// # use puzzle_lib::parser::parse;
/// parse!("n33dl3 in 4 h4yst4ck" => [nums chars into (HashSet<_>) try as u8]);
/// assert_eq!(nums, HashSet::from([3, 4]));
/// ```
/// ```
/// # use puzzle_lib::parser::parse;
/// // parse!("1 2 4 8" => [nums split try into ([u8; 4]) as u8 with u8::reverse_bits]);
/// // assert_eq!(nums, vec![128, 64, 32, 16]);
/// ```
/// ```
/// # use puzzle_lib::parser::parse;
/// parse!("2 fast 2 furious" => [words find /"[a-z]+"/]);
/// assert_eq!(words, vec!["fast", "furious"]);
/// ```
/// ```
/// # use std::collections::HashMap;
/// # use regex::Captures;
/// # use puzzle_lib::parser::parse;
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
///
/// This takes its arguments in the form `@stage; [ key::value ... ]; unprocessed ...`,
/// generally matching things a the start of the unprocessed lists and adding to the key-value list
/// until all arguments are processed, and then using this key-value list to generate an
/// appropriate expression.
///
/// The values will always be capturable in a single token tree to make passing them on unchanged
/// easier, wrapping values in `()` if needed.
///
/// Types will often be passed as `key=(Type)` and will be captured as `key=($($type:tt)+)`. This
/// is done because the `ty` fragment type cannot be matched on, so we only capture as `ty` if we
/// will emit the token as-is, but not if we need to do any matching on it.
#[doc(hidden)]
#[macro_export]
macro_rules! __parse__ {
    // Leading literal.
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; $prefix:literal $($rest:tt)*) => {
        $crate::parser::__parse__!(@literal; [ input=$input tmp=$tmp kind=literal ]; $prefix $($rest)*);
    };
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; /$prefix:literal/ $($rest:tt)*) => {
        $crate::parser::__parse__!(@literal; [ input=$input tmp=$tmp kind=regex ]; $prefix $($rest)*);
    };
    (@literal; [ input=$input:tt tmp=($($tmp:ident)+) kind=$kind:ident ]; $prefix:literal $($rest:tt)*) => {
        ::paste::paste!{
            let [< $($tmp)+ _input >] = $input;
            let [< $($tmp)+ _prefix >] = $crate::parser::__parse_literal__!(kind=$kind action=create value=$prefix);
            let [< $($tmp)+ _stripped >] = $crate::parser::__parse_literal__!(
                kind=$kind
                action=strip_prefix
                input=[< $($tmp)+ _input >]
                prefix=[< $($tmp)+ _prefix >]
            );
        };
        $crate::parser::__parse__!(@parse; [ input=(::paste::paste!([< $($tmp)+ _stripped >]).ok_or_else(|| {
            format!(
                "couldn't find {:?} at the start of {:?}",
                ::paste::paste!([< $($tmp)+ _prefix >]),
                ::paste::paste!([< $($tmp)+ _input >]),
            )
        }).unwrap()) tmp=($($tmp)+) ]; $($rest)*);
    };

    // Infix/trailing literal.
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; $first:tt $sep:literal $($rest:tt)*) => {
        $crate::parser::__parse__!(@literal; [ input=$input tmp=$tmp kind=literal ]; $first $sep $($rest)*);
    };
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; $first:tt /$sep:literal/ $($rest:tt)*) => {
        $crate::parser::__parse__!(@literal; [ input=$input tmp=$tmp kind=regex ]; $first $sep $($rest)*);
    };
    (@literal; [ input=$input:tt tmp=($($tmp:ident)+) kind=$kind:ident ]; $first:tt $sep:literal $($rest:tt)*) => {
        ::paste::paste!{
            let [< $($tmp)+ _input >] = $input;
            let [< $($tmp)+ _sep >] = $crate::parser::__parse_literal__!(kind=$kind action=create value=$sep);
            let mut [< $($tmp)+ >] = $crate::parser::__parse_literal__!(
                kind=$kind
                action=split_once
                input=[< $($tmp)+ _input >]
                sep=[< $($tmp)+ _sep >]
            );
        };
        $crate::parser::__parse__!(@parse; [ input=(::paste::paste!([< $($tmp)+ >]).next().unwrap()) tmp=($($tmp)+ _1) ]; $first);
        $crate::parser::__parse__!(@parse; [ input=(::paste::paste!([< $($tmp)+ >]).next().ok_or_else(|| {
            format!(
                "couldn't find {:?} in {:?}",
                ::paste::paste!([< $($tmp)+ _sep >]),
                ::paste::paste!([< $($tmp)+ _input >]),
            )
        }).unwrap()) tmp=($($tmp)+ _2) ]; $($rest)*);
    };

    // End of instructions.
    (@parse; [ input=$input:tt tmp=$tmp:tt ];) => {
        assert_eq!($input, "", "unparsed tokens in input");
    };

    // Ignore element.
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; _) => {
        let _ = $input;
    };

    // Fixed-length item.
    (@parse; [ input=$input:tt tmp=($($tmp:ident)+) ]; [$name:ident take $num:literal $($restinner:tt)*] $($rest:tt)*) => {
        ::paste::paste!{
            let [< $($tmp)+ _input >] = $input;
            let [< $($tmp)+ >] = [< $($tmp)+ _input >].split_at_checked($num).ok_or_else(|| {
                format!(
                    "couldn't take {} bytes from {:?}",
                    $num,
                    ::paste::paste!([< $($tmp)+ _input >]),
                )
            }).unwrap();
        };
        $crate::parser::__parse__!(@parse; [ input=(::paste::paste!([< $($tmp)+ >]).0) tmp=($($tmp)+ _1) ]; [$name $($restinner)*]);
        $crate::parser::__parse__!(@parse; [ input=(::paste::paste!([< $($tmp)+ >]).1) tmp=($($tmp)+ _2) ]; $($rest)*);
    };

    // Store element as identifier.
    // [
    //   $name
    //   (take $num)?
    //   (
    //      as $type ||
    //      with $transformer ||
    //      as $type with $transformer ||
    //      with [{ nested } => result]
    //   )
    // ]
    // name; $type will be &str

    (@parse; [ input=$input:tt tmp=$tmp:tt ]; [ $ident:ident as $type:tt ]) => {
        let $ident = $crate::parser::__parse_type__!($input => str => $type);
    };
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; [ $ident:ident with $transformer:expr ]) => {
        let $ident = $transformer($input);
    };
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; [ $ident:ident as $type:tt with $transformer:expr ]) => {
        let $ident = $transformer($crate::parser::__parse_type__!($input => str => $type));
    };
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; [ $ident:ident with { $($nested:tt)+ } => $result:expr ]) => {
        let $ident = $crate::parser::parse!($input => { $($nested)+ } => $result);
    };
    (@parse; $args:tt; $ident:ident) => {
        $crate::parser::__parse__!(@parse; $args; [ $ident as str ]);
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
    //      try? into ($collection);
    //      default $collection Vec
    //   )
    //   (
    //      try? as $type ||
    //      with [nested-bracketed] ||
    //      with { nested } => result ||
    //      try? with $transformer ||
    //      try? as $type with $transformer;
    //      default $type &str)
    //   )
    // ]

    // chars
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; [ $ident:ident chars $($rest:tt)* ]) => {
        #[allow(unused_mut)]
        let mut $ident = $crate::parser::__parse__!(@splitp; [ input=$input item=(char) sel=(chars) ]; $($rest)*);
    };
    // find /"regex"/
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; [ $ident:ident find /$pattern:literal/ $($rest:tt)* ]) => {
        #[allow(unused_mut)]
        let mut $ident = $crate::parser::__parse__!(@splitp; [ input=$input item=(str) sel=(find $pattern) ]; $($rest)*);
    };
    // capture /"regex"/
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; [ $ident:ident capture /$pattern:literal/ $($rest:tt)* ]) => {
        #[allow(unused_mut)]
        let mut $ident = $crate::parser::__parse__!(@splitp; [ input=$input item=(str) sel=(capture $pattern) ]; $($rest)*);
    };
    // split
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; [ $ident:ident split $($rest:tt)* ]) => {
        #[allow(unused_mut)]
        let mut $ident = $crate::parser::__parse__!(@splitp; [ input=$input item=(str) ]; $($rest)*);
    };
    // on $sep
    (@splitp; [ input=$input:tt item=$item:tt ]; on /$sep:literal/ $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input item=$item sel=(on regex $sep) ]; $($rest)*)
    };
    (@splitp; [ input=$input:tt item=$item:tt ]; on $sep:literal $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input item=$item sel=(on literal $sep) ]; $($rest)*)
    };
    (@splitp; [ input=$input:tt item=$item:tt ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input item=$item sel=(on literal " ") ]; $($rest)*)
    };
    // indexed
    (@splitp; [ input=$input:tt item=$item:tt sel=$sel:tt ]; indexed $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input item=$item sel=$sel into=(indexed) ]; $($rest)*)
    };
    (@splitp; [ input=$input:tt item=$item:tt sel=$sel:tt ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input item=$item sel=$sel into=() ]; $($rest)*)
    };
    // (try?) into $collection
    (@splitp; [ input=$input:tt item=$item:tt sel=$sel:tt into=($($flags:ident)*) ]; into iterator $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input item=$item sel=$sel into=($($flags)*; Iterator) ]; $($rest)*)
    };
    (@splitp; [ input=$input:tt item=$item:tt sel=$sel:tt into=($($flags:ident)*) ]; try into ($collection:ty) $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input item=$item sel=$sel into=($($flags)*; try $collection) ]; $($rest)*)
    };
    (@splitp; [ input=$input:tt item=$item:tt sel=$sel:tt into=($($flags:ident)*) ]; into ($collection:ty) $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input item=$item sel=$sel into=($($flags)*; $collection) ]; $($rest)*)
    };
    (@splitp; [ input=$input:tt item=$item:tt sel=$sel:tt into=($($flags:ident)*) ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input item=$item sel=$sel into=($($flags)*; Vec<_>) ]; $($rest)*)
    };
    // (try)? as $type
    (@splitp; [ input=$input:tt item=($($item:tt)+) sel=$sel:tt into=$into:tt ]; as $type:tt $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input item=$type sel=$sel into=$into with1=(
            |item| $crate::parser::__parse_type__!(item => $($item)+ => $type)
        ) ]; $($rest)*)
    };
    (@splitp; [ input=$input:tt item=($($item:tt)+) sel=$sel:tt into=$into:tt ]; try as $type:tt $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input item=(Option<$type>) sel=$sel into=$into with1=(try
            |item| $crate::parser::__parse_type__!(item => $($item)+ => try $type)
        ) ]; $($rest)*)
    };
    // with [nested-bracketed]
    (@splitp; [ input=$input:tt item=$item:tt sel=$sel:tt into=$into:tt ]; with [ $($nested:tt)+ ]) => {
        $crate::parser::__parse__!(@splitp; [ input=$input item=$item sel=$sel into=$into with1=(
            |item| {
                $crate::parser::__parse__!(@parse; [ input=item tmp=(tmp) ]; [ result $($nested)+ ]);
                result
            }
        ) ];)
    };
    // with { nested } => result
    (@splitp; [ input=$input:tt item=$item:tt sel=$sel:tt into=$into:tt ]; with { $($nested:tt)+ } => $result:expr) => {
        $crate::parser::__parse__!(@splitp; [ input=$input item=$item sel=$sel into=$into with1=(
            |item| $crate::parser::parse!(item => { $($nested)+ } => $result)
        ) ];)
    };
    // (try)? with $transformer
    (@splitp; [ input=$input:tt item=$item:tt sel=$sel:tt into=$into:tt ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input item=$item sel=$sel into=$into with1=() ]; $($rest)*)
    };
    (@splitp; [ input=$input:tt item=$item:tt sel=$sel:tt into=$into:tt with1=$with1:tt ]; with $transformer:expr) => {
        $crate::parser::__parse__!(@splitp; [ input=$input item=$item sel=$sel into=$into with1=$with1 with2=($transformer) ];)
    };
    (@splitp; [ input=$input:tt item=$item:tt sel=$sel:tt into=$into:tt with1=$with1:tt ]; try with $transformer:expr) => {
        $crate::parser::__parse__!(@splitp; [ input=$input item=$item sel=$sel into=$into with1=$with1 with2=(try $transformer) ];)
    };
    (@splitp; [ input=$input:tt item=$item:tt sel=$sel:tt into=$into:tt with1=$with1:tt ];) => {
        $crate::parser::__parse__!(@splitp; [ input=$input item=$item sel=$sel into=$into with1=$with1 with2=() ];)
    };

    // done. work backwards by repeatedly transforming some portion of the definition into a
    // chained method call
    (@splitp; [ input=$input:tt item=$item:tt $($rest:tt)* ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input item=$item $($rest)* ];)
    };
    // convert to collection (or not)
    (@splitf; [ input=$input:tt item=$item:tt sel=$sel:tt into=($($flags:ident)*; Iterator) $($rest:tt)* ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input item=$item sel=$sel into=($($flags)*) $($rest)* ];)
    };
    (@splitf; [ input=$input:tt item=$item:tt sel=$sel:tt into=($($flags:ident)*; try $collection:ty) $($rest:tt)* ];) => {
        {
            let value = $crate::parser::__parse__!(@splitf; [ input=$input item=$item sel=$sel into=($($flags)*) $($rest)* ];).collect::<Vec<_>>();
            let value: $collection = value.try_into().unwrap();
            value
        }
    };
    (@splitf; [ input=$input:tt item=$item:tt sel=$sel:tt into=($($flags:ident)*; $collection:ty) $($rest:tt)* ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input item=$item sel=$sel into=($($flags)*) $($rest)* ];).collect::<$collection>()
    };
    // second with (which is the custom transform function and explicitly happens after the indexed flag when present)
    (@splitf; [ input=$input:tt item=$item:tt sel=$sel:tt into=$into:tt with1=$with1:tt with2=(try $transformer:expr) ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input item=$item sel=$sel into=$into with1=$with1 with2=($transformer) ];).filter_map($crate::parser::Tryable::to_option)
    };
    (@splitf; [ input=$input:tt item=$item:tt sel=$sel:tt into=$into:tt with1=$with1:tt with2=($transformer:expr) ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input item=$item sel=$sel into=$into with1=$with1 ];).map($transformer)
    };
    (@splitf; [ input=$input:tt item=$item:tt sel=$sel:tt into=$into:tt with1=$with1:tt with2=() ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input item=$item sel=$sel into=$into with1=$with1 ];)
    };
    // indexed flag
    (@splitf; [ input=$input:tt item=$item:tt sel=$sel:tt into=(indexed) $($rest:tt)* ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input item=$item sel=$sel $($rest)* ];).enumerate()
    };
    (@splitf; [ input=$input:tt item=$item:tt sel=$sel:tt into=() $($rest:tt)* ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input item=$item sel=$sel $($rest)* ];)
    };
    // first with
    (@splitf; [ input=$input:tt item=$item:tt sel=$sel:tt with1=(try $transformer:expr) ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input item=$item sel=$sel with1=($transformer) ];).filter_map($crate::parser::Tryable::to_option)
    };
    (@splitf; [ input=$input:tt item=$item:tt sel=$sel:tt with1=($transformer:expr) ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input item=$item sel=$sel ];).map($transformer)
    };
    (@splitf; [ input=$input:tt item=$item:tt sel=$sel:tt with1=() ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input item=$item sel=$sel ];)
    };
    // the initial split
    (@splitf; [ input=$input:tt item=$item:tt sel=(chars) ];) => {
        $input.chars()
    };
    (@splitf; [ input=$input:tt item=$item:tt sel=(on $sepkind:ident $sep:literal) ];) => {
        $crate::parser::__parse_literal__!(
            kind=$sepkind
            action=split
            input=$input
            sep=$crate::parser::__parse_literal__!(kind=$sepkind action=create value=$sep)
        )
    };
    (@splitf; [ input=$input:tt item=$item:tt sel=(find $pattern:literal) ];) => {
        ::regex::Regex::new($pattern).unwrap().find_iter($input).map(|m| m.as_str())
    };
    (@splitf; [ input=$input:tt item=$item:tt sel=(capture $pattern:literal) ];) => {
        ::regex::Regex::new($pattern).unwrap().captures_iter($input)
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
            Ok(t.chars().next().unwrap())
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
        t.to_digit(10).ok_or_else(|| format!("cannot convert character {t:?} to a number"))
    });
    ($var:expr => char => try usize) => ($crate::parser::__parse_type__!($var => char to digit).map(|v| v as usize));
    ($var:expr => char => try u128) => ($crate::parser::__parse_type__!($var => char to digit).map(|v| v as u128));
    ($var:expr => char => try u64) => ($crate::parser::__parse_type__!($var => char to digit).map(|v| v as u64));
    ($var:expr => char => try u32) => ($crate::parser::__parse_type__!($var => char to digit));
    ($var:expr => char => try u16) => ($crate::parser::__parse_type__!($var => char to digit).map(|v| v as u16));
    ($var:expr => char => try u8) => ($crate::parser::__parse_type__!($var => char to digit).map(|v| v as u8));
    ($var:expr => char => try isize) => ($crate::parser::__parse_type__!($var => char to digit).map(|v| v as isize));
    ($var:expr => char => try i128) => ($crate::parser::__parse_type__!($var => char to digit).map(|v| v as i128));
    ($var:expr => char => try i64) => ($crate::parser::__parse_type__!($var => char to digit).map(|v| v as i64));
    ($var:expr => char => try i32) => ($crate::parser::__parse_type__!($var => char to digit).map(|v| v as i32));
    ($var:expr => char => try i16) => ($crate::parser::__parse_type__!($var => char to digit => char to digit));
    ($var:expr => char => try i8) => ($crate::parser::__parse_type__!($var => char to digit).map(|v| v as i8));
    ($var:expr => char => try f64) => ($crate::parser::__parse_type__!($var => char to digit).map(|v| v as f64));
    ($var:expr => char => try f32) => ($crate::parser::__parse_type__!($var => char to digit).map(|v| v as f32));

    ($var:expr => $from:tt => $to:tt) => {
        $crate::parser::__parse_type__!($var => $from => try $to).unwrap()
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
    (kind=literal action=create value=$value:literal) => ($value);
    (kind=literal action=split input=$input:tt sep=$sep:expr) => ($input.split($sep));
    (kind=literal action=split_once input=$input:tt sep=$sep:expr) => ($input.splitn(2, $sep));
    (kind=literal action=strip_prefix input=$input:tt prefix=$prefix:expr) => ($input.strip_prefix($prefix));

    (kind=regex action=create value=$value:literal) => (::regex::Regex::new($value).unwrap());
    (kind=regex action=split input=$input:tt sep=$sep:expr) => ($sep.split($input));
    (kind=regex action=split_once input=$input:tt sep=$sep:expr) => ($sep.splitn($input, 2));
    (kind=regex action=strip_prefix input=$input:tt prefix=$prefix:expr) => {
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
    fn parse_take() {
        parse!("+12" => [prefix take 1 as char] [num as u8]);
        assert_eq!(prefix, '+');
        assert_eq!(num, 12);
    }

    #[test]
    #[should_panic = "couldn't take 10 bytes"]
    fn parse_take_err() {
        parse!("hello" => [_name take 10 as char]);
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
    #[should_panic = "couldn't find"]
    fn parse_leading_literal_regex_mismatch() {
        parse!("1a6" => /"a"/ _v);
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

    #[test]
    fn parse_type_and_with_func() {
        parse!("25" => [num as u8 with u8::isqrt]);
        assert_eq!(num, 5);
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

    #[test]
    fn parse_list_custom_type_char() {
        parse!("a b c" => [items split as char]);
        assert_eq!(items, vec!['a', 'b', 'c']);
    }

    #[test]
    fn parse_list_custom_collection() {
        parse!("1 2" => [items split into (HashSet<_>)]);
        assert_eq!(items, HashSet::from(["1", "2"]));
    }

    #[test]
    fn parse_list_custom_try_collection() {
        parse!("1 2" => [items split try into ([&str; 2])]);
        assert_eq!(items, ["1", "2"]);
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
    fn parse_list_with_try_transform_option() {
        let transformer = |value: &str| {
            if value == value.to_uppercase() {
                Some(value.len())
            } else {
                None
            }
        };
        parse!("Hello WORLD" => [items split try with transformer]);
        assert_eq!(items, vec![5]);
    }

    #[test]
    fn parse_list_with_try_transform_result() {
        parse!("foo 12" => [items split try with str::parse::<u8>]);
        assert_eq!(items, vec![12]);
    }

    #[test]
    fn parse_list_type_and_with_transform() {
        parse!("9 25" => [items split as u8 with u8::isqrt]);
        assert_eq!(items, vec![3, 5]);
    }

    #[test]
    fn parse_list_try_type_and_try_with_transform() {
        parse!("9 foo 25 140" => [items split try as u8 try with u8::checked_next_power_of_two]);
        assert_eq!(items, vec![16, 32]);
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
        parse!("12" => [items chars as i8]);
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
