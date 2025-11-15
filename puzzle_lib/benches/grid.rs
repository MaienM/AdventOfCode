use criterion::{BatchSize, Criterion, criterion_group};
use puzzle_lib::grid::FullGrid;

pub fn create_grid(c: &mut Criterion) {
    c.bench_function("create_grid_from_iterator", |b| {
        b.iter(|| {
            (0..100)
                .map(|y| (0..100).map(move |x| x + y * 1000))
                .collect::<FullGrid<_>>()
        });
    });

    c.bench_function("create_grid_from_vec", |b| {
        let vec = (0..100)
            .map(|y| (0..100).map(move |x| x + y * 1000).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        b.iter_batched(|| vec.clone(), FullGrid::from, BatchSize::SmallInput);
    });

    c.bench_function("create_grid_from_slice", |b| {
        #[allow(clippy::large_stack_arrays)]
        let slice = [[0; 100]; 100];
        b.iter(|| FullGrid::from(slice));
    });
}

criterion_group!(benches, create_grid);
