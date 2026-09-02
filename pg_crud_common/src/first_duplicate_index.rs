#[must_use]
pub fn first_duplicate_index<T>(values: &[T]) -> Option<crate::duplicate_index::DuplicateIndex>
where
    T: PartialEq,
{
    if values.len() < 2 {
        return None;
    }
    values
        .iter()
        .enumerate()
        .find(|(index, current)| values.iter().take(*index).any(|prev| prev == *current))
        .map(|(index, _)| crate::duplicate_index::DuplicateIndex::from(index))
}
