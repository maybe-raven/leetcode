//! 451. Sort Characters By Frequency
//! https://leetcode.com/problems/sort-characters-by-frequency

use std::{collections::HashMap, iter::repeat};

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut map: HashMap<u8, usize> = HashMap::new();
        let byte_vec = s.into_bytes();

        for byte in byte_vec {
            map.entry(byte).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut byte_vec: Vec<(u8, usize)> = map.into_iter().collect();
        byte_vec.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        let byte_vec: Vec<u8> = byte_vec
            .into_iter()
            .flat_map(|(k, count)| repeat(k).take(count))
            .collect();

        String::from_utf8(byte_vec).expect("Need to ensure the function produces valid Strings.")
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_sort() {
        assert!(["eert".to_string(), "eetr".to_string()]
            .contains(&Solution::frequency_sort("tree".to_string())));
        assert!(["cccaaa".to_string(), "aaaccc".to_string()]
            .contains(&Solution::frequency_sort("aaaccc".to_string())));
        assert!(["bbaA".to_string(), "bbAa".to_string()]
            .contains(&Solution::frequency_sort("Aabb".to_string())));
        let result = Solution::frequency_sort("loveleetcode".to_string());
        assert_eq!(&result[..4], "eeee");
        assert!(["ooll", "lloo"].contains(&&result[4..8]));
        assert!("vcdt".chars().all(|ch| result[8..].contains(ch)));
    }
}
