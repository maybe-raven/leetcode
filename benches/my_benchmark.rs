use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetrust::four_sum::Solution;
use rand::{distributions::Uniform, prelude::Distribution, thread_rng, Rng};

fn benchmark_fun(c: &mut Criterion) {
    let mut rng = thread_rng();
    let dist = Uniform::new(-1000, 1000);
    let target = rng.sample(dist);
    let nums = {
        let mut nums: Vec<i32> = dist.sample_iter(&mut rng).take(1000).collect();

        let n = nums.len() - 1;
        for _ in 0..100 {
            let i = rng.sample(Uniform::new(2, n));
            nums.push(target - (nums[i] + nums[i - 1] + nums[i - 2]));
        }

        nums
    };

    c.bench_function("four_sum", |b| {
        b.iter(|| Solution::four_sum(black_box(nums.clone()), black_box(target)))
    });
}

criterion_group!(benches, benchmark_fun);
criterion_main!(benches);
