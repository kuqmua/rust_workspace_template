#[must_use]
pub fn first_duplicate_index_by_hash<T>(values: &[T]) -> Option<crate::domain_types::DuplicateIdx>
where
    T: Eq + std::hash::Hash,
{
    if values.len() < 2 {
        return None;
    }
    let mut seen = std::collections::HashSet::with_capacity(values.len());
    values
        .iter()
        .enumerate()
        .find(|(_, current)| !seen.insert(*current))
        .map(|(idx, _)| crate::domain_types::DuplicateIdx::from(idx))
}
