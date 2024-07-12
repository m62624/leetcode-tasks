impl Solution {
    pub fn my_atoi(mut s: String) -> i32 {
        let mut number: i32 = 0;
        s = s.trim_start().into();

        if s.len() >= 1 {
            let min = i32::MIN.to_string();
            let max = i32::MAX.to_string();

            let mut str: String = s[1..].chars().take_while(|c| c.is_ascii_digit()).collect();

            if let Some(sign_or_number) = s[..1].chars().nth(0) {
                if sign_or_number.is_ascii_digit() {
                    str = format!("{sign_or_number}{str}");

                    if str.trim_start_matches('0').len() > max.len() {
                        number = i32::MAX;
                    } else {
                        if let Ok(v) = str.parse::<i64>() {
                            if v > i32::MAX as i64 {
                                number = i32::MAX;
                            } else {
                                number = v as i32;
                            }
                        }
                    }
                } else if sign_or_number == '+' && !str.is_empty() {
                    return Self::my_atoi(s[1..].into());
                } else if sign_or_number == '-' {
                    str = format!("{sign_or_number}{str}");

                    if str[1..].trim_start_matches('0').len() > min.len() {
                        number = i32::MIN;
                    } else {
                        if let Ok(v) = str.parse::<i64>() {
                            if v < i32::MIN as i64 {
                                number = i32::MIN;
                            } else {
                                number = v as i32;
                            }
                        }
                    }
                }
            }
        }

        number
    }
}
