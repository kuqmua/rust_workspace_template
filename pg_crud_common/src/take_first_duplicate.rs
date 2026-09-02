#[must_use]
pub fn take_first_duplicate<T>(
    duplicate_candidates: &mut crate::duplicate_candidates::DuplicateCandidates<T>,
) -> Option<T>
where
    T: PartialEq,
{
    crate::take_first_duplicate_by::take_first_duplicate_by(
        duplicate_candidates,
        crate::first_duplicate_index::first_duplicate_index,
    )
}
