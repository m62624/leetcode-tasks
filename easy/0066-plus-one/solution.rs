impl Solution {
 pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut numbers = Vec::new();
        let mut skip = 1;
        digits.reverse();

        for n in &digits {
            if n + 1 > 9 {
                numbers.push(0);
                skip += 1;
            } else {
                numbers.push(n + 1);
                digits = digits.drain(skip..).collect();
                numbers.append(&mut digits);
                numbers.reverse();
                break;
            }
        }
        if numbers[0] == 0 {
            numbers.insert(0, 1);
        }

        numbers
    }
}
