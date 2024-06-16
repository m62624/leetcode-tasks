impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }

        let mut x = x;
        let mut y = 0;
        let original = x;

        while x != 0 {
            let digit = x % 10;
            x /= 10;
            y = (y * 10) + digit;
        }
        original == y
    }
}
