use std::collections::HashSet;

pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn buddy_strings(s: String, goal: String) -> bool {
        let len = s.len();
        let s = s.chars().collect::<Vec<char>>();
        let goal = goal.chars().collect::<Vec<char>>();
        if goal.len() != len {
            return false;
        }

        let mut diff_index = Vec::new();
        let mut set = HashSet::new();
        for i in 0..len {
            let ch1: char = s[i];
            set.insert(ch1);
            if ch1 != goal[i] {
                diff_index.push(i);
                if diff_index.len() > 2 {
                    return false
                }
            }
        }

        if diff_index.len() == 2 {
            return s[diff_index[0]] == goal[diff_index[1]] && s[diff_index[1]] == goal[diff_index[0]];
        } else if diff_index.len() == 0 {
            return set.len() < len;
        }
        false
    }
}