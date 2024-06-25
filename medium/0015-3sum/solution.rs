use std::collections::HashSet;

impl Solution {
 pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let len = nums.len();
        if len >= 3 {
            nums.sort_unstable();

            for start in 0..len {
                if start > 0 && nums[start - 1] == nums[start] {
                    continue;
                }

                let target = -nums[start];

                let mut mid = start + 1;
                let mut right = len - 1;

                while mid < right {
                    let sum = nums[mid] + nums[right];

                    if sum == target {
                        result.push(vec![nums[start], nums[mid], nums[right]]);

                        while mid < right && nums[mid] == nums[mid + 1] {
                            mid += 1;
                        }

                        while right > mid && nums[right - 1] == nums[right] {
                            right -= 1;
                        }

                        mid += 1;
                        right -= 1;
                    } else if sum < target {
                        mid += 1;
                    } else {
                        right -= 1;
                    }
                }
            }
        }

        result
    }
}
