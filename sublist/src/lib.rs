#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    // Order matters to the list!!!
    
    if first_list == second_list {
        return Comparison::Equal;
    }

    if first_list == []  {
        return Comparison::Sublist;
    }

    if second_list == [] {
        return Comparison::Superlist;
    }

    if first_list.len() < second_list.len() {

        if second_list.windows(first_list.len()).any(|w| w == first_list) {
            return Comparison::Sublist;
        }
    }

    if first_list.len() > second_list.len() {

        if first_list.windows(second_list.len()).any(|w| w == second_list) {
            return Comparison::Superlist;
        }
    }

    Comparison::Unequal


}
