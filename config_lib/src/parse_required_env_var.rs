use crate::{EnvVarName, EnvVarNameRef, StdEnvVarOk};

pub fn parse_required_env_var<T, ParseError, Error, MapEnvVarError, Parse, MapParseError>(
    env_var_name: EnvVarNameRef<'_>,
    map_env_var_error: MapEnvVarError,
    parse: Parse,
    map_parse_error: MapParseError,
) -> Result<T, Error>
where
    MapEnvVarError: FnOnce(std::env::VarError, EnvVarName) -> Error,
    Parse: FnOnce(StdEnvVarOk) -> Result<T, ParseError>,
    MapParseError: FnOnce(ParseError) -> Error,
{
    let v = std::env::var(env_var_name.0).map_err(|std_env_var_error| {
        map_env_var_error(
            std_env_var_error,
            EnvVarName::try_from(env_var_name.0.to_owned()).unwrap_or_else(EnvVarName::from),
        )
    })?;
    parse(StdEnvVarOk::try_from(v).unwrap_or_else(StdEnvVarOk::from)).map_err(map_parse_error)
}
