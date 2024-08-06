use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

// Definition for a binary tree node.
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
      right: None
    }
  }
}
struct Solution;


impl Solution {
    fn max_deep(root: Option<Rc<RefCell<TreeNode>>>, d: &mut i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let node = root.unwrap();
        let right_deep = Self::max_deep(node.borrow().right.clone(), d);
        let left_deep = Self::max_deep(node.borrow().left.clone(), d);
        *d = max(*d, right_deep + left_deep);
        max(right_deep, left_deep) + 1
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut d = 0;
        Self::max_deep(root, &mut d);
        d
    }
}
