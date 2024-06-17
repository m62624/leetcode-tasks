impl Solution {
      pub fn judge_square_sum(c: i32) -> bool {
        for a in 0..=(c as f32).sqrt() as i32 {
            let b = ((c - (a * a)) as f32).sqrt() as i32;
            if a * a + b * b == c {
                return true;
            }
        }
        false
    }
}
