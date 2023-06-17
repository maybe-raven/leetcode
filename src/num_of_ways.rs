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

use std::{collections::HashMap, u128};

const MAX: u128 = 1000000007;

trait MulMod {
    fn mul_mod(self, other: Self) -> Self;
}

impl MulMod for u128 {
    fn mul_mod(self, other: Self) -> Self {
        (self * other) % MAX
    }
}

trait AddMod {
    fn add_mod(self, other: Self) -> Self;
}

impl AddMod for u128 {
    fn add_mod(self, other: Self) -> Self {
        (self + other) % MAX
    }
}

fn calc_permutations(nums: &[i32], memo: &mut HashMap<(usize, usize), u128>) -> u128 {
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

    calc_permutations(&a, memo)
        .mul_mod(calc_permutations(&b, memo))
        .mul_mod(calc_spliced_permutations(a.len(), b.len(), memo))
}

fn calc_spliced_permutations(a: usize, b: usize, memo: &mut HashMap<(usize, usize), u128>) -> u128 {
    if a == 0 || b == 0 {
        1
    } else if a == 1 {
        (b + 1) as u128
    } else if b == 1 {
        (a + 1) as u128
    } else {
        let key = if a > b { (b, a) } else { (a, b) };

        if let Some(&cached) = memo.get(&key) {
            cached
        } else {
            let value = calc_spliced_permutations(a - 1, b, memo)
                .add_mod(calc_spliced_permutations(a, b - 1, memo));
            memo.insert(key, value);
            value
        }
    }
}

impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let mut memo = HashMap::new();
        ((calc_permutations(&nums, &mut memo) - 1) % MAX) as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(0, Solution::num_of_ways(vec![1, 2, 3]));
        assert_eq!(1, Solution::num_of_ways(vec![2, 1, 3]));
        assert_eq!(5, Solution::num_of_ways(vec![3, 4, 5, 1, 2]));
        assert_eq!(9, Solution::num_of_ways(vec![3, 4, 5, 6, 1, 2]));
        assert_eq!(
            840839,
            Solution::num_of_ways(vec![6, 9, 11, 15, 1, 12, 3, 2, 7, 8, 14, 4, 5, 13, 10])
        );
        assert_eq!(
            936157466,
            Solution::num_of_ways(vec![
                31, 23, 14, 24, 15, 12, 25, 28, 5, 35, 17, 6, 9, 11, 1, 27, 18, 20, 2, 3, 33, 10,
                13, 4, 7, 36, 32, 29, 8, 30, 26, 19, 34, 22, 21, 16
            ])
        );
        Solution::num_of_ways(vec![
            2, 6, -18, -3, -16, -15, 7, -13, -12, 13, -10, -9, 10, 16, 1,
        ]);
        Solution::num_of_ways(vec![
            10, -19, 15, -17, -16, -15, -14, -13, 18, 17, -10, -9, -8, 14, -6, 19, -4, -3, -2, -1,
            0, 1, 2, 3, 4, 13, 6, 7, 8, 9,
        ]);
        Solution::num_of_ways(vec![
            -100, -99, 81, 63, -96, -95, -94, -93, -92, -91, 38, 59, 72, 6, -86, -85, 86, -83, -82,
            39, 77, -79, -78, 60, 87, -75, 10, 13, -72, -71, -70, -69, 94, 91, 18, 70, -64, -63,
            89, 12, 93, 17, 96, -57, 24, -55, -54, 50, -52, 9, -50, -49, -48, 75, 42, 98, 29, -43,
            16, 56, 61, -39, -38, -37, -36, -35, 28, -33, 79, -31, 55, 8, -28, -27, -26, -25, 11,
            46, -22, -21, 99, -19, 97, 68, 71, -15, 85, -13, 83, 0, 31, -9, 45, -7, -6, -5, -4, -3,
            -2, -1,
        ]);
        Solution::num_of_ways(vec![
            -150, -149, -148, 75, 54, -145, -144, -143, -142, -141, -140, -139, -138, 86, -136,
            -135, -134, -133, -132, 69, 64, 126, -128, -127, 95, -125, 65, 50, -122, -121, -120,
            -119, -118, -117, -116, 88, -114, -113, 136, -111, -110, -109, 94, -107, -106, -105,
            119, 97, -102, -101, -100, 62, -98, -97, 106, -95, -94, 125, -92, 130, -90, 135, 82,
            -87, -86, 116, 61, 81, 68, -81, 66, -79, 104, -77, -76, 102, -74, -73, 115, -71, -70,
            -69, 121, -67, -66, -65, 111, -63, -62, 87, 70, -59, 89, 63, -56, -55, -54, -53, -52,
            -51, 147, -49, -48, -47, 122, -45, -44, -43, -42, -41, -40, -39, 93, 53, 127, -35, -34,
            -33, 60, -31, -30, 80, -28, 132, -26, -25, -24, -23, -22, -21, 123, -19, 59, -17, -16,
            92, -14, 72, 73, -11, -10, -9, 134, 52, 99, -5, -4, -3, -2, -1, 0, 1, 131, 3, 4, 5,
            110, 7, 8, 9, 10, 11, 12, 13, 85, 15, 16, 17, 83, 19, 20, 21, 22, 124, 145, 25, 26, 27,
            28, 117, 30, 31, 32, 71, 34, 35, 148, 37, 38, 39, 79, 41, 42, 43, 107, 45, 46, 144, 48,
            49,
        ]);
        Solution::num_of_ways(vec![
            319, 316, -398, -397, -396, 234, 128, -393, -392, -391, -390, 131, 350, -387, -386,
            -385, -384, -383, 308, -381, -380, -379, -378, -377, -376, -375, -374, 320, -372, -371,
            -370, 106, -368, -367, -366, -365, -364, 245, 250, -361, -360, 365, -358, -357, -356,
            -355, -354, -353, -352, -351, 277, -349, -348, -347, 127, -345, -344, 358, -342, -341,
            -340, 223, 195, 393, 218, -335, -334, -333, 330, 228, -330, -329, -328, -327, -326,
            -325, -324, 391, -322, 159, -320, 183, -318, -317, 362, 171, -314, -313, 367, 174,
            -310, -309, 142, 364, -306, 289, -304, -303, -302, -301, -300, -299, -298, 148, 265,
            -295, 178, -293, 232, -291, -290, 290, -288, -287, -286, -285, -284, 140, 297, 150,
            -280, -279, 380, -277, -276, 121, -274, -273, 133, -271, 118, 199, -268, -267, -266,
            -265, -264, 109, -262, 241, 307, 387, -258, 268, 203, 104, -254, 170, 179, -251, -250,
            153, -248, -247, 260, -245, -244, -243, 354, 326, -240, 129, -238, 327, -236, -235,
            251, 202, 331, 372, -230, -229, -228, -227, -226, -225, -224, -223, 253, -221, -220,
            353, -218, -217, 224, -215, -214, 325, 149, -211, -210, -209, -208, 258, -206, 225,
            272, -203, 381, -201, 243, -199, -198, -197, -196, 261, -194, -193, 343, -191, -190,
            335, 371, 226, -186, -185, 256, 304, 370, -181, 276, -179, -178, 231, 227, 328, -174,
            -173, -172, -171, 248, -169, -168, -167, 116, -165, -164, 254, 147, 177, 209, -159,
            310, -157, -156, -155, -154, -153, -152, 173, -150, 340, 305, -147, -146, 255, 213,
            348, -142, -141, -140, 146, 269, -137, 194, 384, 184, 191, -132, 137, 135, -129, -128,
            359, 182, -125, -124, -123, -122, -121, -120, -119, 275, 267, 126, -115, 157, -113,
            299, -111, -110, -109, -108, -107, -106, -105, 317, -103, -102, 200, -100, -99, -98,
            329, -96, -95, -94, 282, 274, 130, 355, -89, 284, -87, -86, -85, 338, -83, 102, -81,
            229, -79, -78, -77, 285, -75, -74, -73, -72, -71, 124, -69, -68, -67, -66, -65, -64,
            369, 239, -61, 352, 192, -58, 376, -56, -55, 172, 336, 176, -51, -50, -49, -48, 204,
            -46, 221, -44, -43, -42, 169, -40, -39, -38, -37, -36, -35, -34, 302, -32, -31, -30,
            -29, 395, 154, -26, 394, 357, -23, 237, -21, 187, 207, 138, 398, -16, -15, 230, -13,
            -12, -11, 215, -9, -8, -7, -6, -5, -4, 287, -2, -1, 161, 216, 283, 145, 4, 293, 377,
            220, 8, 9, 10, 347, 12, 13, 14, 143, 122, 295, 18, 19, 20, 21, 108, 23, 24, 25, 26, 27,
            28, 298, 292, 366, 32, 33, 34, 35, 36, 101, 38, 39, 270, 41, 42, 43, 314, 309, 217, 47,
            303, 49, 50, 51, 52, 323, 301, 55, 56, 57, 119, 156, 60, 61, 62, 306, 64, 65, 151, 375,
            68, 69, 70, 190, 136, 115, 257, 75, 76, 77, 271, 79, 80, 81, 82, 83, 84, 85, 175, 87,
            88, 89, 322, 91, 92, 244, 94, 134, 96, 97, 98, 266,
        ]);
    }
}
