pub(crate) fn try_new_unique_vec<T, FindDuplicate>(
    values: crate::duplicate_candidates::DuplicateCandidates<T>,
    find_duplicate: FindDuplicate,
) -> Result<Vec<T>, crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError<T>>
where
    FindDuplicate: FnOnce(&mut crate::duplicate_candidates::DuplicateCandidates<T>) -> Option<T>,
{
    let raw_values = Vec::from(values);
    if raw_values.is_empty() {
        return Err(
            crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::IsEmpty {
                location: proc_macro_location_bang::location!(),
            },
        );
    }
    if raw_values.len() > crate::not_empty_unique_vec_max_len::NOT_EMPTY_UNIQUE_VEC_MAX_LEN {
        return Err(
            crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::TooLong {
                location: proc_macro_location_bang::location!(),
            },
        );
    }
    let mut candidates = crate::duplicate_candidates::DuplicateCandidates::from(raw_values);
    if let Some(duplicate) = find_duplicate(&mut candidates) {
        return Err(
            crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::NotUnique {
                v: duplicate,
                location: proc_macro_location_bang::location!(),
            },
        );
    }
    Ok(Vec::from(candidates))
}
