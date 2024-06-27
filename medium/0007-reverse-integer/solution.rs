impl Solution {
       pub fn reverse(mut x: i32) -> i32 {
        let mut r: i32 = 0;
        while x != 0 {
            match r.checked_mul(10) {
                Some(v) => r = v,
                None => return 0,
            }
            r += x % 10;
            x /= 10;
        }
        r
    }
}
