use super::StdEnvVarOkRef;

pub(super) fn parse_from_str_with_error<T, ParseError, Error>(
    v: StdEnvVarOkRef<'_>,
    mk_error: impl FnOnce(ParseError) -> Error,
) -> Result<T, Error>
where
    T: std::str::FromStr<Err = ParseError>,
{
    v.0.parse::<T>().map_err(mk_error)
}
