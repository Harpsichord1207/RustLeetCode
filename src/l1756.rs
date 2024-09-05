use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = is_water.len();
        let n = is_water[0].len();
        let mut area = Vec::with_capacity(m);
        let mut points = VecDeque::new();
        for i in 0..m {
            let mut r = Vec::with_capacity(n);
            for j in 0..n {
                if is_water[i][j] == 1 {
                    r.push(0);
                    points.push_back((i, j));
                } else {
                    r.push(-1);
                }
            }
            area.push(r);
        }

        while !points.is_empty() {
            let size = points.len();
            for _ in 0..size {
                let (i, j) = points.pop_front().unwrap();
                let v = area[i][j];

                if i > 0 && area[i-1][j] == -1 {
                    area[i-1][j] = v + 1;
                    points.push_back((i-1, j));
                }

                if i + 1 < m && area[i+1][j] == - 1 {
                    area[i+1][j] = v + 1;
                    points.push_back((i+1, j));
                }

                if j > 0 && area[i][j-1] == -1 {
                    area[i][j-1] = v + 1;
                    points.push_back((i, j-1));
                }

                if j + 1 < n && area[i][j+1] == -1 {
                    area[i][j+1] = v + 1;
                    points.push_back((i, j+1));
                }

            }
        }
        area
    }

}