mod l463;
mod l541;
mod l543;
mod l796;
mod l3238;

use l3238::Solution;

fn main() {
    let resp = Solution::winning_player_count(2, vec![vec![0, 8], vec![0, 3]]);
    println!("{:?}", resp);
}
