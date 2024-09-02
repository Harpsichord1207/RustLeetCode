use std::collections::HashMap;

pub struct Solution;



impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let len = hand.len();
        if len % group_size as usize != 0 {
            return false;
        }

        let mut m = HashMap::new();
        let mut sh = Vec::new();
        for h in hand {
            *m.entry(h).or_insert(0) += 1;
            sh.push(h);
        }
        sh.sort_unstable();
        for h in sh {
            let &c = m.get(&h).unwrap_or(&0);
            if c == 0 {
                continue;
            }

            for i in 0..group_size {
                let nh = h + i;
                if let Some(next_c) = m.get_mut(&nh) {
                    if *next_c < c {
                        return false;
                    }
                    *next_c -= c;
                } else {
                    return false;
                }
            }
        }
        true

    }
}