impl Solution {
   pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut result = 0;
        let mut add = 0;
        let mut max;
        let minutes = minutes as usize;

        let len = customers.len();

        for i in 0..len {
            if grumpy[i] == 0 {
                result += customers[i];
            } else if i < minutes {
                add += customers[i];
            }
        }

        max = add;

        for i in minutes..len {
            if grumpy[i] == 1 {
                add += customers[i];
            }
            if grumpy[i - minutes] == 1 {
                add -= customers[i - minutes];
            }
            max = max.max(add);
        }

        result + max
    }
}
