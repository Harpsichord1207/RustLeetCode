mod l126;
mod l306;
mod l368;
mod l463;
mod l541;
mod l543;
mod l796;
mod l3238;


use l368::Solution;

fn main() {
    let resp = Solution::largest_divisible_subset(vec![1, 2, 3]);
    println!("{:?}", resp);
}
