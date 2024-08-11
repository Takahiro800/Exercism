#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    use Comparison::*;

    let is_sublist = _first_list.is_empty()
        || _second_list
            .windows(_first_list.len())
            .any(|w| w == _first_list);
    let is_superlist = _second_list.is_empty()
        || _first_list
            .windows(_second_list.len())
            .any(|w| w == _second_list);

    match (is_sublist, is_superlist) {
        (true, true) => Equal,
        (true, false) => Sublist,
        (false, true) => Superlist,
        (false, false) => Unequal,
    }
}
