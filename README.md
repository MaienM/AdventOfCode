# Puzzles

## Webassembly

You can run the precompiled webassembly version in the browser [here](https://maienm.github.io/Puzzles). The performance of this is (obviously) worse than when running it natively, but (depending on the solution and browser used) this might only be by as little as 20%.

Chromium based browsers will yield better runtimes and more accurate timings than Firefox.

## Development

Using Nix you can get an environment with all required dependencies by running:

``` shell
nix develop
```

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

## Timing

The timings are _only_ for the actual solution, starting right before the main function is called (with the input passed in as a string), and it ends as soon as this function returns. This means the following are all excluded from the displayed runtimes:

- Parsing arguments.
- Reading the input/expected answer files from disk.
- Initializing the threadpool.
- Validating & displaying the results
