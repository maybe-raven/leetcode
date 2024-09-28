use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{seq::IteratorRandom, thread_rng};

struct Range {
    start: i32,
    end: i32,
}

impl ToString for Range {
    fn to_string(&self) -> String {
        if self.end == self.start {
            self.end.to_string()
        } else {
            let a = self.start.to_string();
            let b = self.end.to_string();
            let mut string = String::with_capacity(a.len() + b.len() + 2);

            string.push_str(a.as_str());
            string.push_str("->");
            string.push_str(b.as_str());

            string
        }
    }
}

#[derive(Debug)]
struct MapContiguousRanges<I> {
    start: i32,
    last: i32,
    iter: Option<I>,
}

impl<I> Default for MapContiguousRanges<I> {
    fn default() -> Self {
        Self {
            start: 0,
            last: 0,
            iter: None,
        }
    }
}

impl<I> MapContiguousRanges<I>
where
    I: Iterator<Item = i32>,
{
    fn new(mut iter: I) -> Self {
        if let Some(head) = iter.next() {
            Self {
                start: head,
                last: head,
                iter: Some(iter),
            }
        } else {
            Self::default()
        }
    }
}

impl<I> Iterator for MapContiguousRanges<I>
where
    I: Iterator<Item = i32>,
{
    type Item = Range;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(iter) = &mut self.iter else {return None; };

        while let Some(x) = iter.next() {
            if self.last == x - 1 {
                self.last = x;
                continue;
            }

            let range = Range {
                start: self.start,
                end: self.last,
            };
            self.start = x;
            self.last = x;
            return Some(range);
        }

        self.iter = None;
        Some(Range {
            start: self.start,
            end: self.last,
        })
    }
}

trait ToMapContiguousRanges: Iterator<Item = i32> + Sized {
    fn map_contiguous_ranges(self) -> MapContiguousRanges<Self> {
        MapContiguousRanges::new(self)
    }
}

impl<I> ToMapContiguousRanges for I where I: Iterator<Item = i32> + Sized {}

impl IterSolution {
    fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        nums.into_iter()
            .map_contiguous_ranges()
            .map(|r| r.to_string())
            .collect()
    }
}

struct IterSolution;

fn add_range(results: &mut Vec<String>, start: i32, end: i32) {
    if end == start {
        results.push(end.to_string());
    } else {
        let a = start.to_string();
        let b = end.to_string();
        let mut range = String::with_capacity(a.len() + b.len() + 2);

        range.push_str(a.as_str());
        range.push_str("->");
        range.push_str(b.as_str());

        results.push(range)
    }
}

impl LoopSolution {
    fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut results = Vec::new();
        let (&(mut start), nums) = match nums.split_first() {
            Some(split_result) => split_result,
            None => return results,
        };
        let mut last = start;

        for &x in nums {
            if last != x - 1 {
                add_range(&mut results, start, last);
                start = x;
            }
            last = x;
        }

        add_range(&mut results, start, last);

        results
    }
}

struct LoopSolution;

fn benchmark_fun(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut group = c.benchmark_group("iter_vs_loop");

    for n in [1000, 1000000, 1000000000] {
        let nums = {
            let mut nums = (i32::MIN..=i32::MAX).choose_multiple(&mut rng, n);
            nums.sort_unstable();
            nums
        };

        group.bench_function(BenchmarkId::new("iter", n), |b| {
            b.iter(|| IterSolution::summary_ranges(black_box(nums.clone())));
        });
        group.bench_function(BenchmarkId::new("loop", n), |b| {
            b.iter(|| LoopSolution::summary_ranges(black_box(nums.clone())));
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_fun);
criterion_main!(benches);
