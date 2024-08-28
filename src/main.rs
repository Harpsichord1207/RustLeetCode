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
mod l2130;
mod l2131;
mod l2582;
mod l3026;
mod l3238;


use l2131::Solution;

fn main() {
    let string = vec!["qo","fo","fq","qf","fo","ff","qq","qf","of","of","oo","of","of","qf","qf","of"];
    // let string = vec!["lc","cl","gg"];
    let resp = Solution::longest_palindrome(string.into_iter().map(|s|s.to_string()).collect());
    println!("{:?}", resp);
}
