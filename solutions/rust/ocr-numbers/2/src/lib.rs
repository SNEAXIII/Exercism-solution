#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}
fn convert_one_char(input: &str) -> char {
    match input {
        " _ | ||_|" => '0',
        "     |  |" => '1',
        " _  _||_ " => '2',
        " _  _| _|" => '3',
        "   |_|  |" => '4',
        " _ |_  _|" => '5',
        " _ |_ |_|" => '6',
        " _   |  |" => '7',
        " _ |_||_|" => '8',
        " _ |_| _|" => '9',
        _ => '?',
    }
}
pub fn convert(input: &str) -> Result<String, Error> {
    let row_size = input
        .find("\n")
        .expect("We expect at least one \n for the exercise");
    let len_eol = input.bytes().filter(|b| *b == b'\n').count();
    if (len_eol + 1) % 4 != 0 {
        return Err(Error::InvalidRowCount(len_eol + 1));
    }
    if row_size % 3 != 0 {
        return Err(Error::InvalidColumnCount(row_size));
    }
    let mut result = String::new();
    let mut repr_number: String = String::with_capacity(9);
    for y in (0..len_eol).step_by(4) {
        if y != 0 {
            result.push(',');
        }
        for x in (0..row_size).step_by(3) {
            repr_number.clear();
            for row_offset in 0..3 {
                let cursor = x + y * (row_size + 1) + row_offset * (row_size + 1);
                repr_number.push_str(&input[cursor..cursor + 3]);
            }
            result.push(convert_one_char(&repr_number));
        }
    }
    Ok(result)
}
