use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {

        let mut m = HashMap::<String, usize>::new();
        let mut res = 0;
        let mut dup_chars_unpaired = 0;

        for word in words {
            let r_word = word.chars().into_iter().rev().collect::<String>();
            let count = m.entry(r_word.clone()).or_insert(0);

            if word == r_word {
                if *count > 0 {
                    *count -= 1;
                    res += 4;
                    dup_chars_unpaired -= 1;
                } else {
                    *count += 1;
                    dup_chars_unpaired += 1;
                }
            } else {
                if *count > 0 {
                    *count -= 1;
                    res += 4;
                } else {
                    *m.entry(word).or_insert(0) += 1;
                }

            }
        }
        res + if dup_chars_unpaired > 0 {2} else {0}
    }
}
