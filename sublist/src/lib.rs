#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/// Returns the Comparison value between first_list and second_list.
pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let lists = (first_list, second_list);

    match lists {
        (a, b) if a == b => Comparison::Equal,
        (a, b) if a.is_empty() || is_sublist(b, a) => Comparison::Sublist,
        (a, b) if b.is_empty() || is_sublist(a, b) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}

/// Returns true if first_list is a sublist of second_list.
fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    first_list
        .windows(second_list.len())
        .any(|l| l == second_list)
}
