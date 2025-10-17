#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// Helper function that check if `needle` is a sublist of `haystack`
fn is_sublist(haystack: &[i32], needle: &[i32]) -> bool {
    needle.is_empty() || haystack.windows(needle.len()).any(|window| window == needle)
}


pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if is_sublist(first_list, second_list) {
        Comparison::Superlist
    } else if is_sublist(second_list, first_list) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}
