mod l126;
mod l306;
mod l368;
mod l438;
mod l463;
mod l541;
mod l543;
mod l796;
mod l997;
mod l1021;
mod l1395;
mod l1756;
mod l2582;
mod l3238;

use l1395::Solution;

fn main() {
    let resp = Solution::num_teams(vec![2,5,3,4,1]);
    println!("{:?}", resp);
}
