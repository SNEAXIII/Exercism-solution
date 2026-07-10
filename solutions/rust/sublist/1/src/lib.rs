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
    if first_list.len() == 0 {
        return Comparison::Sublist;
    }
    if second_list.len() == 0 {
        return Comparison::Superlist;
    }
    for list in first_list.windows(second_list.len()) {
        dbg!( first_list, list );
        if second_list == list {
            return Comparison::Superlist;
        }
    }
    for list in second_list.windows(first_list.len()) {
        dbg!( first_list, list );
        if first_list == list {
            return Comparison::Sublist;
        }
    }
    Comparison::Unequal
}