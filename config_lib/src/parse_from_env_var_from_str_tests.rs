#[cfg(test)]
pub(super) fn parse_from_env_var_from_str<T>(
    env_var_result_var_error: crate::env_var_result_var_error::EnvVarResultVarError,
    parse_env_var_name_ref: crate::parse_env_var_name_ref::ParseEnvVarNameRef<'static>,
    parse_context_ref: crate::parse_context_ref::ParseContextRef,
) -> Result<T, crate::env_parse_error::EnvParseError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    crate::parse_from_env_var_with_tests::parse_from_env_var_with(
        env_var_result_var_error,
        parse_env_var_name_ref,
        |v| {
            crate::parse_from_str_with_context_tests::parse_from_str_with_context(
                v,
                parse_context_ref,
            )
        },
    )
}
