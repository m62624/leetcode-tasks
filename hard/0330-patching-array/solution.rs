impl Solution {
      pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let (mut value, mut index, mut count, n) = (1i64, 0, 0, n as i64);
        while value <= n {
            if index < nums.len() && (nums[index] as i64) <= value {
                value += nums[index] as i64;
                index += 1;
            } else {
                value *= 2;
                count += 1;
            }
        }
        count
    }
}
