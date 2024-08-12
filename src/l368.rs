pub struct Solution;

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let length = nums.len();
        let mut s_nums = nums.clone();
        s_nums.sort_unstable();

        let mut dp = vec![1; length];
        let mut pre = vec![length; length];

        let mut max_size = 1;
        let mut max_index = 0;

        for i in 1..length {
            for j in 0..i {
                if s_nums[i] % s_nums[j] != 0 {
                    continue;
                }
                if dp[i] < dp[j] + 1 {
                    dp[i] = dp[j] + 1;
                    pre[i] = j;
                }
            }

            if dp[i] > max_size {
                max_size = dp[i];
                max_index = i;
            }
        }

        let mut subset = vec![];
        while max_index != length {
            subset.push(s_nums[max_index]);
            max_index = pre[max_index];
        }
        subset.reverse();
        subset
    }
}