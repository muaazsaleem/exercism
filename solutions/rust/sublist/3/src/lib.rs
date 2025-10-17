use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}


pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match first_list.len().cmp(&second_list.len()) {
        Ordering::Equal => {
            if first_list == second_list {
                Comparison::Equal        
            } else {
                Comparison::Unequal            
            }
        },
        Ordering::Greater => {
            if second_list.is_empty() || first_list.windows(second_list.len()).any(|window| window == second_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal                
            }
        },
        Ordering::Less => {
            if first_list.is_empty() || second_list.windows(first_list.len()).any(|window| window == first_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal                
            }
        },        
    }
}
