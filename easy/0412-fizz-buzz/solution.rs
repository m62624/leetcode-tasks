impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n).into_iter().fold(Vec::new(), |mut acc, value| {
            if value % 3 == 0 && value % 5 == 0 {
                acc.push("FizzBuzz".into());
            } else if value % 3 == 0 {
                acc.push("Fizz".into());
            } else if value % 5 == 0 {
                acc.push("Buzz".into());
            } else {
                acc.push(value.to_string());
            }
            acc
        })
    }
}
