pub(super) fn parse_admin_token_text<T, Error>(
    v: super::super::StdEnvVarOk,
    map: impl FnOnce(String) -> Result<T, Error>,
) -> Result<T, super::TryFromStdEnvVarOkAdminTokenTextError> {
    if v.0.is_empty() {
        return Err(super::TryFromStdEnvVarOkAdminTokenTextError::Empty);
    }
    map(v.0).map_err(|_bounded_string_error| super::TryFromStdEnvVarOkAdminTokenTextError::TooLong)
}
