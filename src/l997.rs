pub struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 {
            return 1;
        }

        let mut trust_count = vec![0; (n + 1) as usize];
        let mut trusted_count = vec![0; (n + 1) as usize];

        for pair in trust {
            trust_count[pair[1] as usize] += 1;
            trusted_count[pair[0] as usize] += 1;
        }

        for i in 1..=n as usize {
            if trust_count[i] == n - 1 && trusted_count[i] == 0 {
                return i as i32;
            }
        }

        -1
    }
}