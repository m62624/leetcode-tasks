impl Solution {
      pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (index, _)| {
                acc.push((0..=index).fold(0, |mut acc, index| {
                    acc += nums[index];
                    acc
                }));
                acc
            })
    }
}
