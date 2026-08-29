#[cfg(test)]
pub(super) fn parse_from_env_var_with<T>(
    env_v: crate::env_var_result_var_error::EnvVarResultVarError,
    env_var_name: crate::parse_env_var_name_ref::ParseEnvVarNameRef<'static>,
    parse: impl FnOnce(
        crate::env_var_value_ref::EnvVarValueRef<'_>,
    ) -> Result<T, crate::env_parse_error::EnvParseError>,
) -> Result<T, crate::env_parse_error::EnvParseError> {
    let raw_v = env_v
        .0
        .map_err(|source| crate::env_parse_error::EnvParseError::Read {
            name: crate::env_var_name::EnvVarName::try_from(env_var_name.0.to_owned())
                .unwrap_or_else(crate::env_var_name::EnvVarName::from),
            source: crate::env_var_error::EnvVarError::from(source),
        })?;
    parse(crate::env_var_value_ref::EnvVarValueRef::from(
        raw_v.as_str(),
    ))
}
