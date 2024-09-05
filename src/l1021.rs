pub struct Solution;


#[allow(dead_code)]
impl Solution {

    pub fn remove_outer_parentheses(s: String) -> String {
        let mut ss = String::new();
        let mut n = 0;

        for c in s.chars() {
            if c == '(' {
                n += 1;
                if n == 1 {
                    continue;
                }
            } else {
                n -= 1;
                if n == 0 {
                    continue;
                }
            }
            ss.push(c);
        }
        ss
    }

}
