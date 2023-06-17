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

use std::collections::BTreeSet;

fn calc_permutations(nums: &[i32]) -> usize {
    // The first position is always fixed, so only one possibility.
    // The second position have two.

    let Some((head, tail)) = nums.split_first() else { return 1; };
    if tail.len() <= 1 {
        println!("nums: {:?}; p: {}", nums, 1);
        return 1;
    }

    // This assumes that `Iterator::partition` doesn't disturb original ordering.
    let (a, b): (Vec<i32>, Vec<i32>) = tail.into_iter().partition(|&x| match head.cmp(x) {
        std::cmp::Ordering::Less => false,
        std::cmp::Ordering::Equal => {
            unreachable!("We are asserting that the input contains only unique elements.")
        }
        std::cmp::Ordering::Greater => true,
    });

    if a.is_empty() || b.is_empty() {
        println!("nums: {:?}; p: {}", nums, 1);
        return 1;
    }

    let p = get_spliced_permutations_electric_boogaloo(a.len(), b.len());
    let a = calc_permutations(&a);
    let b = calc_permutations(&b);
    let result = a * b * p;
    println!("nums: {:?}; p: {} * {} * {} = {}", nums, a, b, p, result);
    result

    // We need to find the number of permutations of slicing two arrays.
    // [1, 2, 3]
    // [4, 5]
    // permut 1, [2, 3], [4, 5] {
    //   permut 2, [3], [4, 5] {
    //     permut 3, [], [4, 5] {
    //       [1,2,3,4,5]
    //     }
    //     permut 4, [3], [5] {
    //       [1,2,4,3,5]
    //       [1,2,4,5,3]
    //     }
    //   }
    //   permut 4, [5], [2, 3] {
    //     permut 2, [5], [3] {
    //       [1,4,2,3,5]
    //       [1,4,2,5,3]
    //     }
    //     permut 5, [], [2, 3] {
    //       [1,4,5,2,3]
    //     }
    //   }
    // }
    // permut 4, [5], [1, 2, 3] {
    //   permut 1, [5], [2, 3] {
    //     permut 2, [5], [3] {
    //       [4,1,2,3,5]
    //       [4,1,2,5,3]
    //     }
    //     permut 5, [], [2, 3] {
    //       [4,1,5,2,3]
    //     }
    //   }
    //   permut 5, [], [1, 2, 3] {
    //     [4,5,1,2,3]
    //   }
    // }
    //
    // [2, 1, 3] | [2, 3, 1]
    // 4
    // [6, 5, 7] | [6, 7, 5]
    //
    //
    // at 0, 2 choices
    // at 1, 4 choices
    // at 2, 2 * 2 * 2 choices
    // one of those is the orginal
    // so
}

fn get_spliced_permutations(a: &[i32], b: &[i32]) -> usize {
    // print!("a: {:?}; b: {:?}; ", a, b);

    let Some((&a_head, a_tail)) = a.split_first() else {
        // println!("p: {}", 1);
        return 1;
    };
    let Some((&b_head, b_tail)) = b.split_first() else {
        // println!("p: {}", 1);
        return 1;
    };

    let result = get_spliced_permutations(a_tail, b) + get_spliced_permutations(a, b_tail);
    // println!("p: {}", result);
    result
}

fn get_splices(a: &[i32], b: &[i32]) -> Vec<Vec<i32>> {
    let Some((&a_head, a_tail)) = a.split_first() else {
        return vec![b.to_owned()];
    };
    let Some((&b_head, b_tail)) = b.split_first() else {
        return vec![a.to_owned()];
    };

    let mut results0 = get_splices(a_tail, b);
    let mut results1 = get_splices(a, b_tail);

    for x in &mut results0 {
        x.insert(0, a_head);
    }
    for x in &mut results1 {
        x.insert(0, b_head);
    }
    results0.extend_from_slice(&results1);

    results0
}

fn get_spliced_permutations_electric_boogaloo(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        1
    } else {
        get_spliced_permutations_electric_boogaloo(a - 1, b)
            + get_spliced_permutations_electric_boogaloo(a, b - 1)
    }
}

fn get_permutations(nums: &[i32]) -> BTreeSet<Vec<i32>> {
    match nums.len() {
        0 => unreachable!(),
        1 => BTreeSet::from([nums.to_owned()]),
        n => (0..n)
            .flat_map(|i| {
                let mut nums = nums.to_owned();
                let x = nums.swap_remove(i);
                get_permutations(&nums).into_iter().map(move |mut v| {
                    v.insert(0, x);
                    v
                })
            })
            .collect(),
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

    use rand::{rngs::StdRng, seq::IteratorRandom, SeedableRng};

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
    fn test_get_permutations() {
        // println!("{:?}\n", get_permutations(&[1]));
        // println!("{:?}\n", get_permutations(&[1, 2]));
        // println!("{:?}\n", get_permutations(&[1, 2, 3]));
        // println!("{:?}\n", get_permutations(&[1, 2, 3, 4]));
        // println!("{:?}\n", get_permutations(&[1, 2, 3, 4, 5]));
        get_permutations(&[3, 4, 5, 6, 1, 2])
            .into_iter()
            .filter(|v| match v.as_slice() {
                [3, 4, _tail @ ..] | [3, 1, _tail @ ..] => true,
                _ => false,
            })
            .filter(|v| match v.as_slice() {
                [_, 4, 2, _tail @ ..]
                | [_, 1, 5, _tail @ ..]
                | [_, 1, 6, _tail @ ..]
                | [_, 4, 6, _tail @ ..] => false,
                _ => true,
            })
            .for_each(|v| println!("{:?}", v));
    }

    #[test]
    fn test_get_splices() {
        println!("{:?}", get_splices(&[1, 2, 3], &[4, 5]));
    }

    #[test]
    fn test_get_spliced_permutations() {
        const RAND_SEED: u64 = 7812718947180417012;
        let mut rng = StdRng::seed_from_u64(RAND_SEED);

        let mut v = Vec::new();

        for a in 2..12 {
            for b in 2..12 {
                let nums = (-20..=20).choose_multiple(&mut rng, a + b);
                let numsa = &nums[..a];
                let numsb = &nums[a..];

                let result = get_spliced_permutations_electric_boogaloo(a, b);
                v.push((a, b, result));
                assert_eq!(result, get_splices(numsa, numsb).len());
            }
        }

        v.sort_by_key(|x| x.2);
        v.dedup_by_key(|x| x.2);
        for (a, b, p) in v {
            println!("a: {}; b: {}; p: {}", a, b, p,);
        }
    }
}
