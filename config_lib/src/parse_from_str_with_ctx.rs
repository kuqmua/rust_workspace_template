use crate::{EnvParseError, EnvVarValueRef, ParseCtxRef};

#[cfg(test)]
pub(super) fn parse_from_str_with_ctx<T>(
    v: EnvVarValueRef<'_>,
    parse_ctx: ParseCtxRef,
) -> Result<T, EnvParseError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    T::from_str(v.0).map_err(|error| EnvParseError::Parse {
        context: parse_ctx,
        detail: to_err_string::domain_types::ErrorText::try_from(error.to_string())
            .unwrap_or_else(to_err_string::domain_types::ErrorText::from),
    })
}
