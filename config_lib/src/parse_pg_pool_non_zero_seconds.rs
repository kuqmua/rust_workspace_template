pub(super) fn parse_pg_pool_non_zero_seconds(
    std_env_var_ok: &crate::std_env_var_ok::StdEnvVarOk,
) -> Result<std::num::NonZeroU64, crate::pg_pool_config_parse_error::PgPoolConfigParseError> {
    let value = std_env_var_ok
        .parse::<u64>()
        .map_err(|_error| crate::pg_pool_config_parse_error::PgPoolConfigParseError::Parse)?;
    std::num::NonZeroU64::new(value)
        .ok_or(crate::pg_pool_config_parse_error::PgPoolConfigParseError::Zero)
}
