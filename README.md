# Advent of Code

## Webassembly

You can run the precompiled webassembly version in the browser [here](https://maienm.github.io/AdventOfCode). The performance of this is (obviously) worse than when running it natively, but (depending on the solution and browser used) this might only be by as little as 20%.

Chromium based browsers will yield better runtimes and more accurate timings than Firefox.

## Nix

If you have Nix installed you can run this project without even downloading it!

``` shell
nix run --extra-experimental-features 'nix-command flakes' 'github:MaienM/AdventOfCode#aoc'
nix run --extra-experimental-features 'nix-command flakes' 'github:MaienM/AdventOfCode#21-01'
```

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
