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

fn calc_permutations(nums: &[i32]) -> usize {
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

    if a.is_empty() || b.is_empty() {
        return 1;
    }

    calc_permutations(&a) * calc_permutations(&b) * calc_spliced_permutations(a.len(), b.len())
}

fn calc_spliced_permutations(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        1
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
        // [3, 1, 2, 4, 5, 6]
        // [3, 1, 4, 2, 5, 6]
        // [3, 1, 4, 5, 2, 6]
        // [3, 1, 4, 5, 6, 2]
        // [3, 4, 1, 2, 5, 6]
        // [3, 4, 1, 5, 2, 6]
        // [3, 4, 1, 5, 6, 2]
        // [3, 4, 5, 1, 2, 6]
        // [3, 4, 5, 1, 6, 2]
        // [3, 4, 5, 6, 1, 2]
        assert_eq!(0, Solution::num_of_ways(vec![1, 2, 3]));
    }

    #[test]
    fn test_get_spliced_permutations() {
        let mut v = Vec::new();

        for a in 2..12 {
            for b in 2..12 {
                let result = calc_spliced_permutations(a, b);
                v.push((a, b, result));
            }
        }

        v.sort_by_key(|x| x.2);
        v.dedup_by_key(|x| x.2);
        for (a, b, p) in v {
            println!("a: {}; b: {}; p: {}", a, b, p,);
        }
    }
}
