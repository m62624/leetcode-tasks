impl Solution {
     pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k = 0;
        if nums.len() > 0 {
            let mut index = 0;
            let mut prev = nums[0];

            while index < nums.len() {
                if index > 0 && prev == nums[index] {
                    nums.remove(index);
                    index -= 1;
                } else {
                    k += 1;
                }

                prev = nums[index];
                index += 1;
            }
        }
        k
    }
}
