pub struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {

        let mut pre_char = ' ';
        let mut pre_count = 0;
        let mut index = 0usize;
        let size = chars.len();

        for i in 0..size {
            let ch = chars[i];
            if ch != pre_char {
                pre_char = ch;

                if pre_count > 1 {
                    for _c in pre_count.to_string().chars() {
                        chars[index] = _c;
                        index += 1;
                    }
                }

                chars[index] = ch;
                index += 1;
                pre_count = 1;

            } else {
                pre_count += 1;
            }
        }
        if pre_count > 1 {
            for _c in pre_count.to_string().chars() {
                chars[index] = _c;
                index += 1;
            }
        }
        index as i32
    }

}