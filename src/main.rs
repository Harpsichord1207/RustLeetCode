mod l463;
mod l541;
mod l543;
mod l796;

use l541::Solution;

fn main() {
    let resp = Solution::reverse_str("123456".to_string(), 2);
    println!("{:?}", resp);
}
