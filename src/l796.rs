pub struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        return s.len() == goal.len() && s.repeat(2).contains(goal.as_str())
    }
}