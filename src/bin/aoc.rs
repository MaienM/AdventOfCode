fn main() {
    aoc_runner::multi::BINS.get_or_init(|| Box::new(aoc::bins::BINS.clone()));
    aoc_runner::multi::main();
}
