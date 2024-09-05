use std::collections::HashMap;

pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let mut m = HashMap::new();
        let mut res_vec = vec![0; n as usize];
        for game in pick {
            if let &[p, c] = &game[..] {
                let count_map = m.entry(p).or_insert_with(HashMap::new);
                let current_count = count_map.entry(c).or_insert(0);
                if res_vec[p as usize] == 0 && *current_count >= p {
                    res_vec[p as usize] = 1;
                }
                *current_count += 1;
            }
        }
        res_vec.iter().sum()
    }
}