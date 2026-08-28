pub(super) fn parse_pg_pool_non_zero_seconds(
    v: &crate::domain_types::StdEnvVarOk,
) -> Result<crate::domain_types::ConfigNonZeroU64, super::PgPoolConfigParseError> {
    let value =
        v.0.parse::<u64>()
            .map_err(|_error| super::PgPoolConfigParseError::Parse)?;
    std::num::NonZeroU64::new(value)
        .map(crate::domain_types::ConfigNonZeroU64::from)
        .ok_or(super::PgPoolConfigParseError::Zero)
}
