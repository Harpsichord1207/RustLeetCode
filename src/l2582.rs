pub struct Solution;

impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let time = time % (2 * n - 2);
        if time < n {
            time + 1
        } else {
            n + n - time - 1
        }
    }
}

