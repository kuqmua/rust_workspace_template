#[allow(
    clippy::single_call_fn,
    reason = "a named conversion boundary permits deterministic testing of oversized environment values without mutating process environment"
)]
pub(crate) fn parse_required_env_var_value<T, ParseError, Error, MapValueError, Parse, MapParseError>(
    std_env_var_ok_result: Result<crate::std_env_var_ok::StdEnvVarOk, crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError>,
    env_var_name_ref: crate::env_var_name_ref::EnvVarNameRef<'_>,
    map_value_error: MapValueError,
    parse: Parse,
    map_parse_error: MapParseError,
) -> Result<T, Error>
where
    MapValueError: FnOnce(crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError, crate::env_var_name::EnvVarName) -> Error,
    Parse: FnOnce(crate::std_env_var_ok::StdEnvVarOk) -> Result<T, ParseError>,
    MapParseError: FnOnce(ParseError) -> Error,
{
    let std_env_var_ok = std_env_var_ok_result.map_err(|error| {
        map_value_error(
            error,
            crate::env_var_name::EnvVarName::try_from(env_var_name_ref.as_ref().to_owned())
                .unwrap_or_else(crate::env_var_name::EnvVarName::from),
        )
    })?;
    parse(std_env_var_ok).map_err(map_parse_error)
}
