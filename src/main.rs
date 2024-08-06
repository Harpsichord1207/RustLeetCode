mod l541;

use l541::Solution;

fn main() {
    let resp = Solution::reverse_str("123456".to_string(), 2);
    println!("{:?}", resp);
}
