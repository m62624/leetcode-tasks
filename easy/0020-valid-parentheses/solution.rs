
impl Solution {
      pub fn brackets(l: char, r: char) -> bool {
        match l {
            '(' => r == ')',
            '[' => r == ']',
            '{' => r == '}',
            _ => false,
        }
    }

    pub fn new_brackets(s: String) -> bool {
        let mut s = s;
        loop {
            if s.len() % 2 != 0 {
                return false;
            }

            let mut flag = false;

            let chars = s.chars().collect::<Vec<char>>();
            let mut new_chars = chars.clone();

            for (index, br) in chars.windows(2).enumerate() {
                if Self::brackets(br[0], br[1]) {
                    flag = true;
                    new_chars[index] = ' ';
                    new_chars[index + 1] = ' ';
                }
            }
            s = new_chars
                .iter()
                .filter(|c| !c.is_ascii_whitespace())
                .collect();

            if s.len() == 0 {
                return true;
            } else if !flag {
                return false;
            }
        }
    }

    pub fn is_valid(s: String) -> bool {
        Self::new_brackets(s)
    }
}
