use std::iter::repeat;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::{thread_rng, Rng};

use algs4_rs::strings::lzw::*;

fn lzw_compress_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("lzw_compress");
    group.bench_function("100", |b| {
        b.iter_batched(
            || thread_rng().gen_iter::<u8>().take(100).collect::<Vec<_>>(),
            |arr| black_box(compress(&arr)),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("100k", |b| {
        b.iter_batched(
            || {
                thread_rng()
                    .gen_iter::<u8>()
                    .take(100 * 1024)
                    .collect::<Vec<_>>()
            },
            |arr| black_box(compress(&arr)),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("100k_zero", |b| {
        b.iter_batched(
            || repeat(0).take(100 * 1024).collect::<Vec<_>>(),
            |arr| black_box(compress(&arr)),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = lzw_compress_benches
}
criterion_main!(benches);
