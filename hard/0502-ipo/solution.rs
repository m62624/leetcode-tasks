use std::collections::BinaryHeap;
use std::cmp::Reverse;


impl Solution {
   pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut projects: Vec<(i32, i32)> = capital.into_iter().zip(profits.into_iter()).collect();
        projects.sort_by_key(|&(c, _)| c);

        let mut available_projects = BinaryHeap::new();
        let mut current_capital = w;
        let mut i = 0;

        for _ in 0..k {
            while i < projects.len() && projects[i].0 <= current_capital {
                available_projects.push(projects[i].1);
                i += 1;
            }

            if let Some(max_profit) = available_projects.pop() {
                current_capital += max_profit;
            } else {
                break;
            }
        }

        current_capital
    }
}
