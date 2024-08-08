mod l306;
mod l463;
mod l541;
mod l543;
mod l796;
mod l3238;

use l306::Solution;

fn main() {
    let resp = Solution::is_additive_number("198019823962".to_string());
    println!("{:?}", resp);
}
