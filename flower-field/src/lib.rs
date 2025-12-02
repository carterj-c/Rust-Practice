

pub fn annotate(garden: &[&str]) -> Vec<String> {
    // We cannot check len of an empty Vec
        if garden.is_empty() {
        return Vec::new();
    }

    // For each row, iterate over each byte we can do this by
    // garden[r].as_bytes() then bytes[c]
    let mut filled_garden: Vec<String> = Vec::new(); // I will fill this with the results
    let num_rows: usize = garden.len();
    let num_cols: usize = garden[0].as_bytes().len();

    for r in 0..num_rows {

        let start_row = r.saturating_sub(1);
        let end_row = (r + 1).min(num_rows - 1);
        let mut row_str = String::new();

        for c in 0..num_cols {

            if garden[r].as_bytes()[c] == b'*' {
                row_str.push('*');
                continue;
            }

            let mut num_flowers = 0;
            let start_col = c.saturating_sub(1);
            let end_col = (c + 1).min(num_cols - 1);

            for neighbor_r in start_row..=end_row {

                for neighbor_c in start_col..=end_col {

                    if garden[neighbor_r].as_bytes()[neighbor_c] == b'*' {
                        num_flowers += 1;
                    }
        
                }
            }

            if num_flowers == 0 {
                row_str.push(' ');
            }

            else {
                row_str.push_str(&num_flowers.to_string())
            }

        }
        filled_garden.insert(r, row_str);
    }

    filled_garden

}
