use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

fn lang_int_swap_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("lang_int_swap");
    group.bench_function("by_copy", |b| {
        b.iter(|| {
            let mut arr = [1u32, 2u32];
            let temp = arr[0];
            black_box(arr[0] = arr[1]);
            arr[1] = temp;
            black_box(arr)
        })
    });

    group.bench_function("inplace", |b| {
        b.iter(|| {
            let mut arr = [1u32, 2u32];
            black_box(arr.swap(0, 1))
        })
    });

    group.finish();
}

const S1: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuwwxz1234567890";
const S2: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuwwxz0987654321";
// swap by temp is clone, which requires memory allocation, the cost can't be ignored
// the good thing about Rust is it makes the clone/copy explicit
fn lang_string_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("lang_string");
    let data = [String::from(S1), String::from(S2)];
    group.bench_function("swap_by_copy", |b| {
        b.iter_batched(
            || data.clone(),
            |mut arr| {
                let temp = arr[0].clone();
                black_box(arr[0] = arr[1].clone());
                arr[1] = temp
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("swap_by_empty_temp", |b| {
        b.iter_batched(
            || data.clone(),
            |mut arr| {
                // let temp = arr[0]; // cannot move here!
                let mut temp = String::new();
                std::mem::swap(&mut temp, &mut arr[0]);
                black_box(arr.swap(0, 1));
                std::mem::swap(&mut arr[1], &mut temp)
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("swap_inplace", |b| {
        b.iter_batched(
            || data.clone(),
            |mut arr| black_box(arr.swap(0, 1)),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("comparision", |b| {
        b.iter_batched(
            || (String::from(S1), String::from(S2)),
            |(s1, s2)| {
                black_box(s1 == s2);
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = lang_int_swap_benches, lang_string_benches
}
criterion_main!(benches);
