//! 956. Tallest Billboard
//! https://leetcode.com/problems/tallest-billboard

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SmallIndex(u8);

impl SmallIndex {
    const INVALID_VALUE: u8 = 100;
    const INVALID: Self = Self(Self::INVALID_VALUE);

    fn is_invalid(self) -> bool {
        self.0 == Self::INVALID_VALUE
    }
}

impl Solution {
    pub fn tallest_billboard(mut rods: Vec<i32>) -> i32 {
        let half_sum = rods.iter().sum::<i32>() / 2;

        rods.sort_unstable();
        let mut memo: BTreeMap<i32, Vec<[SmallIndex; 20]>> = BTreeMap::new();

        for (i, &x) in rods.iter().enumerate() {
            if x > half_sum {
                break;
            }

            let i = i as u8;
            let mut inner = [SmallIndex::INVALID; 20];
            inner[0] = SmallIndex(i);
            memo.entry(x)
                .and_modify(|v| v.push(inner))
                .or_insert_with(|| vec![inner]);

            for (&k, v) in memo.clone().iter() {
                let sum = x + k;

                if sum > half_sum {
                    break;
                }

                let mut v = v
                    .iter()
                    .filter_map(|s| {
                        if s.contains(&SmallIndex(i)) {
                            None
                        } else {
                            let mut s = s.clone();
                            for j in s.iter_mut() {
                                if j.is_invalid() {
                                    *j = SmallIndex(i);
                                    break;
                                }
                            }
                            Some(s)
                        }
                    })
                    .collect();

                memo.entry(sum)
                    .and_modify(|v0| v0.append(&mut v))
                    .or_insert(v);
            }
        }

        memo.into_iter()
            .rev()
            .find_map(|(k, v)| {
                for s0 in &v {
                    if v.iter()
                        .any(|s1| !s0.iter().any(|x| x.is_invalid() ^ s1.contains(x)))
                    {
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
    }

    #[test]
    fn failure() {
        // [102,101,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100]
        unimplemented!();
    }
}
