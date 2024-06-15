use std::collections::HashMap;

impl Solution {
     pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let split = |str: String| -> HashMap<char, u32> {
            str.chars().fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            })
        };
        let magazine = split(magazine);
        split(ransom_note).into_iter().all(|(c, count)| {
            magazine
                .get(&c)
                .map_or(false, |&magazine_count| magazine_count >= count)
        })
    }
}
