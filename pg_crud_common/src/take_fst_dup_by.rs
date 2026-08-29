pub(crate) fn take_fst_dup_by<T, FindDuplicateIdx>(
    values: &mut crate::duplicate_candidates::DuplicateCandidates<T>,
    find_duplicate_idx: FindDuplicateIdx,
) -> Option<T>
where
    FindDuplicateIdx: FnOnce(&[T]) -> Option<crate::duplicate_idx::DuplicateIdx>,
{
    let duplicate_idx = find_duplicate_idx(values.get_inner().as_slice())?;
    Some(values.get_inner_mut().swap_remove(duplicate_idx.get()))
}
