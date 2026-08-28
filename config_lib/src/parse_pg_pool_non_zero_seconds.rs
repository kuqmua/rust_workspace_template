pub(super) fn parse_pg_pool_non_zero_seconds(
    v: &crate::domain_types::StdEnvVarOk,
) -> Result<std::num::NonZeroU64, super::PgPoolConfigParseError> {
    let value =
        v.0.parse::<u64>()
            .map_err(|_error| super::PgPoolConfigParseError::Parse)?;
    std::num::NonZeroU64::new(value).ok_or(super::PgPoolConfigParseError::Zero)
}
