#[cfg(test)]
pub(super) fn parse_from_str_with_ctx<T>(
    v: crate::env_var_value_ref::EnvVarValueRef<'_>,
    parse_ctx: crate::parse_ctx_ref::ParseCtxRef,
) -> Result<T, crate::env_parse_error::EnvParseError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    T::from_str(v.as_ref()).map_err(|error| crate::env_parse_error::EnvParseError::Parse {
        context: parse_ctx,
        detail: to_err_string::error_text::ErrorText::try_from(error.to_string())
            .unwrap_or_else(to_err_string::error_text::ErrorText::from),
    })
}
