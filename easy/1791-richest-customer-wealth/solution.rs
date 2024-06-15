impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max_sum = 0;
        accounts.into_iter().for_each(|data| {
            let temp = data.iter().fold(0, |acc, value| acc + value);
            if temp > max_sum {
                max_sum = temp
            }
        });
        max_sum
    }
}
