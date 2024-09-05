use std::collections::HashMap;

pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut result = i64::MIN;
        let mut current_sum = 0i64;

        let nums: Vec<i64> = nums.into_iter().map(|n| n as i64).collect();
        let k = k as i64;

        let mut sum_map = HashMap::new();

        for num in nums {
            if sum_map.get(&num).unwrap_or(&i64::MAX) > &current_sum {
                sum_map.insert(num, current_sum);
            }
            current_sum += num;

            for key in [num + k, num - k] {
                if sum_map.contains_key(&key) {
                    result = result.max(current_sum - sum_map.get(&key).unwrap());
                }
            }
        }

        if result == i64::MIN {
            0
        } else {
            result
        }
    }
}
