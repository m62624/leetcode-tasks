impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x <= 1 {
            return x;
        }

        let x = x as i64;
        let (mut left, mut right) = (2, x / 2);

        while left <= right {
            let mid = left + (right - left) / 2;
            let target = mid * mid;

            if target == x {
                right = mid;
                break;
            } else if target < x {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        right as i32
    }
}
