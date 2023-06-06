use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use leetrust::zigzag_conversion::Solution;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::iter::repeat_with;

fn benchmark_fun(c: &mut Criterion) {
    let mut rng = thread_rng();
    let input: String = repeat_with(|| rng.sample(Alphanumeric) as char)
        .take(1000)
        .collect();

    let mut group = c.benchmark_group("zigzag_conversion");
    for k in [1, 5, 10, 20, 40, 80, 160, 320, 640, 1000].iter() {
        group.throughput(Throughput::Bytes(*k as u64));
        group.bench_with_input(BenchmarkId::from_parameter(k), k, |b, &size| {
            b.iter(|| Solution::convert(black_box(input.clone()), size));
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark_fun);
criterion_main!(benches);
