impl Solution {
  pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let (mut left, mut right) = (0, 0);
        let (mut count, mut odd_count) = (0, 0);

        let mut result = 0;

        while right < nums.len() {
            if nums[right] % 2 == 1 {
                odd_count += 1;
                count = 0;
            }

            while odd_count == k {
                if nums[left] % 2 == 1 {
                    odd_count -= 1;
                }
                left += 1;
                count += 1;
            }

            result += count;
            right += 1;
        }

        result
    }
}
