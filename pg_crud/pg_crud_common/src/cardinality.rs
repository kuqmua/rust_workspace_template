#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, optml::Optml, newtype::FromInner)]
pub struct DuplicateIdx(usize);
impl DuplicateIdx {
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}
#[must_use]
pub fn first_duplicate_idx<T>(values: &[T]) -> Option<DuplicateIdx>
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
        .map(|(idx, _)| DuplicateIdx::from(idx))
}
#[must_use]
pub fn first_duplicate_idx_by_hash<T>(values: &[T]) -> Option<DuplicateIdx>
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
        .map(|(idx, _)| DuplicateIdx::from(idx))
}
#[must_use]
pub fn take_fst_dup<T>(values: &mut Vec<T>) -> Option<T>
where
    T: PartialEq,
{
    let duplicate_idx = first_duplicate_idx(values.as_slice())?;
    Some(values.swap_remove(duplicate_idx.get()))
}
#[must_use]
pub fn take_fst_dup_by_hash<T>(values: &mut Vec<T>) -> Option<T>
where
    T: Eq + std::hash::Hash,
{
    let duplicate_idx = first_duplicate_idx_by_hash(values.as_slice())?;
    Some(values.swap_remove(duplicate_idx.get()))
}
