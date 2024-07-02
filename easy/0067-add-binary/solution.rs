use std::iter::repeat;

impl Solution {
 pub fn add_binary(a: String, b: String) -> String {
        let mut c = String::new();
        let mut flag = false;

        let a_len = a.len();
        let b_len = b.len();
        let m_len = a_len.max(b_len);

        (a.chars().rev())
            .chain(repeat('0').take(m_len - a_len))
            .zip((b.chars().rev()).chain(repeat('0').take(m_len - b_len)))
            .for_each(|(a, b)| match (a, b) {
                ('1', '1') => {
                    match flag {
                        true => c.push('1'),
                        false => c.push('0'),
                    }
                    flag = true;
                }
                ('1', '0') | ('0', '1') => match flag {
                    true => {
                        c.push('0');
                        flag = true;
                    }
                    false => {
                        c.push('1');
                        flag = false;
                    }
                },
                _ => {
                    match flag {
                        true => c.push('1'),
                        false => c.push('0'),
                    }
                    flag = false;
                }
            });

        if flag {
            c.push('1');
        }

        c.chars().rev().collect()
    }
}
