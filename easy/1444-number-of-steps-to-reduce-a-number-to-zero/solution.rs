impl Solution {
       pub fn number_of_steps(num: i32) -> i32 {
        let (mut counter, mut temp) = (0, num);
        while temp != 0 {
            temp = match temp % 2 == 0 {
                true => temp / 2,
                false => temp - 1,
            };
            counter += 1;
        }
        counter
    }
}
