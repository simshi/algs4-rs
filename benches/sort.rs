use std::iter;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{thread_rng, Rng};

use algs4_rs::sorting::*;

macro_rules! make_bench {
    ($group:ident, $input:ident, $name:literal, $sorter:ident, $size:expr) => {
        $group.bench_with_input(
            BenchmarkId::new(format!("{}", $size), $name),
            &$input,
            |b, i| {
                b.iter(|| {
                    let mut arr = i.clone();
                    black_box($sorter(&mut arr))
                });
            },
        )
    };
}

fn sort_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort");

    for size in [7, 15, 100, 1000, 10000].iter() {
        let input = thread_rng()
            .gen_iter::<u32>()
            .take(*size)
            .collect::<Vec<_>>();

        make_bench!(group, input, "insertion", insertion_sort, size);
        make_bench!(group, input, "merge", merge_sort, size);
        make_bench!(group, input, "merge_opt", merge_sort_opt, size);
        make_bench!(group, input, "heap", heap_sort, size);
        make_bench!(group, input, "heap_opt", heap_sort_opt, size);
        make_bench!(group, input, "shell", shell_sort, size);
        make_bench!(group, input, "quick", quick_sort, size);
        make_bench!(group, input, "quick_3way", quick3way_sort, size);
        make_bench!(group, input, "quick_opt", quick_sort_opt, size);
        // make_bench!(group, input, "builtin", sort_unstable, size);
        group.bench_with_input(
            BenchmarkId::new(format!("{}", size), "builtin"),
            &input,
            |b, i| {
                b.iter(|| {
                    let mut arr = i.clone();
                    black_box(arr.sort_unstable());
                });
            },
        );
    }

    group.finish();
}

fn sort_same_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort_same");
    let input = iter::repeat(23u32).take(100).collect::<Vec<_>>();

    make_bench!(group, input, "insertion", insertion_sort, 100);
    make_bench!(group, input, "merge", merge_sort, 100);
    make_bench!(group, input, "merge_opt", merge_sort_opt, 100);
    make_bench!(group, input, "heap", heap_sort, 100);
    make_bench!(group, input, "heap_opt", heap_sort_opt, 100);
    make_bench!(group, input, "shell", shell_sort, 100);
    make_bench!(group, input, "quick", quick_sort, 100);
    make_bench!(group, input, "quick_3way", quick3way_sort, 100);
    make_bench!(group, input, "quick_opt", quick_sort_opt, 100);
    group.bench_with_input(BenchmarkId::new("100", "builtin"), &input, |b, i| {
        b.iter(|| {
            let mut arr = i.clone();
            black_box(arr.sort_unstable());
        });
    });

    group.finish();
}

fn sort_sorted_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort_sorted");
    let input = (0..100).take(100).collect::<Vec<_>>();

    make_bench!(group, input, "insertion", insertion_sort, 100);
    make_bench!(group, input, "merge", merge_sort, 100);
    make_bench!(group, input, "merge_opt", merge_sort_opt, 100);
    make_bench!(group, input, "heap", heap_sort, 100);
    make_bench!(group, input, "heap_opt", heap_sort_opt, 100);
    make_bench!(group, input, "shell", shell_sort, 100);
    make_bench!(group, input, "quick", quick_sort, 100);
    make_bench!(group, input, "quick_3way", quick3way_sort, 100);
    make_bench!(group, input, "quick_opt", quick_sort_opt, 100);
    group.bench_with_input(BenchmarkId::new("100", "builtin"), &input, |b, i| {
        b.iter(|| {
            let mut arr = i.clone();
            black_box(arr.sort_unstable());
        });
    });

    group.finish();
}

fn sort_reversed_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort_reversed");
    let input = (0..1000).rev().collect::<Vec<_>>();

    make_bench!(group, input, "insertion", insertion_sort, 1000);
    make_bench!(group, input, "merge", merge_sort, 1000);
    make_bench!(group, input, "merge_opt", merge_sort_opt, 1000);
    make_bench!(group, input, "heap", heap_sort, 1000);
    make_bench!(group, input, "heap_opt", heap_sort_opt, 1000);
    make_bench!(group, input, "shell", shell_sort, 1000);
    make_bench!(group, input, "quick", quick_sort, 1000);
    make_bench!(group, input, "quick_3way", quick3way_sort, 1000);
    make_bench!(group, input, "quick_opt", quick_sort_opt, 1000);
    group.bench_with_input(BenchmarkId::new("1000", "builtin"), &input, |b, i| {
        b.iter(|| {
            let mut arr = i.clone();
            black_box(arr.sort_unstable());
        });
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = sort_benches, sort_same_benches, sort_sorted_benches, sort_reversed_benches
}
criterion_main!(benches);
