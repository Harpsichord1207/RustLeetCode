pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut s = s.chars().into_iter().map(
            |c| (c as i32 - 'a' as i32 + 1).to_string()
        ).collect::<String>();

        let mut kk = 0;
        while kk < k {
            kk += 1;
            s = s.chars().into_iter().map(
                |c| c.to_digit(10).unwrap() as i32
            ).sum::<i32>().to_string()
        }

        s.parse().unwrap()
    }
}