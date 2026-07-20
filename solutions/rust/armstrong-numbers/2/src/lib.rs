pub fn is_armstrong_number(num: u32) -> bool {
    let digits: u32 = if num == 0 { 1 } else { num.ilog10() + 1 };
    let mut buffer: u32 = num;
    let mut compare: u32 = num;
    for _ in 0..digits {
        match compare.checked_sub((buffer % 10).pow(digits)) {
            Some(rest) => compare = rest,
            None => return false,
        }
        buffer /= 10;
    }
    compare == 0
}