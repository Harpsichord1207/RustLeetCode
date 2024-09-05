pub struct Solution;


impl Solution {
    #[allow(dead_code)]
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let leni32 = len as i32;
        for i in 0..len {
            let v = nums[i];
            if v <= 0 || v > leni32 {
                nums[i] = leni32 + 1;
            }
        }

        for i in 0..len {
            let v = nums[i].abs();
            if v <= leni32 {
                nums[v as usize - 1] = -nums[(v as usize-1)].abs();
            }
        }

        for i in 0..len {
            if nums[i] > 0 {
                return i as i32 + 1;
            }
        }
        leni32 + 1
    }
}