//! 2090. K Radius Subarray Averages
//! https://leetcode.com/problems/k-radius-subarray-averages

impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let window_size = 2 * k + 1;
        let mut results = vec![-1; nums.len()];

        if nums.len() < window_size {
            return results;
        }

        let mut sum = nums[..window_size]
            .iter()
            .map(|&x| x as usize)
            .sum::<usize>();
        results[k] = (sum / window_size) as i32;
        sum -= nums[0] as usize;

        for ((&a, &b), average) in nums
            .iter()
            .zip(nums.iter().skip(window_size - 1))
            .zip(results.iter_mut().skip(k))
            .skip(1)
        {
            sum += b as usize;
            *average = (sum / window_size) as i32;
            sum -= a as usize;
        }

        results
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            vec![-1, -1, -1, 5, 4, 4, -1, -1, -1],
            Solution::get_averages(vec![7, 4, 3, 9, 1, 8, 5, 2, 6], 3)
        );
        assert_eq!(vec![100000], Solution::get_averages(vec![100000], 0));
        assert_eq!(vec![-1], Solution::get_averages(vec![8], 10));
    }
}
