impl Solution {
  pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefixes: Vec<String> = strs[0].chars().map(|c| c.to_string()).collect();
        let mut str: String = Default::default();

        prefixes.iter_mut().for_each(|cmb| {
            str.push_str(cmb);
            *cmb = str.clone();
        });
    
        str = Default::default();
        prefixes.reverse();

        for prefix in prefixes {
            if strs.iter().all(|s| s.starts_with(&prefix)) {
                str = prefix;
                break;
            }
        }

        str
    }
}
