pub(super) fn parse_admin_token_text<T, Error>(
    std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk,
    map: impl FnOnce(String) -> Result<T, Error>,
) -> Result<
    T,
    crate::try_from_std_env_var_ok_admin_token_text_error::TryFromStdEnvVarOkAdminTokenTextError,
> {
    if std_env_var_ok.is_empty() {
        return Err(crate::try_from_std_env_var_ok_admin_token_text_error::TryFromStdEnvVarOkAdminTokenTextError::Empty);
    }
    map(String::from(std_env_var_ok)).map_err(|_bounded_string_error| crate::try_from_std_env_var_ok_admin_token_text_error::TryFromStdEnvVarOkAdminTokenTextError::TooLong)
}
