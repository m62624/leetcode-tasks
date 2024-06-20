impl Solution {
    pub fn check(bd: &[i32], day: i32, m: i32, k: i32) -> bool {
        let mut flowers = 0;

        bd.iter().fold(0, |mut acc, bloom| {
            if bloom <= &day {
                flowers += 1;
                if flowers == k {
                    acc += 1;
                    flowers = 0;
                }
            } else {
                flowers = 0;
            }
            acc
        }) >= m
    }

    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let mut min_days = -1;

        if !(bloom_day.len() < (m * k) as usize) {
            let mut left = 0;
            if let Some(mut right) = bloom_day.iter().max().cloned() {
                while left <= right {
                    let mid = left + (right - left) / 2;
                    if Self::check(&bloom_day, mid, m, k) {
                        min_days = mid;
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                }
            }
        }
        min_days
    }
}

