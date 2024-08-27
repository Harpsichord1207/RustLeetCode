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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut values = vec![];
        let mut h = head;
        while let Some(n) = h {
            values.push(n.val);
            h = n.next;
        }
        let mut res = i32::MIN;
        let len = values.len();
        for i in 0..len / 2 {
            res = res.max(values[i] + values[len-i-1])
        }
        res
    }
}