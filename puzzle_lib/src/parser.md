Macro to quickly perform common parsing operations on a string.

This macro will take the given input, attempt to parse all the provided sections from it, (defining the variables and
performing the transformations defined by these sections), and then ensure that the input is fully consumed by this. Any
failures to do so will cause it to panic with a (hopefully) informative message.

The `parse!(input => { sections } => result)` form is a shorthand for `{ parse!(input => sections); result }`, turning
the result of the macro into a single expression rather than a list of assignments.

# Sections

The following sections are supported:

- [A literal value](#literals). When used at the start this will strip a prefix, and when used after any assignment it
  will set the boundary for the text used for that assignment; that is, the first matching instance of this literal will
  be found and the text between that and the previous literal/start of input will be used for the assignment. The
  matched literal itself will be discarded (i.e., not assigned to any variables).
- An identifier (`name`). This is a shorthand for `[name]`.
- An assignment (`[name]`). This will take the value that is matched at the given location and store it in a variable
  with that name. This supports [transformations](#transformations) (e.g., `[name as type]`).
- An assignment of a fixed size (`[name take num]`). Like `[name]`, but instead of taking all the text matched at the
  given location it only takes the specified number of bytes and leaves the rest for the next section.
- An assignment of a section matching a regular expression (`[name matching /"pattern"/]`). Like `[name]`, but instead
  of taking all the text matched at the given location it only takes the portion matched by the provided regular
  expression and leaves the rest for the next section. See [`regex#syntax`] for the supported syntax.
- An assignment of a section matching a regular expression with captures (`[name capturing /"pattern"/]`). Like `[name
  matching /"pattern"/]`, but instead of the matched string the result will be the [`Captures`](Captures).

# Literals

The following types of literals are supported:

- A literal string (`"foo"`).
- A literal character (`'a'`).
- A regular expression (`/"pattern/"`). See [`regex#syntax`] for the supported syntax. You can also use a [raw string
  literal](https://doc.rust-lang.org/reference/tokens.html#raw-string-literals) between the slashes if convenient
  (`/r#"\w+"#/`).

# Transformations

The following options can be used to transform a matched value:

- A typecast (`as type`). This will convert the matched string into the given type, either using one of the built-in
  methods for some primitives, or using `type::from` as a fallback for any other types.
- A call (`with func`). This will call the provided function with a single argument (the matched string), and it will
  then store the result of that call. When used on an indexed iterator the single argument will instead be a tuple
  (`(index: usize, matched: &str)`).
- A combination of previous two (`as type with func`). This will convert to the given type (as per above) and then call
  the provided function (as per above) with the result of this conversion.
- A nested parse (`with { segments } => result`). This takes the matched string and feeds it into a nested call of this
  macro with the given segments & result mapping.
- [A split operation](#split).
- [A cells operation](#cells).
- [A match operation](#match).

# Split

The `split` keyword will take the matched string and split it into an iterator or collection. This entire operation is
described by a series of options (most optional) which describe how to split the string and how to process the resulting
iterator.

Here are all the options in the order they must appear in:

- The thing to split on (required). Options:
  - `split`. This is a shorthand for `split on ' '`.
  - `split on literal`. This will split the string on the given [literal](#literal), discarding the separators and
    keeping the elements between the separators.
  - `chars`. This will iterate over the individual [`char`]acters of the string.
  - `find matches /"pattern"/`. This will look for all matches of the given regular expression and keep the entire
    matched string, ignoring any capture groups.
  - `find captures /"pattern"/`. This will look for all matches of the given regular expression and keep the
    [`Captures`](Captures).
- `indexed` (optional). This will chain [`enumerate`](Iterator::enumerate) on the iterator.
- The collection to transform the iterator into (optional, default `into (Vec<_>)`):
  - `into iterator`. This will keep the result as a [`Iterator`] (the exact type of which depends on the other options).
  - `into (type)`. This will call [`collect`](Iterator::collect) on the iterator to transform it into the given type.
  - `try into (type)`. This will first call [`collect`](Iterator::collect) on the iterator to transform it into a
    [`Vec`], and then it will call [`TryInto::try_into`] to attempt to convert it into the given type.
- A transformation (optional):
  - Any of the [base transformations](#transformations).
  - `try transformation`. A [base transformation](#transformations) that's allowed to fail, with failing elements
    dropped from the resulting iterator. Only supported for typecasts and calls.
  - `match`. A [match transformation](#match).
  - `try match`. A [match transformation](#match) which returns `Option`s, with `None` results dropped from the
    resulting iterator.
  - `try transformation match`. This combines a [base transformation](#transformations) that's allowed to fail with a
    [match transformation](#match). The input type for this match will be `Result`, with items that are successfully
    transformed by the base transformation being wrapped in`Ok` & items that failed to be transformed being wrapped in
    `Err`.
  - `try transformation try match`. A combination of the `try transformations match` form with the `try match` form,
    with the match input following the former and the match behavior following the latter.

# Cells

The `cells` keyword will take the matched strings and parse it into a [`crate::grid::FullGrid`], with each line being a
row and each character on that line being a cell within that row.

This accepts additional options to transform the `char` values for each cell. Here are all the options, in the order
they must appear in:

- `indexed` (optional). This will transform the cell value into `(Point2<usize>, char)` and then perform the rest of the
  transformation using that value. This doesn't affect the output type, just the type used for the transformation. Using
  this makes the next argument mandatory.
- A transformation (optional). This can be any of the transformations supported by (`split`)[#split) that doesn't filter
  out values (e.g., it can be `as $type` but not `try as $type`). The index type captured by match operations will be
  `Poin2<usize>` instead of `usize`.

# Match

The `match` transformation processes the value by running it through a match statement. Just like a match statement the
body is made up of match rules separated by commas. The format of these match rules is inspired by the default match
statement, but doesn't quite follow it. Each match rule contains the following segments, separated by `=>`:

- The pattern, which functions just like in regular match arm, including binding variables with `@` and using match
  guards.
- Instructions to save the index at which the match occurred (optional). Only available when used as an option
  on a `split` or `cells` operation. This creates an additional variable at the same level that the variable in which
  the collection is stored is located. This has one of the following forms:
  - `index into name`. This simply stores the index in the variable with the given name. It must match exactly once
    per collection (i.e., this variable cannot be undefined or defined multiple times).
  - `try index into name`. This is like the `index into name` form, but the variable becomes a [`Option`], and it is
    allowed to match 0 times (but still not multiple times).
  - `indexes into name`. This is like the `index into name` form, but the variable becomes a [`Vec`], and it is allowed
    to match any number of times.
- The expression to evaluate for matching values (optional). If not provided the input value will be passed through
  (i.e., the rule `1..5` is equivalent to `value @ 1..5 => value`). In addition to an expression/regular match body the
  following [base transformations](#transformations) are allowed:
  - `as type`
  - `with { segments } => result`

A fallback arm will be added at the end which panics with the unmatched value.

# Examples

Literals in various positions:

```rust
# use puzzle_lib::parser::parse;
parse!("hello world" => "hello " subject);
assert_eq!(subject, "world");
```

```rust
# use puzzle_lib::parser::parse;
parse!("howdy pardner" => greeting /r#" \w+"#/);
assert_eq!(greeting, "howdy");
```

```rust
# use puzzle_lib::parser::parse;
parse!("g'day mate" => greeting ' ' subject);
assert_eq!(greeting, "g'day");
assert_eq!(subject, "mate");
```

Transformations:

```rust
# use puzzle_lib::parser::parse;
parse!("detonating in 4..." => "detonating in " [seconds as u8] "...");
assert_eq!(seconds, 4u8);
```

```rust
# use puzzle_lib::parser::parse;
parse!("+12" => [prefix take 1 as char] [num as u8]);
assert_eq!(prefix, '+');
assert_eq!(num, 12);
```

```rust
# use puzzle_lib::parser::parse;
parse!("example8" => [prefix matching /r"\D*"/] [num as u8]);
assert_eq!(prefix, "example");
assert_eq!(num, 8);
```

```rust
# use puzzle_lib::parser::parse;
parse!("boo" => [word with str::to_uppercase]);
assert_eq!(word, "BOO");
```

```rust
# use puzzle_lib::parser::parse;
parse!("sqrt(25)" => "sqrt(" [root as u8 with u8::isqrt] ')');
assert_eq!(root, 5);
```

```rust
# use puzzle_lib::parser::parse;
//parse!("|1 -1|" => '|' [pair with { [l as i8] ' ' [r as i8] } => (l, r)] '|');
//assert_eq!(pair, (1i8, -1i8));
```

Split:

```rust
# use puzzle_lib::parser::parse;
parse!("fee-fi-fo-fum" => [words split on '-']);
assert_eq!(words, vec!["fee", "fi", "fo", "fum"]);
```

```rust
# use puzzle_lib::parser::parse;
parse!("10 -10" => [nums split into iterator as i8]);
assert_eq!(nums.next(), Some(10));
assert_eq!(nums.next(), Some(-10));
assert_eq!(nums.next(), None);
```

```rust
# use std::collections::HashSet;
# use puzzle_lib::parser::parse;
parse!("n33dl3 in 4 h4yst4ck" => [nums chars into (HashSet<_>) try as u8]);
assert_eq!(nums, HashSet::from([3, 4]));
```

```rust
# use puzzle_lib::parser::parse;
parse!("1 2 4 8" => [nums split try into ([u8; 4]) as u8 with u8::reverse_bits]);
assert_eq!(nums, [128, 64, 32, 16]);
```

```rust
# use puzzle_lib::parser::parse;
parse!("2 fast 2 furious" => [words find matches /"[a-z]+"/]);
assert_eq!(words, vec!["fast", "furious"]);
```

```rust
# use puzzle_lib::parser::parse;
parse!("hello world" => [words split match { "hello" | "goodbye" => "greeting", _ => "subject" } ]);
assert_eq!(words, vec!["greeting", "subject"]);
```

```rust
# use puzzle_lib::parser::parse;
parse!("1 2 30 4" => [nums split as u8 try match { 1..5, _ => None } ]);
assert_eq!(nums, vec![1, 2, 4]);
```

```rust
# use puzzle_lib::parser::parse;
parse!("1 2 | 30 4" => [nums split match { "|" => index into split_idx => 0, _ => as u8 } ]);
assert_eq!(nums, vec![1, 2, 0, 30, 4]);
assert_eq!(split_idx, 2);
```

```rust
# use puzzle_lib::parser::parse;
parse!("one 12 two 4 6 0" => [items split try as u8 match { Ok(1..=4) => "good", Ok(5..=8) => "great", Ok(9..) => "fantastic", _ => "terrible" }]);
assert_eq!(
    items,
    vec!["terrible", "fantastic", "terrible", "good", "great", "terrible"]
);
```

```rust,compile_fail
# use puzzle_lib::parser::parse;
// Combining `into iterator` with a match that captures indexes doesn't work as expected (since
// these captures would only happen when consuming the iterator) so this is disallowed.
parse!("1 2" => [items split into iterator match { "1" => index into idx, _ } ]);
```

```rust
# use std::collections::HashMap;
# use puzzle_lib::parser::{parse, Captures};
fn to_pair(capture: Captures) -> (&str, u8) {
    (
        capture.get(1).unwrap().as_str(),
        capture.get(2).unwrap().as_str().parse().unwrap(),
    )
}
parse!("foo=1 bar baz=2" => [map find captures /r#"(\w+)=(\d+)"#/ into (HashMap<_, _>) with to_pair]);
assert_eq!(map.len(), 2);
assert_eq!(map.get(&"foo"), Some(&1));
assert_eq!(map.get(&"baz"), Some(&2));
```

Grid:

```rust
# use puzzle_lib::parser::parse;
parse!("012\n345" => [grid cells as u8]);
assert_eq!(grid, [[0, 1, 2], [3, 4, 5]].into());
```

```rust
# use puzzle_lib::parser::parse;
parse!("012\n345" => [grid cells match { '0'..='4' => as u8, _ => 100 }]);
assert_eq!(grid, [[0, 1, 2], [3, 4, 100]].into());
```

```rust
# use puzzle_lib::parser::parse;
# use puzzle_lib::point::Point2;
parse!("012\n345" => [grid cells match { '3' => index into start, _ }]);
assert_eq!(grid, [['0', '1', '2'], ['3', '4', '5']].into());
assert_eq!(start, Point2::new(0, 1));
```
