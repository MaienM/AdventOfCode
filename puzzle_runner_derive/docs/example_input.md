Mark an attribute as an example input.

The leading and trailing newline + a static amount of indentation for each line will be stripped to make the input match
the original. The result will be stored in an [`Example`](puzzle_runner::derived::Example) along with the expected outputs
(if provided).

A test will be generated for each part that has an expected output defined.
