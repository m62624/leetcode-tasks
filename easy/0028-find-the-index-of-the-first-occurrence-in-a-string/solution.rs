impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let l = needle.len();

        if haystack.len() >= l {
            if haystack[..l] == needle[..] {
                return 0;
            }

            for c in l..haystack.len() + 1 {
                if needle[..] == haystack[c - l..c] {
                    return (c - l) as i32;
                }
            }
        }
        -1
    }
}
