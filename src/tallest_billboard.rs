//! 956. Tallest Billboard
//! https://leetcode.com/problems/tallest-billboard

#![allow(unused, dead_code)]

use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};
impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        let mut memo: BTreeMap<i32, Vec<_>> = BTreeMap::new();

        for (i, &x) in rods.iter().enumerate() {
            memo.entry(x)
                .and_modify(|v| v.push(BTreeSet::from([i])))
                .or_insert_with(|| vec![BTreeSet::from([i])]);

            for (&k, v) in memo.clone().iter() {
                let sum = x + k;
                let mut v = v
                    .iter()
                    .filter_map(|s| {
                        if s.contains(&i) {
                            None
                        } else {
                            let mut s = s.clone();
                            s.insert(i);
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
                    if v.iter().any(|s1| s0.is_disjoint(s1)) {
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
