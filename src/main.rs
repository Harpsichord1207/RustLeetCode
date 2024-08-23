mod l126;
mod l306;
mod l368;
mod l438;
mod l443;
mod l463;
mod l541;
mod l543;
mod l796;
mod l855;
mod l859;
mod l997;
mod l1021;
mod l1054;
mod l1395;
mod l1756;
mod l2582;
mod l3026;
mod l3238;

use l443::Solution;

fn main() {
    let mut string = vec!['a','b','b','b','b','b','b','b','b','b','b','b','b'];
    let resp = Solution::compress(&mut string);
    println!("{:?}", resp);
}
