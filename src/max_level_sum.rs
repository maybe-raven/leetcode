//! 1161. Maximum Level Sum of a Binary Tree
//! https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/

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
    pub fn collect_sum(&self, out: &mut Vec<i32>, level: usize) {
        if let Some(sum) = out.get_mut(level) {
            *sum += self.val;
        } else {
            assert_eq!(out.len(), level, "We go deeper one level at a time, and at every level a value is added. If this assertion fails that means we either jumped a level, or some node forgot to add its value.");
            out.push(self.val);
        }

        if let Some(left) = &self.left {
            left.borrow().deref().collect_sum(out, level + 1);
        }

        if let Some(right) = &self.right {
            right.borrow().deref().collect_sum(out, level + 1);
        }
    }
}

use std::cell::RefCell;
use std::cmp::Ordering;
use std::ops::Deref;
use std::rc::Rc;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const ERRMSG: &str = "The number of nodes in the tree is in the range [1, 10^4].";

        let mut v = Vec::new();
        root.expect(ERRMSG).borrow().deref().collect_sum(&mut v, 0);
        let (i, _) = v
            .into_iter()
            .enumerate()
            .max_by(|a, b| match a.1.cmp(&b.1) {
                Ordering::Equal => Ordering::Greater,
                ordering => ordering,
            })
            .expect(ERRMSG);

        (i + 1) as i32
    }
}

pub struct Solution;
