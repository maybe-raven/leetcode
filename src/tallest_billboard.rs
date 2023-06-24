//! 956. Tallest Billboard
//! https://leetcode.com/problems/tallest-billboard

use std::collections::BTreeMap;

impl Solution {
    pub fn tallest_billboard(mut rods: Vec<i32>) -> i32 {
        let half_sum = rods.iter().sum::<i32>() / 2;

        const N: usize = 20;
        assert!(rods.len() <= N);

        rods.sort_unstable();
        let Some((&(mut previous), tail)) = rods.split_first() else { return 0; };

        let mut rods: [i32; N] = [0; N];
        let mut counts: [u8; N] = [0; N];
        let mut iter = rods.iter_mut().zip(counts.iter_mut());
        let (mut num, mut count) = iter.next().unwrap();
        *num = previous;
        *count = 1;

        for &x in tail {
            if x == previous {
                *count += 1;
            } else {
                (num, count) = iter.next().unwrap();
                *num = x;
                *count = 1;
            }
            previous = x;
        }

        let mut memo: BTreeMap<i32, Vec<[u8; 20]>> = BTreeMap::new();

        for ((i, &x), &count) in rods.iter().enumerate().zip(&counts) {
            if x > half_sum || x == 0 {
                break;
            }

            // TODO: Find me some real variable names. This is way too fucking toxic!
            for (&k, v) in memo.clone().iter() {
                for r in 1..=count {
                    let sum = x * r as i32 + k;

                    if sum > half_sum {
                        break;
                    }

                    let mut v = v.clone();
                    for s in v.iter_mut() {
                        s[i] = r;
                    }

                    memo.entry(sum)
                        .and_modify(|v0| v0.append(&mut v))
                        .or_insert(v);
                }
            }

            for r in 1..=count {
                let sum = x * r as i32;
                if sum > half_sum {
                    continue;
                }

                let mut s = [0; 20];
                s[i] = r;
                memo.entry(sum)
                    .and_modify(|v| v.push(s))
                    .or_insert_with(|| vec![s]);
            }
        }

        memo.into_iter()
            .rev()
            .find_map(|(k, v)| {
                for s0 in &v {
                    'l: for s1 in &v {
                        for ((&r0, &r1), &count) in s0.iter().zip(s1).zip(&counts) {
                            if count < r0 + r1 {
                                continue 'l;
                            }
                        }
                        return Some(k);
                    }
                }

                None
            })
            .unwrap_or(0) as i32
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
