fn calculate_guess(min: usize, max: usize, old_guess: usize) -> usize {
    let new_guess = min + ((max - min) / 2);
    if new_guess == old_guess {
        old_guess + 1
    } else {
        new_guess
    }
}
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    } else if array[0] == key {
        return Some(0);
    }
    let mut min = 0;
    let mut max = array.len() - 1;
    let mut guess = 0;
    for _ in 0..array.len() {
        guess = dbg!(calculate_guess(min, max, guess));
        if guess > max {
            return None;
        } else if array[guess] == key {
            return Some(guess);
        } else if array[guess] < key {
            min = guess;
        } else {
            max = guess;
        }
    }
    None
}