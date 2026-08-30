pub(super) fn parse_from_str_with_error<T, ParseError, Error>(
    v: crate::std_env_var_ok_ref::StdEnvVarOkRef<'_>,
    mk_error: impl FnOnce(ParseError) -> Error,
) -> Result<T, Error>
where
    T: std::str::FromStr<Err = ParseError>,
{
    v.as_ref().parse::<T>().map_err(mk_error)
}
