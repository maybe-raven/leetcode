//! 18. 4Sum
//! https://leetcode.com/problems/4sum

// let q = target / 4
// let a <= b <= c <= d and a + b + c + d == target
// if a < q then d > q and vice versa
// therefore a <= q <= d is always true
// practically speaking, we check a == b == c == d == q before the loop,
// so we only check a and d where a < q < d

// if target > 0 and d > target then a < 0
// if target < 0 and a < target then d > 0

// let sums_memo[sum] = [(b1, c1), (b2, c2), ...]
// such that for all pairs of b and c: b + c == sum

// b + c == target - (a + d)
// since a < q, a + d <= q1 + n1
// where n1 is the largest number, and q1 is the largest number that's smaller than q
// since d > q, a + d >= n2 + q2
// where n2 is the smallest number, and q2 is the smallest number that's larger than q
// target - (q1 + nums.last()) <= b + c <= target - (q2 + nums.first())

use std::collections::HashMap;

const EMPTY_INPUT_ERR: &'static str = "Input vec cannot be empty.";

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }

        let counter = {
            let mut counter: HashMap<i32, usize> = HashMap::new();

            for num in nums.drain(..) {
                counter.entry(num).and_modify(|v| *v += 1).or_insert(1);
            }

            counter
        };

        nums.extend(counter.keys());
        nums.sort_unstable();

        let &first = nums.first().expect(EMPTY_INPUT_ERR);
        let &last = nums.last().expect(EMPTY_INPUT_ERR);

        let mut results = Vec::new();
        let quarter = (target as f64) / 4.0;

        if quarter == quarter.trunc() {
            let quarter = quarter as i32;
            if let Some(&count) = counter.get(&quarter) {
                if count >= 4 {
                    results.push(vec![quarter, quarter, quarter, quarter]);
                }
            }
        }

        if (first as f64) >= quarter || (last as f64) <= quarter {
            return results;
        }

        let a_range_end = nums.partition_point(|&x| (x as f64) < quarter);
        let d_range_start = if (nums[a_range_end] as f64) == quarter {
            a_range_end + 1
        } else {
            a_range_end
        };

        for (ia, &a) in nums[..a_range_end].iter().enumerate() {
            for (id, &d) in nums[d_range_start..].iter().enumerate() {
                if target.is_positive() && d > target && a >= 0 {
                    break;
                }

                if target.is_negative() && a < target && d <= 0 {
                    continue;
                }

                let diff = target - a - d;

                for &b in &nums[ia..(id + d_range_start + 1)] {
                    let c = diff.checked_sub(b);
                    if c.is_none() {
                        continue;
                    }
                    let c = c.expect("Continue if None.");

                    if c < b {
                        break;
                    }

                    if c > d {
                        continue;
                    }

                    if let Some(&c_count) = counter.get(&c) {
                        if ((a == b && a == c) || (b == d && c == d)) && c_count < 3 {
                            continue;
                        } else if b == c && c_count < 2 {
                            continue;
                        }

                        if (a == b || a == c) && counter[&a] < 2 {
                            continue;
                        }

                        if (b == d || c == d) && counter[&d] < 2 {
                            continue;
                        }

                        results.push(vec![a, b, c, d]);
                    }
                }
            }
        }

        results
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Uniform, thread_rng, Rng};
    use std::{collections::HashSet, iter::repeat_with};

    fn slow_four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut results = HashSet::new();
        for (i1, &n1) in nums.iter().enumerate() {
            for (i2, &n2) in nums.iter().enumerate() {
                for (i3, &n3) in nums.iter().enumerate() {
                    for (i4, &n4) in nums.iter().enumerate() {
                        if i1 == i2 || i1 == i3 || i1 == i4 || i2 == i3 || i2 == i4 || i3 == i4 {
                            continue;
                        }

                        if let Some(sum) = n1
                            .checked_add(n2)
                            .and_then(|sum| sum.checked_add(n3))
                            .and_then(|sum| sum.checked_add(n4))
                        {
                            if sum != target {
                                continue;
                            }

                            let mut quadruplet = [n1, n2, n3, n4];
                            quadruplet.sort_unstable();
                            results.insert(quadruplet);
                        }
                    }
                }
            }
        }

        results.into_iter().map(Vec::from).collect()
    }

    fn do_test_and_assert(nums: Vec<i32>, target: i32, expected_num_results: usize) {
        println!("nums: {:?}; target: {}", nums, target);
        let mut results = Solution::four_sum(nums, target);
        println!("{:?}", results);
        sort_and_assert_results(target, &mut results);
        assert_eq!(results.len(), expected_num_results);
    }

    fn sort_and_assert_results(target: i32, results: &mut Vec<Vec<i32>>) {
        results.iter_mut().for_each(|quadruplet| {
            assert_eq!(target, quadruplet.iter().sum::<i32>());
            quadruplet.sort_unstable();
        });
        let count = results.len();
        results.sort_unstable();
        results.dedup();
        assert_eq!(count, results.len());
    }

    #[test]
    fn test_four_sum() {
        do_test_and_assert(vec![1, 0, -1, 0, -2, 2], 0, 3);
        do_test_and_assert(vec![2, 2, 2, 2, 2], 8, 1);
        do_test_and_assert(vec![2, 3, 4, 5, 6, 7], 8, 0);
        do_test_and_assert(vec![-1, 0, 1, 2, 3, 4], -8, 0);
        do_test_and_assert(vec![-100, -50, -25, 25, 50, 100], -10, 0);
        do_test_and_assert(vec![1, 2, 3, 4, 21, 22, 23, 24, 25], 20, 0);
        do_test_and_assert(vec![0, 0, 0, 0], 1, 0);
        do_test_and_assert(vec![1, 1, 1], 8, 0);
        do_test_and_assert(vec![], 8, 0);
        do_test_and_assert(
            vec![
                2397, -4364, 7070, -6284, -8303, -3041, -7671, 3045, 9148, -4872, 583, 1833, 6679,
                8960, -8075, 8211, 5803, -2539, 9103, 9858, -8275, -3921, 5630, -6760, 8440, -4943,
                -9517, 6080, -4560, -2487, 9200, 7554, -1443, -7055, 2734, 9263, -2536, -3413,
                -4464, 4212, 3710, 8236, -2891, 3702, -5173, 9066, 8875, 2048, -8468, 171, 5227,
                5881, 748, 6118, 6538, -855034095, -855029412, -855023876,
            ],
            -855017937,
            3,
        );
    }

    #[test]
    fn test_against_slow() {
        let mut rng = thread_rng();
        let dist = Uniform::new(-1000, 1000);
        let target_dist = Uniform::new(-1000, 1000);
        let guaranteed_results_dist = Uniform::new(0, 10);

        for _ in 0..1000 {
            let target = rng.sample(target_dist);
            let mut nums: Vec<i32> = repeat_with(|| rng.sample(dist)).take(100).collect();

            for _ in 0..rng.sample(guaranteed_results_dist) {
                let i = rng.sample(Uniform::new(2, nums.len()));
                nums.push(target - (nums[i] + nums[i - 1] + nums[i - 2]))
            }

            let mut results = slow_four_sum(nums.clone(), target);
            sort_and_assert_results(target, &mut results);
            do_test_and_assert(nums, target, results.len());
        }
    }

    #[test]
    fn test_overflow() {
        let mut rng = thread_rng();
        let dist = Uniform::new(-1000000000, 1000000000);
        let target_dist = Uniform::new(-1000000000, 1000000000);

        for _ in 0..100 {
            let target = rng.sample(target_dist);
            let nums: Vec<i32> = repeat_with(|| rng.sample(dist)).take(200).collect();

            let mut results = Solution::four_sum(nums.clone(), target);
            sort_and_assert_results(target, &mut results);
        }
    }
}
