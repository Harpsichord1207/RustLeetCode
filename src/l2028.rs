pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let sm = rolls.iter().sum::<i32>();
        let s = (rolls.len() as i32 + n) * mean;
        let sn = s - sm;

        if sn < n || sn > n * 6 {
            return vec![];
        }

        // 找到n个1~6的i32其和为sn
        let remain = (sn % n) as usize;
        let mut ans = vec![sn / n; n as usize];
        for i in 0..remain {
            ans[i] += 1;
        }

        ans
    }
}