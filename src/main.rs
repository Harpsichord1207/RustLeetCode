mod l126;
mod l306;
mod l368;
mod l438;
mod l463;
mod l541;
mod l543;
mod l796;
mod l1021;
mod l3238;

use l1021::Solution;

fn main() {
    let resp = Solution::remove_outer_parentheses("(()())(())".to_string());
    println!("{:?}", resp);
}
