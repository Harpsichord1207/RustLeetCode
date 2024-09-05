use std::collections::HashMap;

pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn beautiful_substrings(s: String, k: i32) -> i32 {
        let mut length = 0;
        for i in 1..=s.len() {
            if i*i % k as usize == 0 {
                length = i;
                break
            }
        }
        if length == 0 {
            return 0;
        }
        let mut prefix_map = HashMap::<(usize, usize), i32>::new();
        prefix_map.insert((0, 0), 1);

        let mut v = 0;
        let mut v_c = 0;
        let mut ans = 0;

        for (_, ch) in s.chars().enumerate() {
            if "aeiou".contains(ch) {
                v += 1;
                v_c += 1;
            } else {
                v_c -= 1;
            }

            v = v % length;
            if let Some(&count) = prefix_map.get(&(v, v_c)) {
                ans += count;
            }
            *prefix_map.entry((v, v_c)).or_insert(0) += 1;
        }

        ans
    }
}