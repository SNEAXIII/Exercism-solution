const DIGITS: [&str; 10] = [
    " _ | ||_|", // 0
    "     |  |", // 1
    " _  _||_ ", // 2
    " _  _| _|", // 3
    "   |_|  |", // 4
    " _ |_  _|", // 5
    " _ |_ |_|", // 6
    " _   |  |", // 7
    " _ |_||_|", // 8
    " _ |_| _|", // 9
];
#[derive(Debug, PartialEq, Eq)]

pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let eol_index: Vec<usize> = input
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '\n')
        .map(|(i, _)| i)
        .collect();
    if eol_index.is_empty() || (eol_index.len() + 1) % 4 != 0 {
        return Err(Error::InvalidRowCount(eol_index.len() + 1));
    }
    // We expect every lines have the same length
    let row_size = eol_index[0];
    if row_size % 3 != 0 {
        return Err(Error::InvalidColumnCount(eol_index[0]));
    }
    let mut result = String::new();
    for y_start in (0..eol_index.len()).step_by(4) {
        if y_start != 0 {
            result.push(',');
        }
        for x_start in (0..eol_index[0]).step_by(3) {
            let mut repr_number = String::new();
            for row_offset in 0..3 {
                let starting_index =
                    x_start + (y_start * (row_size + 1)) + row_offset * (row_size + 1);
                repr_number.push_str(&input[starting_index..starting_index + 3]);
            }
            let mut char_to_add = '?';
            for (index, digit) in DIGITS.iter().enumerate() {
                if repr_number == *digit {
                    char_to_add = char::from_digit(index as u32, 10)
                        .expect("index < 10 : DIGITS have 10 values")
                }
            }
            result.push(char_to_add);
        }
    }
    Ok(result)
}
