impl Solution {
       pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() != 0 {
            if s.len() == 1 {
                return 1;
            }
            let mut max = 0;
            let mut start = 0;
            let mut take = 1;
            let chars = s.chars().collect::<Vec<char>>();

            let mut substring: String;
            let mut next_substring: String;

            while take < chars.len() && start < chars.len() {
                substring = chars.iter().skip(start).take(take).collect();
                next_substring = chars.iter().skip(start + take).take(1).collect();

                if substring.contains(&next_substring) || substring.contains(&next_substring) {
                    start += 1;
                    take = 1;
                } else {
                    take += 1;
                }

                if take > max {
                    max = take;
                }
            }
            return max as i32;
        }
        0
    }
}
