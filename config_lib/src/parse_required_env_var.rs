pub fn parse_required_env_var<T, ParseError, Error, MapEnvVarError, Parse, MapParseError>(
    env_var_name: crate::env_var_name_ref::EnvVarNameRef<'_>,
    map_env_var_error: MapEnvVarError,
    parse: Parse,
    map_parse_error: MapParseError,
) -> Result<T, Error>
where
    MapEnvVarError: FnOnce(std::env::VarError, crate::env_var_name::EnvVarName) -> Error,
    Parse: FnOnce(crate::std_env_var_ok::StdEnvVarOk) -> Result<T, ParseError>,
    MapParseError: FnOnce(ParseError) -> Error,
{
    let v = std::env::var(env_var_name.0).map_err(|std_env_var_error| {
        map_env_var_error(
            std_env_var_error,
            crate::env_var_name::EnvVarName::try_from(env_var_name.0.to_owned())
                .unwrap_or_else(crate::env_var_name::EnvVarName::from),
        )
    })?;
    parse(
        crate::std_env_var_ok::StdEnvVarOk::try_from(v)
            .unwrap_or_else(crate::std_env_var_ok::StdEnvVarOk::from),
    )
    .map_err(map_parse_error)
}
