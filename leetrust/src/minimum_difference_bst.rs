//! 530. Minimum Absolute Difference in BST
//! https://leetcode.com/problems/minimum-absolute-difference-in-bst

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl TreeNode {
    pub fn add_to_vec(&self, out: &mut Vec<i32>) {
        if let Some(ref left) = self.left {
            left.borrow().add_to_vec(out);
        }

        out.push(self.val);

        if let Some(ref right) = self.right {
            right.borrow().add_to_vec(out);
        }
    }
}

impl<const N: usize> From<[i32; N]> for TreeNode {
    fn from(mut value: [i32; N]) -> Self {
        value.sort_unstable();
        Self::from(value.as_slice())
    }
}

impl From<&[i32]> for TreeNode {
    fn from(arr: &[i32]) -> Self {
        let i = arr.len() / 2;
        let (left, val, right) = match arr.len() {
            3 => (
                Some(TreeNode::new(arr[0])),
                arr[1],
                Some(TreeNode::new(arr[2])),
            ),
            2 => (Some(TreeNode::new(arr[0])), arr[1], None),
            _ => (
                Some(TreeNode::from(&arr[..i])),
                arr[i],
                Some(TreeNode::from(&arr[i + 1..])),
            ),
        };

        let wrap = |node| Rc::new(RefCell::new(node));

        Self {
            val,
            left: left.map(wrap),
            right: right.map(wrap),
        }
    }
}

impl From<&TreeNode> for Vec<i32> {
    fn from(root: &TreeNode) -> Self {
        let mut result = Vec::new();
        root.add_to_vec(&mut result);
        result
    }
}

use std::cell::RefCell;
use std::cmp::min;
use std::ops::Deref;
use std::rc::Rc;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const ERRMSG: &str =
            "The number of nodes in input is expected to be in the range [2, 10^4].";

        let v: Vec<i32> = root.expect(ERRMSG).borrow().deref().into();

        let mut min_diff = i32::MAX;
        let (&(mut previous), v) = v.split_first().expect(ERRMSG);
        for &x in v {
            min_diff = min(min_diff, (x - previous).abs());
            previous = x;
        }

        min_diff
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node_conversion() {
        assert_eq!(Vec::from(&TreeNode::from([4, 2, 6, 1, 3])), [1, 2, 3, 4, 6]);
        assert_eq!(
            Vec::from(&TreeNode::from([1, 0, 48, 12, 49])),
            [0, 1, 12, 48, 49]
        );
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            1,
            Solution::get_minimum_difference(Some(Rc::new(RefCell::new(TreeNode::from([
                4, 2, 6, 1, 3
            ])))))
        );
        assert_eq!(
            1,
            Solution::get_minimum_difference(Some(Rc::new(RefCell::new(TreeNode::from([
                1, 0, 48, 12, 49
            ])))))
        );
    }
}
