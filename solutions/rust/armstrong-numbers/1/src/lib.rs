pub fn is_armstrong_number(num: u32) -> bool {
    let mut buffer = num;
    let mut compare = 0;
    let number_iter = if num == 0 { 0 } else { num.ilog10() }+1;
    for _ in 0..number_iter {
        compare += (buffer % 10).pow(number_iter);
        if compare > num {
            return false;
        }
        buffer /= 10;
    }
    compare == num
}