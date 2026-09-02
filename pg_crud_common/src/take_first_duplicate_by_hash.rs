#[must_use]
pub fn take_first_duplicate_by_hash<T>(
    duplicate_candidates: &mut crate::duplicate_candidates::DuplicateCandidates<T>,
) -> Option<T>
where
    T: Eq + std::hash::Hash,
{
    crate::take_first_duplicate_by::take_first_duplicate_by(
        duplicate_candidates,
        crate::first_duplicate_index_by_hash::first_duplicate_index_by_hash,
    )
}
