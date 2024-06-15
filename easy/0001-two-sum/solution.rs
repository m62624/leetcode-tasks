impl Solution {
   pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(2);
        let mut hashmap =
            std::collections::HashMap::with_capacity(nums.len() - 1);

        for (index, f) in nums.into_iter().enumerate() {
            match hashmap.get(&(&target - f)) {
                Some(s) => {
                    result.push(index as i32);
                    result.push(*s);
                    break;
                }
                None => {
                    hashmap.insert(f, index as i32);
                }
            }
        }
        result
    }
}
