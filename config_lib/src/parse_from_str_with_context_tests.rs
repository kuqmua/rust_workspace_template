#[cfg(test)]
pub(super) fn parse_from_str_with_context<T>(
    env_var_value_ref: crate::env_var_value_ref::EnvVarValueRef<'_>,
    parse_context_ref: crate::parse_context_ref::ParseContextRef,
) -> Result<T, crate::env_parse_error::EnvParseError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    T::from_str(env_var_value_ref.as_ref()).map_err(|error| {
        crate::env_parse_error::EnvParseError::Parse {
            context: parse_context_ref,
            detail: to_err_string::error_text::ErrorText::try_from(error.to_string())
                .unwrap_or_else(to_err_string::error_text::ErrorText::from),
        }
    })
}
