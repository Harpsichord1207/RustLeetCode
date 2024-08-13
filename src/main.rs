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
mod l3238;

use l997::Solution;

fn main() {
    let resp = Solution::find_judge(4, vec![vec![1,2], vec![1,3], vec![2,1], vec![2,3], vec![1,4], vec![4,3], vec![4,1]]);
    println!("{:?}", resp);
}
