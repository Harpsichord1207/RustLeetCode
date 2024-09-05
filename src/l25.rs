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
        Solution::reverse(head, k)
    }
    pub fn reverse(mut head:Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut new_head: Option<Box<ListNode>> = None;
        let mut i = 0;
        while let Some(mut head_val) = head.take() {
            head = head_val.next.take();
            head_val.next = new_head;
            new_head = Some(head_val);
            i += 1;
            if i == k {
                break;
            }
        }
        if i != k {
            // i < k时再换回来
            return Solution::reverse(new_head, i);
        }
        // we now have two lists:
        // head -> rest of list
        // new_head -> reversed list
        // i cannot figure out how in rust you do this in one pass,
        // so we will go walk the list again to get the tail of new_head and make it head
        // 'append'
        head = Solution::reverse(head, k);  // now head -> rest is k-grouped

        // combine:  new_head -> reversed list -> head -> rest of reversed list
        let mut tailw = &mut new_head;
        if let Some(ref mut tail_val) = tailw {
            let mut tail = tail_val;
            while let Some(ref mut tail_next) = tail.next {
                tail = tail_next;
            }
            tail.next = head;
        }
        new_head
    }
}
