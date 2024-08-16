mod l126;
mod l306;
mod l368;
mod l438;
mod l463;
mod l541;
mod l543;
mod l796;
mod l859;
mod l997;
mod l1021;
mod l1395;
mod l1756;
mod l2582;
mod l3238;

use l859::Solution;

fn main() {
    let resp = Solution::buddy_strings("cba".to_string(), "abc".to_string());
    println!("{:?}", resp);
}
