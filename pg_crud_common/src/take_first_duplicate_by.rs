pub(crate) fn take_first_duplicate_by<T, FindDuplicateIndex>(
    duplicate_candidates: &mut crate::duplicate_candidates::DuplicateCandidates<T>,
    find_duplicate_index: FindDuplicateIndex,
) -> Option<T>
where
    FindDuplicateIndex: FnOnce(&[T]) -> Option<crate::duplicate_index::DuplicateIndex>,
{
    let duplicate_index = find_duplicate_index(duplicate_candidates.get_inner().as_slice())?;
    Some(
        duplicate_candidates
            .get_inner_mut()
            .swap_remove(duplicate_index.get()),
    )
}
