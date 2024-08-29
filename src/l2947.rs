use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn beautiful_substrings(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut ans = 0;

        for i in 0..s.len() {
            let mut vow = 0;
            let mut con = 0;

            for j in i..s.len() {
                if "aeiou".contains(s[j]) {
                    vow += 1;
                } else {
                    con += 1;
                }
                if (vow * con) % k == 0 && vow == con {
                    ans += 1;
                }
            }
        }

        ans
    }
}