#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list==second_list {
        return Comparison::Equal;
    }
    if second_list.len() == 0 || first_list.windows(second_list.len()).any(|list| second_list == list) {
        return Comparison::Superlist;
    }
    if first_list.len() == 0 || second_list.windows(first_list.len()).any(|list| first_list == list) {
        return Comparison::Sublist;
    }
    Comparison::Unequal
}