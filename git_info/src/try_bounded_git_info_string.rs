pub(super) fn try_bounded_git_info_string(
    string: String,
) -> Result<
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN },
        false,
    >,
    crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
> {
    if string.len() > crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN {
        Err(crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong {
            len: string.len(),
            max: crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN,
        })
    } else {
        bounded_types::bounded_string::BoundedString::try_from(string).map_err(|source| match source {
            bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                actual_length,
                maximum_length,
            }
            | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                actual_length,
                minimum_length: maximum_length,
            } => crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong {
                len: actual_length.get(),
                max: maximum_length.get(),
            },
        })
    }
}
