#[must_use]
pub fn take_fst_dup_by_hash<T>(
    values: &mut crate::domain_types::DuplicateCandidates<T>,
) -> Option<T>
where
    T: Eq + std::hash::Hash,
{
    let duplicate_idx = crate::domain_types::first_duplicate_index_by_hash(values.0.as_slice())?;
    Some(values.0.swap_remove(duplicate_idx.get()))
}
