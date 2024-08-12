pub struct Solution;

impl Solution {
    fn build_counter(s: &str) -> [usize; 26] {
        let mut counter = [0; 26];
        for c in s.chars() {
            counter[c as usize - 'a' as usize] += 1;
        }
        counter
    }

    fn compare_counter(c1: &[usize; 26], c2: &[usize; 26]) -> bool {
        c1 == c2
    }

    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let lp = p.len();
        let ls = s.len();
        if ls < lp {
            return vec![];
        }

        let counter_p = Self::build_counter(&p);
        let mut counter_s = [0; 26];
        let mut res = vec![];
        let chars = s.chars().collect::<Vec<char>>();
        let mut i = 0;
        for c in &chars {
            counter_s[*c as usize - 'a' as usize] += 1;

            if i >= lp {
                let left_char = chars[i - lp];
                counter_s[left_char as usize - 'a' as usize] -= 1;
            }

            if i >= lp - 1 && Self::compare_counter(&counter_s, &counter_p) {
                res.push((i + 1 - lp) as i32);
            }
            i += 1;
        }
        res
    }
}
