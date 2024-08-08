pub struct Solution;

impl Solution {
    pub fn is_additive_number(num: String) -> bool {

        fn check(_num1: &str, _num2: &str, _start: usize, _num: &str) -> bool {
            let _value1: u64 = _num1.parse().unwrap();
            let _value2: u64 = _num2.parse().unwrap();
            let _sum_str = (_value1 + _value2).to_string();
            let _next_start = _start + _sum_str.len();
            if _next_start > _num.len() || _num[_start.._next_start] != _sum_str {
                return false;
            }

            if _next_start == _num.len() {
                return true;
            }

            check(_num2, &_sum_str, _next_start, _num)
        }

        for i in 1..num.len() {
            for j in i+1..num.len() {
                let num1 = &num[..i];
                let num2 = &num[i..j];

                if num1.len() > 1 && num1.starts_with('0') {
                    continue;
                }
                if num2.len() > 1 && num2.starts_with('0') {
                    continue;
                }

                if check(num1, num2, j, &num) {
                    return true;
                }
            }
        }

        false
    }
}
