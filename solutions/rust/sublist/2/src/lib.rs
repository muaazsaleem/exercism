#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}


// create_sublists takes a list and a length and returns a list of sublists for the list of length
fn create_sublists(list: &[i32], len: usize) -> Vec<&[i32]> {
    let index_len = list.len() - 1;
    let mut sublists = vec![];
    for i in 0..index_len {
        let end = i + len - 1;
        if  end <= index_len {
            sublists.push(&list[i..=end]);         
        } 
    }
    sublists
}

// compare_eq compares equal lists
fn compare_eq(first_list: &[i32], second_list: &[i32]) -> Comparison {
    for i in 0..second_list.len() {
        if first_list[i] != second_list[i] {
            return Comparison::Unequal
        } 
    }
    Comparison::Equal
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.is_empty() && second_list.is_empty() {
        return Comparison::Equal
    } else if !first_list.is_empty() && second_list.is_empty() {
        return Comparison::Superlist
    } else if first_list.is_empty() && !second_list.is_empty() {
        return Comparison::Sublist
    }

    if first_list.len() == second_list.len() {
        compare_eq(first_list, second_list)
    } else if first_list.len() > second_list.len() {
        let sublists = create_sublists(first_list, second_list.len());
        for sublist in sublists {
            match compare_eq(sublist, second_list) {
                Comparison::Equal => return Comparison::Superlist,
                _ => continue,
            }
        }
            Comparison::Unequal
    } else {
        let sublists = create_sublists(second_list, first_list.len());
        for sublist in sublists {
            match compare_eq(sublist, first_list) {
                Comparison::Equal => return Comparison::Sublist,
                _ => continue,
            }
        }
            Comparison::Unequal
    }
}
