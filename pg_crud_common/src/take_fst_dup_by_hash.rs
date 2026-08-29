#[must_use]
pub fn take_fst_dup_by_hash<T>(
    values: &mut crate::duplicate_candidates::DuplicateCandidates<T>,
) -> Option<T>
where
    T: Eq + std::hash::Hash,
{
    crate::take_fst_dup_by::take_fst_dup_by(
        values,
        crate::first_duplicate_index_by_hash::first_duplicate_index_by_hash,
    )
}
