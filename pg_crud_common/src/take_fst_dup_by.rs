pub(crate) fn take_fst_dup_by<T, FindDuplicateIdx>(
    values: &mut crate::domain_types::DuplicateCandidates<T>,
    find_duplicate_idx: FindDuplicateIdx,
) -> Option<T>
where
    FindDuplicateIdx: FnOnce(&[T]) -> Option<crate::domain_types::DuplicateIdx>,
{
    let duplicate_idx = find_duplicate_idx(values.get_inner().as_slice())?;
    Some(values.get_inner_mut().swap_remove(duplicate_idx.get()))
}
