#[derive(Clone, Copy)]
enum Numbers {
    I(i32),
    V(i32),
    X(i32),
    L(i32),
    C(i32),
    D(i32),
    M(i32),
}

struct NumbersIter {
    data: Vec<Numbers>,
}

impl NumbersIter {
    fn new(data: Vec<Numbers>) -> Self {
        NumbersIter { data }
    }
}

impl Iterator for NumbersIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }

        let now = self.data.pop()?;
        let now_value = match now {
            Numbers::I(a) => a,
            Numbers::V(a) => a,
            Numbers::X(a) => a,
            Numbers::L(a) => a,
            Numbers::C(a) => a,
            Numbers::D(a) => a,
            Numbers::M(a) => a,
        };

        if let Some(&next) = self.data.last() {
            let next_value = match next {
                Numbers::I(a) => a,
                Numbers::V(a) => a,
                Numbers::X(a) => a,
                Numbers::L(a) => a,
                Numbers::C(a) => a,
                Numbers::D(a) => a,
                Numbers::M(a) => a,
            };

            if now_value < next_value {
                self.data.pop();
                return Some(next_value - now_value);
            }
        }

        Some(now_value)
    }
}

impl From<String> for NumbersIter {
    fn from(value: String) -> Self {
        let mut temp =
            NumbersIter::new(value.chars().into_iter().fold(Vec::new(), |mut acc, c| {
                match c {
                    'I' => acc.push(Numbers::I(1)),
                    'V' => acc.push(Numbers::V(5)),
                    'X' => acc.push(Numbers::X(10)),
                    'L' => acc.push(Numbers::L(50)),
                    'C' => acc.push(Numbers::C(100)),
                    'D' => acc.push(Numbers::D(500)),
                    'M' => acc.push(Numbers::M(1000)),
                    _ => (),
                }
                acc
            }));
        temp.data.reverse();
        temp
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        NumbersIter::from(s).sum()
    }
}
