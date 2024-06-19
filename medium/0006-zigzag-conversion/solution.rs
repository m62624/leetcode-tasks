impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let num_rows = num_rows as usize;
        let mut rows = vec![String::default(); num_rows];

        let mut current_row = 0;
        let mut going_down = false;

        s.chars().for_each(|c| {
            rows[current_row].push(c);
            if current_row == 0 || current_row == (num_rows - 1) {
                going_down = !going_down;
            }
            if going_down {
                current_row += 1;
            } else {
                current_row -= 1;
            }
        });
        rows.concat()
    }
}


