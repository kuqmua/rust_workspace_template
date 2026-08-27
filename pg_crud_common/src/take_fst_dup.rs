#[must_use]
pub fn take_fst_dup<T>(values: &mut crate::domain_types::DuplicateCandidates<T>) -> Option<T>
where
    T: PartialEq,
{
    let duplicate_idx = crate::domain_types::first_duplicate_index(values.0.as_slice())?;
    Some(values.0.swap_remove(duplicate_idx.get()))
}
