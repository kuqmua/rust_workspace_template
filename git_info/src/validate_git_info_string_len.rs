pub(super) const fn validate_git_info_string_len(
    len: usize,
) -> Result<(), crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError> {
    if len > crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN {
        Err(crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong {
            len,
            max: crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN,
        })
    } else {
        Ok(())
    }
}
