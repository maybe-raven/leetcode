//! 75. Sort Colors
//! https://leetcode.com/problems/sort-colors

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // let i, j, k such that, i <= j <= k and
        // nums[..i] are all 0, nums[i..j] are all 1, nums[k..] are all 2,
        // which means nums[..j] and nums[k..] are all sorted,
        // and nums[i] is the first 1,
        // and nums[j] is the first unsorted number,
        // and nums[k - 1] is the last unsorted number.

        let mut i = 0;
        let mut j = 0;
        let mut k = nums.len();

        while j < k {
            match nums[j] {
                0 => {
                    nums.swap(i, j);
                    i += 1;
                    j += 1;
                }
                1 => {
                    j += 1;
                }
                2 => {
                    nums.swap(j, k - 1);
                    k -= 1;
                }
                _ => panic!("Input must contain only 0 | 1 | 2."),
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Uniform, thread_rng, Rng};
    use std::iter::repeat_with;

    fn sort_and_assert(mut nums: Vec<i32>) {
        let n = nums.len();
        Solution::sort_colors(&mut nums);
        assert_eq!(n, nums.len());
        let mut last = nums.first().expect("Should not be empty.");
        println!("{:?}", nums);
        assert!(nums.iter().all(|x| if x < last {
            false
        } else {
            last = x;
            true
        }));
    }

    #[test]
    fn test_sort_colors() {
        sort_and_assert(vec![2, 0, 2, 1, 1, 0]);
        sort_and_assert(vec![2, 0, 1]);
        sort_and_assert(vec![0]);
    }

    #[test]
    fn test_sort_colors_with_rand() {
        let mut rng = thread_rng();
        let dist = Uniform::new(0, 3);

        for _ in 0..100 {
            let nums: Vec<i32> = repeat_with(|| rng.sample(dist)).take(300).collect();
            sort_and_assert(nums);
        }
    }
}
