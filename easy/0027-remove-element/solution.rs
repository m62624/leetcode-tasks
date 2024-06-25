impl Solution {
 pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() > 0 {
            nums.sort_unstable();
            let len = nums.len() - 1;
            let mut left = 0;
            let mut right = len;

            'main_loop: while left <= right {
                let mid = left + (right - left) / 2;
                if nums[mid] == val {
                    let (mut left, mut right) = (mid, mid);

                    loop {
                        if left >= 1 && nums[mid] == nums[left - 1] {
                            left -= 1;
                        } else if right < len && nums[mid] == nums[right + 1] {
                            right += 1;
                        } else {
                            nums.splice(left.min(mid)..right + 1, []);
                            break 'main_loop;
                        }
                    }
                } else if nums[mid] < val {
                    match mid.checked_add(1) {
                        Some(l) => left = l,
                        None => break,
                    }
                } else {
                    match mid.checked_sub(1) {
                        Some(r) => right = r,
                        None => break,
                    }
                }
            }
        }
        nums.len() as i32
    }
}
