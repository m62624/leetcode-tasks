use std::collections::BinaryHeap;

impl Solution {
     pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut median = 0.0;
        let mut heap = BinaryHeap::from(
            nums1
                .iter()
                .chain(nums2.iter())
                .copied()
                .collect::<Vec<i32>>(),
        );

        let len = heap.len();

        if len % 2 == 0 {
            (1..len / 2).for_each(|_| {
                heap.pop();
            });
            if let Some(val1) = heap.pop() {
                if let Some(val2) = heap.pop() {
                    median = (val1 + val2) as f64 / 2.0;
                }
            }
        } else {
            (0..len / 2).for_each(|_| {
                heap.pop();
            });
            if let Some(val) = heap.pop() {
                median = val as f64;
            }
        }
        median
    }
}
