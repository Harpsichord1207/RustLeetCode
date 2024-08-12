mod l126;
mod l306;
mod l368;
mod l438;
mod l463;
mod l541;
mod l543;
mod l796;
mod l3238;

use l438::Solution;

fn main() {
    let resp = Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string());
    println!("{:?}", resp);
}
