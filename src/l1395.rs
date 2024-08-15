pub struct Solution;


impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {

        let mut result = 0;

        for (i, r) in rating.iter().enumerate() {
            let mut left_larger = 0;
            let mut left_smaller = 0;

            for j in 0..i {
                if &rating[j] > r {
                    left_larger += 1;
                } else {
                    left_smaller += 1;
                }
            }

            let mut right_larger = 0;
            let mut right_smaller = 0;

            for j in i+1..rating.len() {
                if &rating[j] > r {
                    right_larger += 1;
                } else {
                    right_smaller += 1;
                }
            }

            result += left_larger * right_smaller;
            result += left_smaller * right_larger;
        }

        result
    }
}