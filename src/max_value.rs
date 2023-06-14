//! 1802. Maximum Value at a Given Index in a Bounded Array
//! https://leetcode.com/problems/maximum-value-at-a-given-index-in-a-bounded-array

// The constraints `abs(nums[i] - nums[i+1]) <= 1`
// means that any `nums[i]` is either the same as its neighbors, or differs by 1.
// To maximize `nums[i]` means to minimize all `nums[j]` where `j != i`,
// let's say `a = nums[i]`, then `nums[i - k] == nums[i + k] == a - k` is optimal.
// Since all numbers need to be possitive, it's capped to
// `nums[i - k] == nums[i + k] == max(1, a - k)` for all `k <= i` and `k < n - i`.
// `nums[0] == nums[i - i] == max(1, a - i)`
// `nums[n - 1] == nums[i + (n - i - 1)] == max(1, a - (n - i - 1))`

// Let `j` be the index of the last `1` before `i`, assuming it exists,
// `nums[j] == 1 == nums[i - k] == a - k` means `k == a - 1` means `j == i - a + 1`,
// therefore the slice `nums[..j] == nums[..i - a + 1]`,
// is either empty or all 1s, so it sums up to its length, which is `max(0, i - a + 1)`.

// let `l` be the index of the first `1` after `i`, assuming it exists,
// `nums[l] == 1 == nums[i + k] == a - k` means `k == a - 1` means `l == i + a - 1`,
// therefore the slice `nums[l + 1..] == nums[i + a..]`,
// is either empty or all 1s, so it sums up to its length, which is `max(0, n - i - a)`.

// The slice `nums[j..i] == 1..a` for `j >= 0`,
// or to cap it to the first element of `nums`,
// `nums[j..i] == max(1, a - i)..a` for all `j`

// The slice `nums[i + 1..l] == rev(1..a)` for `l <= n`,
// or to cap it to the last element of `nums`,
// `nums[i + 1..l] == rev(max(1, a - n + i + 1))..a)` for all `l`

// None of the four slices above contain `nums[i]` itself.
// Now summing up all the parts:
// `nums.sum() == a + max(0, i - a + 1) + (max(1, a - i)..a).sum() + (max(1, a - n + i + 1)..a).sum() + max(0, n - i - a)`

// The objective is to find either:
// the smallest `a` such that `nums.sum() > max_sum`, then the answer would be `a - 1`.
// or the smallest `a` such that `nums.sum() = max_sum`, and the answer would just be `a`.

// Since `nums` contains only possitive numbers, the minimum sum would be `n`.
// `a > max_sum - nums.len()` guarantees `nums.sum() >= max_sum`,
// therefore `a` is at most `max_sum - n + 1`.
// `a <= max_sum / num.len()` guarantees `nums.sum() <= max_sum`,
// therefore `a` is at least `max_sum / n`.

use std::cmp::max;

impl Solution {
    pub fn max_value(n: i32, i: i32, max_sum: i32) -> i32 {
        for a in (max_sum / n)..=(max_sum - n + 1) {
            let sum = max(0, i - a + 1)
                + (max(1, a - i)..a).sum::<i32>()
                + (max(1, a - n + i + 1)..a).sum::<i32>()
                + max(0, n - i - a)
                + a;

            match sum.cmp(&max_sum) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Equal => return a,
                std::cmp::Ordering::Greater => return a - 1,
            }
        }

        unreachable!()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        // Explanation: nums = [1,2,2,1] is one array that satisfies all the conditions.
        // There are no arrays that satisfy all the conditions
        // and have nums[2] == 3, so 2 is the maximum nums[2].
        assert_eq!(2, Solution::max_value(4, 2, 6));
        assert_eq!(1, Solution::max_value(1, 0, 1));
        assert_eq!(1, Solution::max_value(2, 0, 2));
        assert_eq!(2, Solution::max_value(1, 0, 2));
        assert_eq!(2, Solution::max_value(2, 0, 3));
        // [2, 3, 2, 1, 1, 1]
        assert_eq!(3, Solution::max_value(6, 1, 10));
        // [1, 2, 3, 2, 1, 1]
        assert_eq!(3, Solution::max_value(6, 2, 12));
        // [1, 1, 1, 2, 3, 4]
        assert_eq!(4, Solution::max_value(6, 5, 12));

        // Test against overflow.
        Solution::max_value(1000000000, 0, 1000000000);
        Solution::max_value(1000000000, 500000000, 1000000000);
        Solution::max_value(1000000000, 999999999, 1000000000);
    }
}
