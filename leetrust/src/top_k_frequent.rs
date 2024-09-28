//! 347. Top K Frequent Elements
//! https://leetcode.com/problems/top-k-frequent-elements/

impl Solution {
    pub fn top_k_frequent(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        nums.sort_unstable();

        let mut key = nums[0];
        let mut count = 0;

        let mut frequencies: Vec<(i32, i32)> = nums
            .into_iter()
            .filter_map(|num| {
                if num == key {
                    count += 1;
                    None
                } else {
                    let result = (key, count);
                    key = num;
                    count = 1;
                    Some(result)
                }
            })
            .collect();
        frequencies.push((key, count));
        frequencies.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        frequencies
            .into_iter()
            .take(k as usize)
            .map(|x| x.0)
            .collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
