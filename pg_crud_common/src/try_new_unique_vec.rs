pub(crate) fn try_new_unique_vec<T, FindDuplicate>(
    values: crate::domain_types::DuplicateCandidates<T>,
    find_duplicate: FindDuplicate,
) -> Result<Vec<T>, crate::domain_types::NotEmptyUniqueVecTryNewError<T>>
where
    FindDuplicate: FnOnce(&mut crate::domain_types::DuplicateCandidates<T>) -> Option<T>,
{
    let raw_values = Vec::from(values);
    if raw_values.is_empty() {
        return Err(crate::domain_types::NotEmptyUniqueVecTryNewError::IsEmpty {
            location: location_macros::location!(),
        });
    }
    if raw_values.len() > crate::domain_types::NOT_EMPTY_UNIQUE_VEC_MAX_LEN {
        return Err(crate::domain_types::NotEmptyUniqueVecTryNewError::TooLong {
            location: location_macros::location!(),
        });
    }
    let mut candidates = crate::domain_types::DuplicateCandidates::from(raw_values);
    if let Some(duplicate) = find_duplicate(&mut candidates) {
        return Err(
            crate::domain_types::NotEmptyUniqueVecTryNewError::NotUnique {
                v: duplicate,
                location: location_macros::location!(),
            },
        );
    }
    Ok(Vec::from(candidates))
}
