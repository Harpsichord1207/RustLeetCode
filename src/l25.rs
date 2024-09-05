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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut node = &mut head;

        for _ in 0..k {
            if let Some(n) = node {
                node = &mut n.next;
            } else {
                return head;
            }
        }

        let mut remain = Self::reverse_k_group(node.take(), k);

        while let Some(mut n) = head {
            head = n.next;
            n.next = remain;
            remain = Some(n);
        }

        remain
    }
}
