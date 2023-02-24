#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list { return Comparison::Equal }
    else if _first_list.is_empty() { return Comparison::Sublist }
    else if _second_list.is_empty() { return Comparison::Superlist }
    else if _first_list.len() < _second_list.len() && 
        first_is_sublist(_first_list, _second_list) {
        return Comparison::Sublist
    }
    else if _first_list.len() > _second_list.len() && 
        first_is_sublist(_second_list, _first_list){
        return Comparison::Superlist
    }
    return Comparison::Unequal;
}

fn first_is_sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    for slice in _second_list.windows(_first_list.len()){
        if _first_list == slice { return true }
    }
    false
}
