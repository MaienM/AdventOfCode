#![doc(hidden)]

// Unfortunately generating this in `aoc_runner` doesn't work as this macro expands into a mod
// containing the source of all binaries, which when done in `aoc_runner` will not have access to
// `aoc` or any of its dependencies.
#[aoc_runner::inject_binaries(path = "bin")]
pub static BINS: Vec<Bin>;
