#[must_use]
pub fn take_fst_dup<T>(
    values: &mut crate::duplicate_candidates::DuplicateCandidates<T>,
) -> Option<T>
where
    T: PartialEq,
{
    crate::take_fst_dup_by::take_fst_dup_by(
        values,
        crate::first_duplicate_index::first_duplicate_index,
    )
}
