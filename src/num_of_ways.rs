//! 1569. Number of Ways to Reorder Array to Get Same BST
//! https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst

// Assuming input vec contains only unique numbers.
// let [head, tail @ ..] = nums,
// Seperate tail into two vecs (a, b), where a[i] < nums and b[i] > nums for all i.
// Recursively repeat, seperate a into (a1, a2), where a1[i] < nums and a2[i] > nums for all i;
// seperate b into (b1, b2), where b1[i] < nums and b2[i] > nums for all i;
// this will give me the ordering.
// Find all permutations of nums that create the same ordering based on the above algorithm.

// Find the first number that is greater than `head`, call it `x`, and its index `i`.
// Find the first number that is less than `head`, call it `y`, and its index `j`.
// Find the number of permutations of slice `nums[..max(i, j)]` that
// keep x being the first number that is greater than `head`, and
// y being first number that is less than `head`.

use std::u128;

fn calc_permutations(nums: &[i32]) -> u128 {
    let Some((head, tail)) = nums.split_first() else { return 1; };
    if tail.len() <= 1 {
        return 1;
    }

    let (a, b): (Vec<i32>, Vec<i32>) = tail.into_iter().partition(|&x| match head.cmp(x) {
        std::cmp::Ordering::Less => false,
        std::cmp::Ordering::Equal => {
            unreachable!("We are asserting that the input contains only unique elements.")
        }
        std::cmp::Ordering::Greater => true,
    });

    calc_permutations(&a) * calc_permutations(&b) * calc_spliced_permutations(a.len(), b.len())
}

fn calc_spliced_permutations(a: usize, b: usize) -> u128 {
    if a == 0 || b == 0 {
        1
    } else if a == 1 {
        (b + 1) as u128
    } else if b == 1 {
        (a + 1) as u128
    } else {
        calc_spliced_permutations(a - 1, b) + calc_spliced_permutations(a, b - 1)
    }
}

impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        ((calc_permutations(&nums) - 1) % 1000000007) as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(1, Solution::num_of_ways(vec![2, 1, 3]));
        assert_eq!(5, Solution::num_of_ways(vec![3, 4, 5, 1, 2]));
        assert_eq!(9, Solution::num_of_ways(vec![3, 4, 5, 6, 1, 2]));
        assert_eq!(
            840839,
            Solution::num_of_ways(vec![6, 9, 11, 15, 1, 12, 3, 2, 7, 8, 14, 4, 5, 13, 10])
        );
        Solution::num_of_ways(vec![
            2, 6, -18, -3, -16, -15, 7, -13, -12, 13, -10, -9, 10, 16, 1,
        ]);
        Solution::num_of_ways(vec![
            10, -19, 15, -17, -16, -15, -14, -13, 18, 17, -10, -9, -8, 14, -6, 19, -4, -3, -2, -1,
            0, 1, 2, 3, 4, 13, 6, 7, 8, 9,
        ]);
        Solution::num_of_ways(vec![
            -100, -99, 81, 63, -96, -95, -94, -93, -92, -91, 38, 59, 72, 6, -86, -85, 86, -83, -82,
            39, 77, -79, -78, 60, 87, -75, 10, 13, -72, -71, -70, -69, 94, 91, 18, 70, -64, -63,
            89, 12, 93, 17, 96, -57, 24, -55, -54, 50, -52, 9, -50, -49, -48, 75, 42, 98, 29, -43,
            16, 56, 61, -39, -38, -37, -36, -35, 28, -33, 79, -31, 55, 8, -28, -27, -26, -25, 11,
            46, -22, -21, 99, -19, 97, 68, 71, -15, 85, -13, 83, 0, 31, -9, 45, -7, -6, -5, -4, -3,
            -2, -1,
        ]);
    }
}
