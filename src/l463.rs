pub struct Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let row_count = grid.len();
        let col_count = grid[0].len();
        let mut p = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                if v == 0 {
                    continue;
                }
                if i == 0 || grid[i-1][j] == 0 {
                    p += 1;
                }
                if i == row_count - 1 || grid[i+1][j] == 0 {
                    p += 1;
                }
                if j == 0 || grid[i][j-1] == 0 {
                    p += 1;
                }
                if j == col_count - 1|| grid[i][j+1] == 0 {
                    p += 1;
                }
            }
        }
        p
    }
}