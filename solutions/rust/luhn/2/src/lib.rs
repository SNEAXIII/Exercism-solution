pub fn is_valid(code: &str) -> bool {
    if code.trim().len() < 2 {
        return false;
    }
    let mut result = 0;
    let mut is_double = false;
    for char in code.chars().rev() {
        if char.is_ascii_whitespace() {
            continue;
        }
        let Some(int) = char.to_digit(10) else {
            return false;
        };
        if is_double {
            let mut to_add = int * 2;
            if to_add >= 10 {
                to_add -= 9
            }
            result += to_add;
        } else {
            result += int
        }
        is_double = !is_double;
    }
    result % 10 == 0
}