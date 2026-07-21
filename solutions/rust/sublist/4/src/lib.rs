#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn contains<T: Eq>(suposed_big: &[T], suposed_fit: &[T]) -> bool {
    suposed_fit.is_empty()
        || suposed_big
            .windows(suposed_fit.len())
            .any(|list| suposed_fit == list)
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if contains(first_list, second_list) {
        Comparison::Superlist
    } else if contains(second_list, first_list) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}
