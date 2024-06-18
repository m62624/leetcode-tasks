impl Solution {
      pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut worker: Vec<&i32> = worker.iter().collect();
        worker.sort();

        let max_level = worker[worker.len() - 1];
        let mut sum = 0;

        let mut dp: Vec<(&i32, &i32)> = difficulty
            .iter()
            .zip(profit.iter())
            .filter(|v| v.0 <= max_level)
            .collect();

        dp.sort_by(|a, b| b.1.cmp(a.1));

        for lw in worker {
            for l in &dp {
                if lw >= l.0 {
                    sum += *l.1;
                    break;
                }
            }
        }
        sum
    }
}
