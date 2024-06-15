use std::collections::BinaryHeap;



impl Solution {
 pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut projects: Vec<(&i32, &i32)> = capital.iter().zip(profits.iter()).collect();

        projects.sort_unstable_by_key(|&(c, _)| c);

        let mut available_projects = BinaryHeap::new();
        let mut current_capital = w;
        let mut i = 0;

        for _ in 1..=k {
            while i < projects.len() && projects[i].0 <= &current_capital {
                available_projects.push(projects[i].1);
                i += 1;
            }

            match available_projects.pop() {
                Some(p) => current_capital += p,
                None => break,
            }
        }

        current_capital
    }
}
