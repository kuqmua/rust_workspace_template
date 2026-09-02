pub(super) fn parse_from_str_with_error<T, ParseError, Error>(
    std_env_var_ok_ref: crate::std_env_var_ok_ref::StdEnvVarOkRef<'_>,
    make_error: impl FnOnce(ParseError) -> Error,
) -> Result<T, Error>
where
    T: std::str::FromStr<Err = ParseError>,
{
    std_env_var_ok_ref.as_ref().parse::<T>().map_err(make_error)
}
