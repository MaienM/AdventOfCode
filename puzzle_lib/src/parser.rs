//! A macro ([`parse!`]) to parse text into structures.

pub use regex::{Captures, Regex};

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

#[doc = include_str!("./parser.md")]
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

    // Store element as identifier.
    // [
    //   $name
    //   (
    //      take $num ||
    //      matching /$pattern/ ||
    //      capturing /$pattern/
    //   )?
    //   (
    //      as $type ||
    //      with $transformer ||
    //      as $type with $transformer ||
    //      with [{ nested } => result]
    //   )
    // ]
    // name; $type will be &str

    // $name
    (@parse; $args:tt; $ident:ident) => {
        $crate::parser::__parse__!(@parse; $args; [ $ident ]);
    };
    // [$name]
    (@parse; [ input=$input:tt tmp=$tmp:tt $(itype=$itype:tt)? ]; [$ident:ident]) => {
        let $ident = $input;
    };
    // [$name take $num ...]
    (@parse; [ input=$input:tt tmp=($($tmp:ident)+) ]; [$name:ident take $num:literal $($restinner:tt)*] $($rest:tt)*) => {
        ::paste::paste!{
            let [< $($tmp)+ _input >] = $input;
            let [< $($tmp)+ >] = [< $($tmp)+ _input >].split_at_checked($num).ok_or_else(|| {
                format!("couldn't take {} bytes from {:?}", $num, [< $($tmp)+ _input >])
            }).unwrap();
        };
        $crate::parser::__parse__!(@parse; [ input=(::paste::paste!([< $($tmp)+ >]).0) tmp=($($tmp)+ _1) ]; [$name $($restinner)*]);
        $crate::parser::__parse__!(@parse; [ input=(::paste::paste!([< $($tmp)+ >]).1) tmp=($($tmp)+ _2) ]; $($rest)*);
    };
    // [$name matching /$pattern/ ...]
    (@parse; [ input=$input:tt tmp=($($tmp:ident)+) ]; [$name:ident matching /$pattern:literal/ $($restinner:tt)*] $($rest:tt)*) => {
        ::paste::paste!{
            let [< $($tmp)+ _input >] = $input;
            let [< $($tmp)+ _regex >] = $crate::parser::Regex::new($pattern).unwrap();
            let [< $($tmp)+ _match >] = [< $($tmp)+ _regex >].find([< $($tmp)+ _input >]).ok_or_else(|| {
                format!("couldn't match {} at start of {:?}", stringify!($pattern), [< $($tmp)+ _input >])
            }).unwrap();
            assert!(
                [< $($tmp)+ _match >].start() == 0,
                "couldn't match {} at start of {:?}", stringify!($pattern), [< $($tmp)+ _input >],
            );
            $crate::parser::__parse__!(@parse; [ input=([< $($tmp)+ _match >].as_str()) tmp=($($tmp)+ _1) ]; [$name $($restinner)*]);
            $crate::parser::__parse__!(@parse; [ input=([< $($tmp)+ _input >].get([< $($tmp)+ _match >].end()..).unwrap()) tmp=($($tmp)+ _2) ]; $($rest)*);
        };
    };
    // [$name capturing /$pattern/ ...]
    (@parse; [ input=$input:tt tmp=($($tmp:ident)+) ]; [$name:ident capturing /$pattern:literal/ $($restinner:tt)*] $($rest:tt)*) => {
        ::paste::paste!{
            let [< $($tmp)+ _input >] = $input;
            let [< $($tmp)+ _regex >] = $crate::parser::Regex::new($pattern).unwrap();
            let [< $($tmp)+ _capture >] = [< $($tmp)+ _regex >].captures([< $($tmp)+ _input >]).ok_or_else(|| {
                format!("couldn't capture {} at start of {:?}", stringify!($pattern), [< $($tmp)+ _input >])
            }).unwrap();
            let [< $($tmp)+ _match >] = [< $($tmp)+ _capture >].get_match();
            assert!(
                [< $($tmp)+ _match >].start() == 0,
                "couldn't capture {} at start of {:?}", stringify!($pattern), [< $($tmp)+ _input >],
            );
            $crate::parser::__parse__!(@parse; [ input=([< $($tmp)+ _capture >]) tmp=($($tmp)+ _1) itype=($crate::parser::Captures) ]; [$name $($restinner)*]);
            $crate::parser::__parse__!(@parse; [ input=([< $($tmp)+ _input >].get([< $($tmp)+ _match >].end()..).unwrap()) tmp=($($tmp)+ _2) ]; $($rest)*);
        };
    };
    // if input type unspecified -> str
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; [ $ident:ident as $type:tt $($rest:tt)* ]) => {
        $crate::parser::__parse__!(@parse; [ input=$input tmp=$tmp itype=(str) ]; [ $ident as $type $($rest)* ]);
    };
    // [... as $type]
    (@parse; [ input=$input:tt tmp=$tmp:tt itype=($($itype:tt)+) ]; [ $ident:ident as $type:tt ]) => {
        let $ident = $crate::parser::__parse_type__!($input => $($itype)+ => $type);
    };
    // [... with $transformer]
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; [ $ident:ident with $transformer:expr ]) => {
        let $ident = $transformer($input);
    };
    // [... as $type with $transformer]
    (@parse; [ input=$input:tt tmp=$tmp:tt itype=($($itype:tt)+) ]; [ $ident:ident as $type:tt with $transformer:expr ]) => {
        let $ident = $transformer($crate::parser::__parse_type__!($input => $($itype)+ => $type));
    };
    // [... with { nested } => result]
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; [ $ident:ident with { $($nested:tt)+ } => $result:expr ]) => {
        let $ident = $crate::parser::parse!($input => { $($nested)+ } => $result);
    };
    // match
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; [ $ident:ident match $match:tt ]) => {
        let $ident = $crate::parser::__parse__!(@matchp; [ next=parse input=$input matchflag=() ]; match $match);
    };
    (@parse; [ match=$match:tt indexes=() input=$input:tt matchflag=() ];) => {
        $crate::parser::__parse__!(@match_body; [ match=$match indexing=none type=((), (&str)) input=$input matchflag=() ];)
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
    //      try? match { ... } ||
    //      try? with $transformer ||
    //      try? as $type try? match { ... } ||
    //      try? as $type with $transformer;
    //      default $type &str
    //   )
    // ]

    // Note that there are 3 possible values for the indexing flag:
    // - `explicit`: The user explictly requested indexing, and the (index, value) tuple will be
    //   used as value for any further expressions/transformations.
    // - `implicit`: The user used one of the index storing statements in the match body, and thus
    //   we need to iterate with index, but the matched value & and further
    //   expressions/transformations will continue to use the value without index.
    // - `none`: The used did not request indexing & the match body doesn't require it, so we do
    //   not need to iterate with index.

    // chars
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; [ $ident:ident chars $($rest:tt)* ]) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=(char) sel=(chars) ]; $($rest)*);
    };
    // find matches /"regex"/
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; [ $ident:ident find matches /$pattern:literal/ $($rest:tt)* ]) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=(str) sel=(match $pattern) ]; $($rest)*);
    };
    // find captures /"regex"/
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; [ $ident:ident find captures /$pattern:literal/ $($rest:tt)* ]) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=($crate::parser::Captures) sel=(capture $pattern) ]; $($rest)*);
    };
    // split
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; [ $ident:ident split $($rest:tt)* ]) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=(str) ]; $($rest)*);
    };
    // on $sep
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt ]; on /$sep:literal/ $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=$item sel=(on regex $sep) ]; $($rest)*)
    };
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt ]; on $sep:literal $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=$item sel=(on literal $sep) ]; $($rest)*)
    };
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=$item sel=(on literal " ") ]; $($rest)*)
    };
    // indexed
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt sel=$sel:tt ]; indexed $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=$item sel=$sel indexing=explicit ]; $($rest)*)
    };
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt sel=$sel:tt ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=$item sel=$sel indexing=none ]; $($rest)*)
    };
    // (try?) into $collection
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt sel=$sel:tt indexing=$indexing:tt ]; into iterator $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=$item sel=$sel indexing=$indexing into=(Iterator) ]; $($rest)*)
    };
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt sel=$sel:tt indexing=$indexing:tt ]; try into ($collection:ty) $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=$item sel=$sel indexing=$indexing into=(try $collection) ]; $($rest)*)
    };
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt sel=$sel:tt indexing=$indexing:tt ]; into ($collection:ty) $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=$item sel=$sel indexing=$indexing into=($collection) ]; $($rest)*)
    };
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt sel=$sel:tt indexing=$indexing:tt ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=$item sel=$sel indexing=$indexing into=(Vec<_>) ]; $($rest)*)
    };
    // try as $type (try)? match
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt sel=$sel:tt indexing=$indexing:ident into=$into:tt ]; try as $type:tt match $match:tt $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=splitp input=$input ident=$ident item=$item sel=$sel indexing=$indexing into=$into trytype=$type matchflag=() ]; match $match $($rest)*)
    };
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt sel=$sel:tt indexing=$indexing:ident into=$into:tt ]; try as $type:tt try match $match:tt $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=splitp input=$input ident=$ident item=$item sel=$sel indexing=$indexing into=$into trytype=$type matchflag=(try) ]; match $match $($rest)*)
    };
    (@splitp; [ match=$match:tt indexes=$indexes:tt input=$input:tt ident=$ident:tt item=($($item:tt)+) sel=$sel:tt indexing=$indexing:ident into=$into:tt trytype=$type:tt matchflag=($($matchflag:ident)?) ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=(_) sel=$sel indexing=$indexing into=$into with1=(
            |item| $crate::parser::Tryable::to_option($crate::parser::__parse_type__!(item => $($item)+ => try $type)).ok_or(item)
        ) indexes=$indexes with2=($($matchflag)?
            |item| $crate::parser::__parse__!(@match_body; [ match=$match indexing=$indexing type=((usize), (Result<$($item)+, $type>)) input=(item) matchflag=($($matchflag)?) ];)
        ) ]; $($rest)*)
    };
    // (try)? as $type
    (@splitp; [ input=$input:tt ident=$ident:tt item=($($item:tt)+) sel=$sel:tt indexing=$indexing:tt into=$into:tt ]; as $type:tt $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=($type) sel=$sel indexing=$indexing into=$into with1=(
            |item| $crate::parser::__parse_type__!(item => $($item)+ => $type)
        ) ]; $($rest)*)
    };
    (@splitp; [ input=$input:tt ident=$ident:tt item=($($item:tt)+) sel=$sel:tt indexing=$indexing:tt into=$into:tt ]; try as $type:tt $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=(Option<$type>) sel=$sel indexing=$indexing into=$into with1=(try
            |item| $crate::parser::__parse_type__!(item => $($item)+ => try $type)
        ) ]; $($rest)*)
    };
    // with [nested-bracketed]
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt sel=$sel:tt indexing=$indexing:tt into=$into:tt ]; with [ $($nested:tt)+ ]) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=$item sel=$sel indexing=$indexing into=$into with1=(
            |item| {
                $crate::parser::__parse__!(@parse; [ input=item tmp=(tmp) ]; [ result $($nested)+ ]);
                result
            }
        ) ];)
    };
    // with { nested } => result
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt sel=$sel:tt indexing=$indexing:tt into=$into:tt ]; with { $($nested:tt)+ } => $result:expr) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=$item sel=$sel indexing=$indexing into=$into with1=(
            |item| $crate::parser::parse!(item => { $($nested)+ } => $result)
        ) ];)
    };
    // (try)? with $transformer
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt sel=$sel:tt indexing=$indexing:tt into=$into:tt ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=$item sel=$sel indexing=$indexing into=$into with1=() ]; $($rest)*)
    };
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt sel=$sel:tt indexing=$indexing:tt into=$into:tt with1=$with1:tt ]; with $transformer:expr) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=$item sel=$sel indexing=$indexing into=$into with1=$with1 indexes=() with2=($transformer) ];)
    };
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt sel=$sel:tt indexing=$indexing:tt into=$into:tt with1=$with1:tt ]; try with $transformer:expr) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=$item sel=$sel indexing=$indexing into=$into with1=$with1 indexes=() with2=(try $transformer) ];)
    };
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt sel=$sel:tt indexing=$indexing:tt into=$into:tt with1=$with1:tt ];) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=$item sel=$sel indexing=$indexing into=$into with1=$with1 indexes=() with2=() ];)
    };
    // (try)? match
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt sel=$sel:tt indexing=$indexing:tt into=$into:tt with1=$with1:tt ]; match $match:tt $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=splitp input=$input ident=$ident item=$item sel=$sel indexing=$indexing into=$into with1=$with1 matchflag=() ]; match $match $($rest)*)
    };
    (@splitp; [ input=$input:tt ident=$ident:tt item=$item:tt sel=$sel:tt indexing=$indexing:tt into=$into:tt with1=$with1:tt ]; try match $match:tt $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=splitp input=$input ident=$ident item=$item sel=$sel indexing=$indexing into=$into with1=$with1 matchflag=(try) ]; match $match $($rest)*)
    };
    (@splitp; [ match=$match:tt indexes=$indexes:tt input=$input:tt ident=$ident:tt item=($($item:tt)+) sel=$sel:tt indexing=$indexing:ident into=$into:tt with1=$with1:tt matchflag=($($matchflag:ident)?) ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@splitp; [ input=$input ident=$ident item=(_) sel=$sel indexing=$indexing into=$into with1=$with1 indexes=$indexes with2=($($matchflag)?
            |item| $crate::parser::__parse__!(@match_body; [ match=$match indexing=$indexing type=((usize), ($($item)+)) input=(item) matchflag=($($matchflag)?) ];)
        ) ]; $($rest)*)
    };
    // combining into iterator with a match that captures indexes doesn't work as expected since
    // the capture will only happen once the iterator is consumed, so prevent this case
    (@splitp; [ input=$input:tt ident=$ident:tt item=($($item:tt)+) sel=$sel:tt indexing=$indexing:ident into=(Iterator) with1=$with1:tt indexes=($($indexes:tt)+) $($args:tt)* ]; $($rest:tt)*) => {
        compile_error!("Cannot combine `into iterator` with a `match` that captures indexes.");
    };

    // done. work backwards by repeatedly transforming some portion of the definition into a
    // chained method call
    (@splitp; [ input=$input:tt ident=$ident:ident item=$item:tt sel=$sel:tt indexing=$indexing:tt into=$into:tt with1=$with1:tt indexes=$indexes:tt $($rest:tt)* ];) => {
        $crate::parser::__parse__!(@match_define_vars; [ indexes=$indexes itype=(usize) ];);
        #[allow(unused_mut)]
        let mut $ident = $crate::parser::__parse__!(@splitf; [ input=$input sel=$sel indexing=$indexing into=$into with1=$with1 $($rest)* ];);
        $crate::parser::__parse__!(@match_validate_vars; [ indexes=$indexes ];);
    };
    // convert to collection (or not)
    (@splitf; [ input=$input:tt sel=$sel:tt indexing=$indexing:tt into=(Iterator) $($rest:tt)* ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input sel=$sel indexing=$indexing $($rest)* ];)
    };
    (@splitf; [ input=$input:tt sel=$sel:tt indexing=$indexing:tt into=(try $collection:ty) $($rest:tt)* ];) => {
        {
            let value = $crate::parser::__parse__!(@splitf; [ input=$input sel=$sel indexing=$indexing $($rest)* ];).collect::<Vec<_>>();
            let value: $collection = value.try_into().unwrap();
            value
        }
    };
    (@splitf; [ input=$input:tt sel=$sel:tt indexing=$indexing:tt into=($collection:ty) $($rest:tt)* ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input sel=$sel indexing=$indexing $($rest)* ];).collect::<$collection>()
    };
    // second with (which is the custom transform function and explicitly happens after the indexed flag when present)
    (@splitf; [ input=$input:tt sel=$sel:tt indexing=$indexing:tt with1=$with1:tt with2=(try $transformer:expr) ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input sel=$sel indexing=$indexing with1=$with1 with2=($transformer) ];).filter_map($crate::parser::Tryable::to_option)
    };
    (@splitf; [ input=$input:tt sel=$sel:tt indexing=$indexing:tt with1=$with1:tt with2=($transformer:expr) ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input sel=$sel indexing=$indexing with1=$with1 ];).map($transformer)
    };
    (@splitf; [ input=$input:tt sel=$sel:tt indexing=$indexing:tt with1=$with1:tt with2=() ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input sel=$sel indexing=$indexing with1=$with1 ];)
    };
    // indexing
    (@splitf; [ input=$input:tt sel=$sel:tt indexing=none $($rest:tt)* ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input sel=$sel $($rest)* ];)
    };
    (@splitf; [ input=$input:tt sel=$sel:tt indexing=$indexing:tt $($rest:tt)* ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input sel=$sel $($rest)* ];).enumerate()
    };
    // first with
    (@splitf; [ input=$input:tt sel=$sel:tt with1=(try $transformer:expr) ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input sel=$sel with1=($transformer) ];).filter_map($crate::parser::Tryable::to_option)
    };
    (@splitf; [ input=$input:tt sel=$sel:tt with1=($transformer:expr) ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input sel=$sel ];).map($transformer)
    };
    (@splitf; [ input=$input:tt sel=$sel:tt with1=() ];) => {
        $crate::parser::__parse__!(@splitf; [ input=$input sel=$sel ];)
    };
    // the initial split
    (@splitf; [ input=$input:tt sel=(chars) ];) => {
        $input.chars()
    };
    (@splitf; [ input=$input:tt sel=(on $sepkind:ident $sep:literal) ];) => {
        $crate::parser::__parse_literal__!(
            kind=$sepkind
            action=split
            input=$input
            sep=$crate::parser::__parse_literal__!(kind=$sepkind action=create value=$sep)
        )
    };
    (@splitf; [ input=$input:tt sel=(match $pattern:literal) ];) => {
        $crate::parser::Regex::new($pattern).unwrap().find_iter($input).map(|m| m.as_str())
    };
    (@splitf; [ input=$input:tt sel=(capture $pattern:literal) ];) => {
        $crate::parser::Regex::new($pattern).unwrap().captures_iter($input)
    };

    // Parse element into a grid.
    // [
    //   $name
    //   cells
    //   indexed?
    //   (
    //      as $type ||
    //      match { ... } ||
    //      with $transformer ||
    //      try? as $type match { ... };
    //      default $type char
    //   )
    // ]

    // cells
    (@parse; [ input=$input:tt tmp=$tmp:tt ]; [ $ident:ident cells $($rest:tt)* ]) => {
        $crate::parser::__parse__!(@cellsp; [ input=$input ident=$ident item=(char) ]; $($rest)*);
    };
    // indexed
    (@cellsp; [ input=$input:tt ident=$ident:tt item=$item:tt ]; indexed $($rest:tt)*) => {
        $crate::parser::__parse__!(@cellsp; [ input=$input ident=$ident item=$item indexing=explicit ]; $($rest)*)
    };
    (@cellsp; [ input=$input:tt ident=$ident:tt item=$item:tt ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@cellsp; [ input=$input ident=$ident item=$item indexing=none ]; $($rest)*)
    };
    // try as $type match
    (@cellsp; [ input=$input:tt ident=$ident:tt item=$item:tt indexing=$indexing:ident ]; try as $type:tt match $match:tt $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=cellsp input=$input ident=$ident item=$item indexing=$indexing trytype=$type ]; match $match $($rest)*)
    };
    (@cellsp; [ match=$match:tt indexes=$indexes:tt input=$input:tt ident=$ident:tt item=($($item:tt)+) indexing=$indexing:ident trytype=$type:tt matchflag=() ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@cellsp; [ input=$input ident=$ident item=(_) indexing=$indexing indexes=$indexes with1=(
            |item| $crate::parser::Tryable::to_option($crate::parser::__parse_type__!(item => $($item)+ => try $type)).ok_or(item)
        ) with2=(
            move |item| $crate::parser::__parse__!(@match_body; [ match=$match indexing=$indexing type=(($crate::point::Point2<usize>), (Result<$($item)+, $type>)) input=(item) matchflag=() ];)
        ) ]; $($rest)*)
    };
    // as $type
    (@cellsp; [ input=$input:tt ident=$ident:tt item=($($item:tt)+) indexing=$indexing:tt ]; as $type:tt $($rest:tt)*) => {
        $crate::parser::__parse__!(@cellsp; [ input=$input ident=$ident item=($type) indexing=$indexing with1=(
            |item| $crate::parser::__parse_type__!(item => $($item)+ => $type)
        ) ]; $($rest)*)
    };
    // with $transformer
    (@cellsp; [ input=$input:tt ident=$ident:tt item=$item:tt indexing=$indexing:tt ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@cellsp; [ input=$input ident=$ident item=$item indexing=$indexing with1=() ]; $($rest)*)
    };
    (@cellsp; [ input=$input:tt ident=$ident:tt item=$item:tt indexing=$indexing:tt with1=$with1:tt ]; with $transformer:expr) => {
        $crate::parser::__parse__!(@cellsp; [ input=$input ident=$ident item=$item indexing=$indexing indexes=() with1=$with1 with2=($transformer) ];)
    };
    (@cellsp; [ input=$input:tt ident=$ident:tt item=$item:tt indexing=$indexing:tt with1=$with1:tt ];) => {
        $crate::parser::__parse__!(@cellsp; [ input=$input ident=$ident item=$item indexing=$indexing indexes=() with1=$with1 with2=() ];)
    };
    // match
    (@cellsp; [ input=$input:tt ident=$ident:tt item=$item:tt indexing=$indexing:tt with1=$with1:tt ]; match $match:tt $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=cellsp input=$input ident=$ident item=$item indexing=$indexing with1=$with1 matchflag=() ]; match $match $($rest)*)
    };
    (@cellsp; [ match=$match:tt indexes=$indexes:tt input=$input:tt ident=$ident:tt item=($($item:tt)+) indexing=$indexing:ident with1=$with1:tt matchflag=() ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@cellsp; [ input=$input ident=$ident item=(_) indexing=$indexing indexes=$indexes with1=$with1 with2=(
            move |item| $crate::parser::__parse__!(@match_body; [ match=$match indexing=$indexing type=((usize), ($($item)+)) input=(item) matchflag=() ];)
        ) ]; $($rest)*)
    };

    // done. work backwards by repeatedly transforming some portion of the definition into a
    // chained method call
    (@cellsp; [ input=$input:tt ident=$ident:ident item=$item:tt indexing=$indexing:tt indexes=$indexes:tt $($rest:tt)* ];) => {
        $crate::parser::__parse__!(@match_define_vars; [ indexes=$indexes itype=($crate::point::Point2<usize>) ];);
        #[allow(unused_mut)]
        let mut $ident = $crate::parser::__parse__!(@cellsf; [ input=$input indexing=$indexing indexes=$indexes $($rest)* ];).collect::<$crate::grid::FullGrid<_>>();
        $crate::parser::__parse__!(@match_validate_vars; [ indexes=$indexes ];);
    };
    // indexing
    (@cellsf; [ input=$input:tt indexing=none indexes=$indexes:tt $($rest:tt)* ];) => {
        $input.split('\n').map(|row| {
            let iter = row.chars();
            $crate::parser::__parse__!(@cellsf; [ input=iter $($rest)* ];)
        })
    };
    (@cellsf; [ input=$input:tt indexing=$indexing:tt indexes=$indexes:tt $($rest:tt)* ];) => {
        $input.split('\n').enumerate().map(|(y, row)| {
            $crate::parser::__parse__!(@match_clone_vars; [ indexes=$indexes ];);
            let iter = row.char_indices().map(move |(x, cell)| ($crate::point::Point2::new(x, y), cell));
            $crate::parser::__parse__!(@cellsf; [ input=iter $($rest)* ];)
        })
    };
    // second with
    (@cellsf; [ input=$input:tt with1=$with1:tt with2=($transformer:expr) ];) => {
        $crate::parser::__parse__!(@cellsf; [ input=$input with1=$with1 ];).map($transformer)
    };
    (@cellsf; [ input=$input:tt with1=$with1:tt with2=() ];) => {
        $crate::parser::__parse__!(@cellsf; [ input=$input with1=$with1 ];)
    };
    // first with
    (@cellsf; [ input=$input:tt with1=($transformer:expr) ];) => {
        $crate::parser::__parse__!(@cellsf; [ input=$input ];).map($transformer)
    };
    (@cellsf; [ input=$input:tt with1=() ];) => {
        $crate::parser::__parse__!(@cellsf; [ input=$input ];)
    };
    // done
    (@cellsf; [ input=$input:tt ];) => ($input);

    // Match transformation. Note that not all options will be available in all contexts (e.g., the
    // indexes are only available on split/cells).
    // [
    //   match {
    //      (
    //         selector
    //         (=> (
    //             try? index into $name
    //             indexes into $name
    //         ))?
    //         (=>
    //            as $type ||
    //            with $transformer ||
    //            as $type with $transformer ||
    //            with [{ nested } => result] ||
    //            literal
    //         )?
    //      ),+
    //   }
    // ]

    // Process the macro body into an easier to work with form. This takes in the `match { ... }`
    // form and processes it, and then it calls the step specified with `$next` with the modified
    // list of arguments.
    //
    // The following items are prepended to the arguments:
    // - match=([ $pat ]::[ ($indexvar $indextype)? ]::[ $transform? ] ...)
    // - indexes=([ $indexvar $indextype] ...)
    //
    // In addition the indexed argument (if present) is updated if required.
    (@matchp; [ next=$next:tt $($passthrough:tt)* ]; match { $($args:tt)+ } $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=$next done=() todo=($($args)+) $($passthrough)* ];)
    };
    // read next match arm
    (@matchp; [ next=$next:tt done=$done:tt todo=($pat:pat $(if $cond:expr)? $(=> $($idents:ident)+)? $(, $($todo:tt)*)?) $($passthrough:tt)* ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=$next done=$done current=([$pat $(if $cond)?]::[$($($idents)+)?]::[]) todo=($($($todo)*)?) $($passthrough)* ]; $($rest)*)
    };
    (@matchp; [ next=$next:tt done=$done:tt todo=($pat:pat $(if $cond:expr)? $(=> $($idents:ident)+)? $(, $($todo:tt)*)?) $($passthrough:tt)* ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; ( next=$next done=$done current=([$pat $(if $cond)?]::[$($($idents)+)?]::[]) todo=($($($todo)*)?) $($passthrough)* ); $($rest)*)
    };
    (@matchp; [ next=$next:tt done=$done:tt todo=($pat:pat $(if $cond:expr)? => $($index:ident)+ => $($trans:ident)+ $(, $($todo:tt)*)?) $($passthrough:tt)* ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; ( next=$next done=$done current=([$pat $(if $cond)?]::[$($index)+]::[$($trans)+]) todo=($($($todo)*)?) $($passthrough)* ); $($rest)*)
    };
    (@matchp; [ next=$next:tt done=$done:tt todo=($pat:pat $(if $cond:expr)? => $($index:ident)+ => $trans:expr $(, $($todo:tt)*)?) $($passthrough:tt)* ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=$next done=$done current=([$pat $(if $cond)?]::[$($index)+]::[$trans]) todo=($($($todo)*)?) $($passthrough)* ]; $($rest)*)
    };
    (@matchp; [ next=$next:tt done=$done:tt todo=($pat:pat $(if $cond:expr)? => $trans:expr $(, $($todo:tt)*)?) $($passthrough:tt)* ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=$next done=$done current=([$pat $(if $cond)?]::[]::[$trans]) todo=($($($todo)*)?) $($passthrough)* ]; $($rest)*)
    };
    // $selector => index into $name (=> $transformation)?
    (@matchp; [ next=$next:tt done=($($done:tt)*) current=($pat:tt::[index into $pname:ident]::$trans:tt) todo=$todo:tt $($passthrough:tt)* ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=$next done=($($done)* $pat::[$pname index]::$trans) todo=$todo $($passthrough)* ]; $($rest)*)
    };
    // $selector => try index into $name (=> $transformation)?
    (@matchp; [ next=$next:tt done=($($done:tt)*) current=($pat:tt::[try index into $pname:ident]::$trans:tt) todo=$todo:tt $($passthrough:tt)* ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=$next done=($($done)* $pat::[$pname try index]::$trans) todo=$todo $($passthrough)* ]; $($rest)*)
    };
    // $selector => indexes into $name (=> $transformation)?
    (@matchp; [ next=$next:tt done=($($done:tt)*) current=($pat:tt::[indexes into $pname:ident]::$trans:tt) todo=$todo:tt $($passthrough:tt)* ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=$next done=($($done)* $pat::[$pname indexes]::$trans) todo=$todo $($passthrough)* ]; $($rest)*)
    };
    // $selector (=> $transformation)?
    (@matchp; [ next=$next:tt done=($($done:tt)*) current=($pat:tt::[]::$trans:tt) todo=$todo:tt $($passthrough:tt)* ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=$next done=($($done)* $pat::[]::$trans) todo=$todo $($passthrough)* ]; $($rest)*)
    };
    // $selector => idents is ambiguous as to whether these idents are about the index or about a
    // transformation. We tried the first option above, so given that failed we'll move it to the
    // transformation slot and consider this arm complete (the transformation slot isn't parsed
    // during the normalization step).
    (@matchp; [ next=$next:tt done=($($done:tt)*) current=($pat:tt::[$($args:ident)+]::[]) todo=$todo:tt $($passthrough:tt)* ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=$next done=($($done)* $pat::[]::[$($args)+]) todo=$todo $($passthrough)* ]; $($rest)*)
    };
    // parsing is done, add match and indexes flags
    (@matchp; [ next=$next:tt done=($($pat:tt::[$($($indexargs:ident)+)?]::$trans:tt)+) todo=() $($passthrough:tt)* ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=$next match=($($pat::[$($($indexargs)+)?]::$trans)+) indexes=($($([$($indexargs)+])?)+) ] [ $($passthrough)* ]; $($rest)*)
    };
    // update indexing flag (if present)
    (@matchp; [ next=$next:tt match=$match:tt indexes=() $($flags:tt)+ ] [ indexing=none $($tail:tt)* ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=$next match=$match indexes=() $($flags)+ indexing=none $($tail)+ ] []; $($rest)*)
    };
    (@matchp; [ next=$next:tt match=$match:tt indexes=$indexes:tt $($flags:tt)+ ] [ indexing=none $($tail:tt)* ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ next=$next match=$match indexes=$indexes $($flags)+ indexing=implicit $($tail)+ ] []; $($rest)*)
    };
    (@matchp; [ $($flags:tt)+ ] [ indexing=$indexing:ident $($tail:tt)* ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ $($flags)+ indexing=$indexing $($tail)+ ] []; $($rest)*)
    };
    (@matchp; [ $($flags:tt)+ ] [ $key:ident = $value:tt $($tail:tt)* ]; $($rest:tt)*) => {
        $crate::parser::__parse__!(@matchp; [ $($flags)+ $key=$value ] [ $($tail)* ]; $($rest)*)
    };
    (@matchp; [ next=$next:tt $($flags:tt)+ ] []; $($rest:tt)*) => {
        $crate::parser::__parse__!(@$next; [ $($flags)+ ]; $($rest)*)
    };

    // The following sections are to generate the actual code segments. There are a few rules to
    // this:
    // - All segments must be used, and they must be in the correct order (@match_define_vars,
    //   @match_body, @match_validate_vars)
    // - The type must always be in the form (index_type, value_type) regardless of the index flag.
    // - The input must be in the form expected for the index flag (i.e., of value_type if the
    //   flags is none and of (index_type, value_type) otherwise).

    // Generate definitions for the variables set in the match block.
    (@match_define_vars; [ indexes=($($index:tt)*) itype=$itype:tt ];) => {
        $(
            $crate::parser::__parse__!(@match_define_vars; [ index=$index itype=$itype ];);
        )*
    };
    (@match_define_vars; [ index=[$name:ident $(try)? index] itype=($($itype:tt)+) ];) => {
        let $name = ::std::rc::Rc::new(::std::cell::OnceCell::<$($itype)+>::new());
    };
    (@match_define_vars; [ index=[$name:ident indexes] itype=($($itype:tt)+) ];) => {
        let $name = ::std::rc::Rc::new(::std::cell::RefCell::new(Vec::<$($itype)+>::new()));
    };

    // Generate clones of the variables set in the match block.
    (@match_clone_vars; [ indexes=($($index:tt)*) ];) => {
        $(
            $crate::parser::__parse__!(@match_clone_vars; [ index=$index ];);
        )*
    };
    (@match_clone_vars; [ index=[$name:ident $($args:ident)+] ];) => {
        let $name = $name.clone();
    };

    // Generate match expression.
    (@match_body; [ match=$match:tt indexing=explicit type=($itype:tt, $vtype:tt) input=($input:expr) matchflag=$matchflag:tt ];) => {
        $crate::parser::__parse__!(@match_body; [ match=$match vtype=(($itype, $vtype)) index=($input.0) value=($input) matchflag=$matchflag ];)
    };
    (@match_body; [ match=$match:tt indexing=implicit type=($itype:tt, $vtype:tt) input=($input:expr) matchflag=$matchflag:tt ];) => {
        $crate::parser::__parse__!(@match_body; [ match=$match vtype=$vtype index=($input.0) value=($input.1) matchflag=$matchflag ];)
    };
    (@match_body; [ match=$match:tt indexing=none type=($itype:tt, $vtype:tt) input=($input:expr) matchflag=$matchflag:tt ];) => {
        $crate::parser::__parse__!(@match_body; [ match=$match vtype=$vtype index=(None) value=($input) matchflag=$matchflag ];)
    };
    (@match_body; [ match=($([ $pat:pat $(if $cond:expr)? ]::$indexargs:tt::$trans:tt)+) vtype=$vtype:tt index=$index:tt value=$value:tt matchflag=$matchflag:tt ];) => {
        match $value {
            $(
                $pat $(if $cond)? => {
                    $crate::parser::__parse__!(@match_body_index; [ index=$index args=$indexargs ];);
                    $crate::parser::__parse__!(@match_body_transform; [ vtype=$vtype value=$value matchflag=$matchflag trans=$trans ];)
                },
            )+
            #[allow(unreachable_patterns)]
            _ => $crate::parser::__parse__!(@match_body_fallback; [ index=$index value=$value ];),
        }
    };
    (@match_body_fallback; [ index=(None) value=$value:tt ];) => {
        panic!("value {:?} doesn't match any of the match arms.", $value)
    };
    (@match_body_fallback; [ index=$index:tt value=($value:expr) ];) => {
        panic!("value {:?} at index {:?} doesn't match any of the match arms.", $value, $index)
    };

    // Generate index assignment.
    (@match_body_index; [ index=($index:expr) args=[$name:ident $(try)? index] ];) => {
        if $name.set($index).is_err() {
            panic!("index {} was set multiple times (at {:?} and {:?}).", stringify!($name), $name.get().unwrap(), $index);
        }
    };
    (@match_body_index; [index=($index:expr) args=[$name:ident indexes] ];) => {
        $name.borrow_mut().push($index);
    };
    (@match_body_index; [ index=($index:expr) args=[] ];) => {};

    // Generate transformation.
    (@match_body_transform; [ vtype=$vtype:tt value=($value:expr) matchflag=(try) trans=[] ];) => (Some($value));
    (@match_body_transform; [ vtype=$vtype:tt value=($value:expr) matchflag=() trans=[] ];) => ($value);
    (@match_body_transform; [ vtype=($($vtype:tt)+) value=($value:expr) matchflag=$matchflag:tt trans=[as $target:tt] ];) => {
        $crate::parser::__parse_type__!($value => $($vtype)+ => $target)
    };
    (@match_body_transform; [ vtype=$vtype:tt value=($value:expr) matchflag=$matchflag:tt trans=[with { $($nested:tt)+ } => $result:expr] ];) => {
        $crate::parser::parse!($input => { $($nested)+ } => $result)
    };
    (@match_body_transform; [ vtype=$vtype:tt value=($value:expr) matchflag=$matchflag:tt trans=[$expr:expr] ];) => {
        #[allow(unused_braces)]
        $expr
    };

    // Generate definitions for the variables set in the match block.
    (@match_validate_vars; [ indexes=($($index:tt)*) ];) => {
        $(
            $crate::parser::__parse__!(@match_validate_vars; [ index=$index ];);
        )*
    };
    (@match_validate_vars; [ index=[$name:ident index] ];) => {
        let $name = ::std::rc::Rc::into_inner($name).unwrap().into_inner().expect(&format!("index {} was never set.", stringify!($name)));
    };
    (@match_validate_vars; [ index=[$name:ident $($args:ident)+] ];) => {
        let $name = ::std::rc::Rc::into_inner($name).unwrap().into_inner();
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

    ($var:expr => $from:tt => $type:tt) => {
        $crate::parser::__parse_type__!($var => $from => try $type).unwrap()
    };
    ($var:expr => $from:tt => try $type:tt) => (<$type>::try_from($var));
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

    (kind=regex action=create value=$value:literal) => ($crate::parser::Regex::new($value).unwrap());
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
    use crate::point::Point2;

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
    fn parse_matching() {
        parse!("example8" => [prefix matching /r"\D*"/] [num as u8]);
        assert_eq!(prefix, "example");
        assert_eq!(num, 8);
    }

    // This error message is mangled by being escaped an additional time by Result::unwrap.
    #[test]
    #[should_panic] // = r#"couldn't match r"\d+" at start of "foo""#]
    fn parse_matching_mismatch() {
        parse!("foo" => [_num matching /r"\d+"/] _rest);
    }

    #[test]
    #[should_panic = r#"couldn't match r"\d+" at start of "foo8""#]
    fn parse_matching_non_start() {
        parse!("foo8" => [_num matching /r"\d+"/] _rest);
    }

    #[test]
    fn parse_capturing() {
        parse!("foo-8" => [prefix capturing /r"(\w+)\D*"/] [num as u8]);
        assert_eq!(prefix.get(1).unwrap().as_str(), "foo");
        assert_eq!(num, 8);
    }

    // This error message is mangled by being escaped an additional time by Result::unwrap.
    #[test]
    #[should_panic] // = r#"couldn't capture r"\d+" at start of "foo""#]
    fn parse_capturing_mismatch() {
        parse!("foo" => [_num capturing /r"\d+"/] _rest);
    }

    #[test]
    #[should_panic = r#"couldn't capture r"\d+" at start of "foo8""#]
    fn parse_capturing_non_start() {
        parse!("foo8" => [_num capturing /r"\d+"/] _rest);
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
        parse!("1-2,3" => [items find matches /r"\d"/]);
        assert_eq!(items, vec!["1", "2", "3"]);
    }

    #[test]
    fn parse_list_capture() {
        parse!("a1,b2!!!c3" => [items find captures /r"\w(\d)"/]);
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
    fn parse_list_match() {
        parse!("1 2 3" => [items split match { "1" => "one", "2" => "two", _ => "too many" }]);
        assert_eq!(items, vec!["one", "two", "too many"]);
    }

    #[test]
    fn parse_list_match_as() {
        parse!("12 4 6" => [items split as u8 match { (..=4) => "good", 5..=8 => "great", 9.. => "fantastic" }]);
        assert_eq!(items, vec!["fantastic", "good", "great"]);
    }

    #[test]
    fn parse_list_match_try_as() {
        parse!("one 12 two 4 6 0" => [items split try as u8 match { Ok(1..=4) => "good", Ok(5..=8) => "great", Ok(9..) => "fantastic", _ => "terrible" }]);
        assert_eq!(
            items,
            vec![
                "terrible",
                "fantastic",
                "terrible",
                "good",
                "great",
                "terrible"
            ]
        );
    }

    #[test]
    #[should_panic = "doesn't match any of the match arms"]
    fn parse_list_match_unmatched() {
        parse!("foo bar baz" => [_items split match { "foo" | "bar" => 42 }]);
    }

    #[test]
    fn parse_list_match_store_index() {
        parse!("foo bar baz" => [items split match { "bar" => index into bar_idx, _ }]);
        assert_eq!(items, vec!["foo", "bar", "baz"]);
        assert_eq!(bar_idx, 1);
    }

    #[test]
    #[should_panic = "index _idx was never set"]
    fn parse_list_match_store_index_not_found() {
        parse!("1 2 3" => [_items split match { "hello" => index into _idx, _ }]);
    }

    #[test]
    #[should_panic = "index _large_idx was set multiple times"]
    fn parse_list_match_store_index_multiple() {
        parse!("1 2 3 80 4 90" => [_items split as u8 match { 50.. => index into _large_idx, _ }]);
    }

    #[test]
    fn parse_list_match_try_store_index() {
        parse!("foo bar baz" => [items split match { "bar" => try index into bar_idx, _ }]);
        assert_eq!(items, vec!["foo", "bar", "baz"]);
        assert_eq!(bar_idx, Some(1));
    }

    #[test]
    fn parse_list_match_try_store_index_not_found() {
        parse!("1 2 3" => [items split match { "hello" => try index into idx, _ }]);
        assert_eq!(items, vec!["1", "2", "3"]);
        assert_eq!(idx, None);
    }

    #[test]
    #[should_panic = "index _large_idx was set multiple times"]
    fn parse_list_match_try_store_index_multiple() {
        parse!("1 2 3 80 4 90" => [_items split as u8 match { 50.. => try index into _large_idx, _ }]);
    }

    #[test]
    fn parse_list_try_as_match() {
        parse!("1 2 foo 3 4 foobar 5 6" => [items split try as u8 match { Ok(v) => v, Err(v) => (v.len() as u8 * 10) }]);
        assert_eq!(items, vec![1, 2, 30, 3, 4, 60, 5, 6]);
    }

    #[test]
    fn parse_list_try_match() {
        parse!("hay hay hay needle hay shoe hay hay" => [items split try match { "hay" => None, v => Some(v) }]);
        assert_eq!(items, vec!["needle", "shoe"]);
    }

    #[test]
    fn parse_list_match_guard() {
        parse!("foo bar baz" => [items split match { v if v.starts_with('b'), _ => "boo" }]);
        assert_eq!(items, vec!["boo", "bar", "baz"]);
    }

    #[test]
    fn parse_list_indexed_match() {
        parse!("fee-fi-fo-fum" => [items split on '-' indexed match { (i, v) if v.len() > i => 10, (i, v) => (i - v.len()) }]);
        assert_eq!(items, vec![10, 10, 0, 0]);
    }

    #[test]
    fn parse_list_indexed_match_store_indexes() {
        parse!("1 2 22 3 4 53 5 6" => [items split as u8 match { v if v > 10 => indexes into indexes => 0, _ }]);
        assert_eq!(items, vec![1, 2, 0, 3, 4, 0, 5, 6]);
        assert_eq!(indexes, vec![2, 5]);
    }

    #[test]
    fn parse_list_nested_match() {
        #[derive(Debug, PartialEq, Eq)]
        enum Sign {
            Plus,
            Minus,
        }
        parse!("+1 +2 -3" => [items split with
            {
                [sign take 1 match { "+" => Sign::Plus, "-" => Sign::Minus }]
                [num as u8]
            }
            => (sign, num)
        ]);
        assert_eq!(
            items,
            vec![(Sign::Plus, 1), (Sign::Plus, 2), (Sign::Minus, 3)]
        );
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
    fn parse_cells() {
        parse!("012\n345" => [grid cells]);
        assert_eq!(grid, [['0', '1', '2'], ['3', '4', '5']].into());
    }

    #[test]
    fn parse_cells_as() {
        parse!("012\n345" => [grid cells as u8]);
        assert_eq!(grid, [[0, 1, 2], [3, 4, 5]].into());
    }

    #[test]
    fn parse_cells_match() {
        parse!("012\n345" => [grid cells match { '3' => 100, _ => as u8 }]);
        assert_eq!(grid, [[0, 1, 2], [100, 4, 5]].into());
    }

    #[test]
    fn parse_cells_match_index() {
        parse!("012\n345" => [grid cells match { '3' => index into start, _ }]);
        assert_eq!(grid, [['0', '1', '2'], ['3', '4', '5']].into());
        assert_eq!(start, Point2::new(0, 1));
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
