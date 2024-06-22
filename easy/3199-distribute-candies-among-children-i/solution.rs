impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
    let mut count = 0;
        for i in 0..=limit {
            for j in 0..=limit {
                let k = n - i - j;
                if k >= 0 && k <= limit {
                    count += 1;
                }
            }
        }
        count
    }
}
