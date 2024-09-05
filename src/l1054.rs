use std::collections::{BinaryHeap, HashMap};

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut h: BinaryHeap<(i32, i32)> = barcodes.into_iter().fold(
            HashMap::new(), |mut acc, b| {
                *acc.entry(b).or_insert(0) += 1;
                acc
            }
        ).into_iter().map(|(b, c)| (c, b)).collect();

        let mut result = Vec::new();
        let mut pre_freq = 0;
        let mut pre_code = 0;
        while let Some((c, b)) = h.pop() {
            result.push(b);
            if pre_freq > 0 {
                h.push((pre_freq, pre_code))
            }
            pre_freq = c - 1;
            pre_code = b;
        }
        result
    }
}
