use crate::{EnvParseError, EnvVarError, EnvVarResultVarError, EnvVarValueRef, ParseEnvVarNameRef};

#[cfg(test)]
pub(super) fn parse_from_env_var_with<T>(
    env_v: EnvVarResultVarError,
    env_var_name: ParseEnvVarNameRef<'static>,
    parse: impl FnOnce(EnvVarValueRef<'_>) -> Result<T, EnvParseError>,
) -> Result<T, EnvParseError> {
    let raw_v = env_v.0.map_err(|source| EnvParseError::Read {
        name: super::EnvVarName::try_from(env_var_name.0.to_owned())
            .unwrap_or_else(super::EnvVarName::from),
        source: EnvVarError::from(source),
    })?;
    parse(EnvVarValueRef::from(raw_v.as_str()))
}
