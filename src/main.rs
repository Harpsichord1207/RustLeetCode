mod l126;
mod l306;
mod l368;
mod l438;
mod l443;
mod l463;
mod l541;
mod l543;
mod l796;
mod l846;
mod l855;
mod l859;
mod l874;
mod l997;
mod l1021;
mod l1054;
mod l1357;
mod l1395;
mod l1592;
mod l1756;
mod l1945;
mod l2028;
mod l2130;
mod l2131;
mod l2582;
mod l2947;
mod l3026;
mod l3238;


use l2028::Solution;

fn main() {
    let resp = Solution::missing_rolls(vec![6,3,4,3,5,3], 1, 6);
    println!("{:?}", resp);
}
