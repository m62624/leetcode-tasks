impl Solution {
   pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        let mut index = 0;

        while left < right {
            let mid = left + (right - left) / 2;
            index = mid;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if target < nums[index] {
            index as i32
        } else {
            index as i32 + 1
        }
    }
}
