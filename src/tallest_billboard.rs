//! 956. Tallest Billboard
//! https://leetcode.com/problems/tallest-billboard

use std::{cmp::Ordering, collections::BTreeMap};

impl Solution {
    pub fn tallest_billboard(mut rods: Vec<i32>) -> i32 {
        rods.sort_unstable_by(|a, b| b.cmp(a));

        let mut sum = rods.iter().sum::<i32>();

        let n = match rods.iter().enumerate().find_map(|(i, &x)| {
            let double = 2 * x;

            match double.cmp(&sum) {
                Ordering::Less => Some(Err(i)),
                Ordering::Equal => Some(Ok(x)),
                Ordering::Greater => {
                    sum -= x;
                    None
                }
            }
        }) {
            Some(Err(n)) => n,
            Some(Ok(x)) => return x,
            None => return 0,
        };

        if rods.len() - n < 2 {
            return 0;
        }

        let mut memo = BTreeMap::from([(0, 0)]);

        for x in rods.into_iter().skip(n) {
            for (diff, taller) in memo.clone() {
                let mut height = taller + x;
                memo.entry(diff + x)
                    .and_modify(|h| *h = (*h).max(height))
                    .or_insert(height);

                if diff == 0 {
                    continue;
                }

                if diff < x {
                    height -= diff;
                    memo.entry(x - diff)
                        .and_modify(|h| *h = (*h).max(height))
                        .or_insert(height);
                } else {
                    memo.entry(diff - x)
                        .and_modify(|h| *h = (*h).max(taller))
                        .or_insert(taller);
                }
            }
        }

        memo.get(&0).copied().unwrap_or(0)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(6, Solution::tallest_billboard(vec![1, 2, 3, 6]));
        assert_eq!(10, Solution::tallest_billboard(vec![1, 2, 3, 4, 5, 6]));
        assert_eq!(6, Solution::tallest_billboard(vec![3, 4, 3, 3, 2]));
        assert_eq!(0, Solution::tallest_billboard(vec![1, 2]));
        assert_eq!(
            0,
            Solution::tallest_billboard(vec![1, 2, 4, 8, 16, 32, 64, 128, 256])
        );
        assert_eq!(
            255,
            Solution::tallest_billboard(vec![1, 2, 4, 8, 16, 32, 64, 128, 255])
        );
        assert_eq!(
            900,
            Solution::tallest_billboard(vec![
                102, 101, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100,
            ])
        );
    }
}
