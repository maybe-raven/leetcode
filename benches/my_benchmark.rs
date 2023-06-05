use std::iter::repeat_with;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use leetrust::frequency_sort::Solution;

fn benchmark_fun(c: &mut Criterion) {
    let mut rng = thread_rng();
    let input: String = repeat_with(|| rng.sample(Alphanumeric) as char)
        .take(500000)
        .collect();

    c.bench_function("frequency_sort", |b| {
        b.iter(|| Solution::frequency_sort(black_box(input.clone())))
    });
}

criterion_group!(benches, benchmark_fun);
criterion_main!(benches);
