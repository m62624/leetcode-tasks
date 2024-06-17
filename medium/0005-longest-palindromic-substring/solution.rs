impl Solution {
   pub fn longest_palindrome(s: String) -> String {
        let mut start = 0;
        let mut max = 0;

        let bytes = s.as_bytes();
        let mut end = bytes.len() - 1;

        let check_palindrom = |bytes: &[u8], mut left: usize, mut right: usize| -> bool {
            while left < right {
                if bytes[left] != bytes[right] {
                    return false;
                }
                left += 1;
                right -= 1;
            }
            true
        };
        (0..bytes.len()).for_each(|st| {
            (st..bytes.len()).for_each(|mut i| {
                if check_palindrom(bytes, st, i) {
                    i += 1;
                    if i - st > max {
                        start = st;
                        max = i - start;
                        end = i;
                    }
                }
            });
        });
        s[start..end].to_string()
    }
}
