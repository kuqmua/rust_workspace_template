#[must_use]
pub fn take_fst_dup<T>(values: &mut crate::domain_types::DuplicateCandidates<T>) -> Option<T>
where
    T: PartialEq,
{
    crate::domain_types::take_fst_dup_by(values, crate::domain_types::first_duplicate_index)
}
