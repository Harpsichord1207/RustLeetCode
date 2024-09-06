mod l25;
mod l41;
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
mod l3217;
mod l3238;


use l41::Solution;

fn main() {
    let resp = Solution::first_missing_positive(vec![3,4,-1,1]);
    println!("{:?}", resp);
}
