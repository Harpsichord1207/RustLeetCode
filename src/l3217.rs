use std::collections::HashSet;

pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    #[allow(dead_code)]
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode { next: None, val: 0 };
        let mut prev = &mut dummy;
        let mut current = head;

        let nums = nums.into_iter().collect::<HashSet<i32>>();

        while let Some(mut n) = current {
            current = n.next.take();
            if !nums.contains(&n.val) {
                prev.next = Some(n);
                // prev = &mut *prev.next.as_mut().unwrap();
                prev = prev.next.as_mut().unwrap();
            }

        }
        dummy.next
    }
}