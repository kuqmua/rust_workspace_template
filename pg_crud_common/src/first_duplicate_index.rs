#[must_use]
pub fn first_duplicate_index<T>(values: &[T]) -> Option<crate::duplicate_idx::DuplicateIdx>
where
    T: PartialEq,
{
    if values.len() < 2 {
        return None;
    }
    values
        .iter()
        .enumerate()
        .find(|(idx, current)| values.iter().take(*idx).any(|prev| prev == *current))
        .map(|(idx, _)| crate::duplicate_idx::DuplicateIdx::from(idx))
}
