//! 956. Tallest Billboard
//! https://leetcode.com/problems/tallest-billboard

use std::collections::{btree_map::Entry, BTreeMap};
impl Solution {
    pub fn tallest_billboard(mut rods: Vec<i32>) -> i32 {
        let half_sum = rods.iter().sum::<i32>() / 2;

        rods.sort_unstable();
        let mut memo: BTreeMap<i32, Vec<_>> = BTreeMap::new();

        for (i, &x) in rods.iter().enumerate() {
            if x > half_sum {
                break;
            }

            memo.entry(x)
                .and_modify(|v| v.push(vec![i]))
                .or_insert_with(|| vec![vec![i]]);

            for (&k, v) in memo.clone().iter() {
                let sum = x + k;

                if sum > half_sum {
                    break;
                }

                let mut v = v
                    .iter()
                    .filter_map(|s| {
                        if s.contains(&i) {
                            None
                        } else {
                            let mut s = s.clone();
                            s.push(i);
                            Some(s)
                        }
                    })
                    .collect();

                match memo.entry(sum) {
                    Entry::Vacant(entry) => {
                        entry.insert(v);
                    }
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().append(&mut v);
                    }
                }
            }
        }

        memo.into_iter()
            .rev()
            .find_map(|(k, v)| {
                for s0 in &v {
                    if v.iter().any(|s1| !s1.iter().any(|x| s0.contains(x))) {
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
        assert_eq!(0, Solution::tallest_billboard(vec![1, 2]));
        assert_eq!(
            0,
            Solution::tallest_billboard(vec![1, 2, 4, 8, 16, 32, 64, 128, 256])
        );
        assert_eq!(
            255,
            Solution::tallest_billboard(vec![1, 2, 4, 8, 16, 32, 64, 128, 255])
        );
    }
}
