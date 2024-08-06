pub struct Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        s.chars().collect::<Vec<_>>().chunks(k as usize).enumerate().map(
            |(i, ck)| {
                let mut new_ck = ck.iter().cloned().collect::<Vec<char>>();
                if i % 2 == 0 {
                    new_ck.reverse();
                }
                new_ck.iter().collect::<String>()
            }
        ).collect()
    }
}