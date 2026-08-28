#[must_use]
pub fn take_fst_dup_by_hash<T>(
    values: &mut crate::domain_types::DuplicateCandidates<T>,
) -> Option<T>
where
    T: Eq + std::hash::Hash,
{
    crate::domain_types::take_fst_dup_by(values, crate::domain_types::first_duplicate_index_by_hash)
}
