mod factorize;
mod grid;

use criterion::criterion_main;

criterion_main! {
    factorize::benches,
    grid::benches,
}
