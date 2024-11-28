# Advent of Code

To run a given day use either:

``` shell
make test-and-run 21-01
cargo run --release --bin 21-01 [input.txt]
```

The first form will run the test and then process the input from the corresponding file in `inputs` (e.g. `inputs/21-01.txt`). If this file doesn't exist it will try to download it from the AoC website using the session cookie stored in `.session`.

The second version will process the provided path.

To run all days:

```
make run-all
```

This will not attempt to download any new inputs.
